// Abuse guards for api/assistant.ts, which spends model tokens on every call.
// No dependencies, so it can be unit-tested. Files starting with `_` under
// api/ are not deployed as Vercel functions.
//
// The counters live in the function instance's memory: they stop a flood that
// hits one instance, not a distributed attack spread over many instances. The
// global limit belongs to the Vercel Firewall rate-limit rule on the docs
// project (docs/standards/user-docs-assistant.md); this is the layer that
// still works without it, and the one that answers with a readable 429.

export const LIMITS = {
  // One person asking questions: a few per minute, a working day's worth.
  perIpPerMinute: 8,
  perIpPerDay: 80,
  // Everyone on this instance together, to cap the burst from many IPs.
  instancePerMinute: 120,
};

const MINUTE = 60_000;
const DAY = 24 * 60 * MINUTE;
// Bound the memory a flood of distinct IPs can take.
const MAX_TRACKED_IPS = 10_000;

type Window = {start: number; count: number};

export type RateLimitResult = {ok: true} | {ok: false; retryAfterSeconds: number; scope: 'ip-minute' | 'ip-day' | 'instance'};

export class RateLimiter {
  private ipMinute = new Map<string, Window>();
  private ipDay = new Map<string, Window>();
  private instance: Window = {start: 0, count: 0};

  constructor(
    private limits = LIMITS,
    private now: () => number = Date.now,
  ) {}

  // Counts the request when it is allowed; a refused request costs nothing.
  check(ip: string): RateLimitResult {
    const t = this.now();
    if (this.ipMinute.size > MAX_TRACKED_IPS) this.prune(t);

    const instance = this.current(this.instance, t, MINUTE);
    const minute = this.current(this.ipMinute.get(ip), t, MINUTE);
    const day = this.current(this.ipDay.get(ip), t, DAY);

    if (instance.count >= this.limits.instancePerMinute) return this.refuse(instance, t, MINUTE, 'instance');
    if (day.count >= this.limits.perIpPerDay) return this.refuse(day, t, DAY, 'ip-day');
    if (minute.count >= this.limits.perIpPerMinute) return this.refuse(minute, t, MINUTE, 'ip-minute');

    instance.count++;
    minute.count++;
    day.count++;
    this.instance = instance;
    this.ipMinute.set(ip, minute);
    this.ipDay.set(ip, day);
    return {ok: true};
  }

  private current(w: Window | undefined, t: number, size: number): Window {
    return w && t - w.start < size ? w : {start: t, count: 0};
  }

  private refuse(w: Window, t: number, size: number, scope: 'ip-minute' | 'ip-day' | 'instance'): RateLimitResult {
    return {ok: false, retryAfterSeconds: Math.max(1, Math.ceil((w.start + size - t) / 1000)), scope};
  }

  private prune(t: number) {
    for (const [ip, w] of this.ipMinute) if (t - w.start >= MINUTE) this.ipMinute.delete(ip);
    for (const [ip, w] of this.ipDay) if (t - w.start >= DAY) this.ipDay.delete(ip);
    // Still too many live entries: drop the oldest rather than grow forever.
    if (this.ipMinute.size > MAX_TRACKED_IPS) this.ipMinute.clear();
    if (this.ipDay.size > MAX_TRACKED_IPS) this.ipDay.clear();
  }
}

// Client IP as Vercel reports it. Vercel overwrites `x-forwarded-for` with the
// real client address (a client-sent value doesn't survive), first entry wins.
export function clientIp(headers: Headers): string {
  const forwarded = headers.get('x-forwarded-for')?.split(',')[0]?.trim();
  return forwarded || headers.get('x-real-ip') || 'unknown';
}

// Only the docs site itself may call the assistant. Without this, any page on
// the web could make its visitors' browsers spend our tokens: a `text/plain`
// POST needs no CORS preflight and runs even though the reply is unreadable.
// Browsers always send `Origin` on a POST; a missing one is a script, not the
// site. (A script can forge it; that is what the rate limits are for.)
//
// Allowed: the listed origins (the public domain, which reaches the function
// through the CDN under another host) and the origin of the request's own host
// (previews, staging and the *.vercel.app domains, same-origin by definition).
export function allowedOrigin(headers: Headers, allowed: string[], requestUrl?: string): boolean {
  const origin = headers.get('origin');
  if (!origin) return false;
  if (allowed.includes(origin)) return true;
  let originHost: string;
  try {
    originHost = new URL(origin).host;
  } catch {
    return false;
  }
  const hosts = [headers.get('x-forwarded-host'), headers.get('host'), requestUrl ? safeHost(requestUrl) : null];
  return hosts.some((h) => !!h && h.split(',')[0].trim() === originHost);
}

const safeHost = (url: string): string | null => {
  try {
    return new URL(url).host;
  } catch {
    return null;
  }
};

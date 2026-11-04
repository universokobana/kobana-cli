// MCP server of the documentation itself (not of the product): lets an AI
// client list, search and read the pages of this site. Public and read-only,
// like the site. Served by the docs Vercel project as a function, routed from
// `/mcp` by vercel.json. Streamable HTTP, stateless, JSON responses only.
//
// The content is the build output of @signalwire/docusaurus-plugin-llms-txt:
// `/<locale>/llms.txt` indexes every page and `<route>.md` holds each page.

// ---- Per docs site: the only block that differs between apps. ----
// Canonical URL (what llms.txt links to and what tools return).
const SITE = 'https://cli.kobana.com.br';
const SERVER = {name: 'kobana-cli-docs', title: 'Kobana CLI — Docs', version: '1.0.0'};
// Product name as the tool descriptions read it (English, like the protocol).
const PRODUCT = 'Kobana CLI (the kobana command-line tool for the APIs of all Kobana products)';
// Locale the tools use when the client doesn't pass one (the site default).
const DEFAULT_LOCALE = 'pt';
// --------------------------------------------------------------------

// Origin the function fetches from; overridable to test against a local build.
const ORIGIN = process.env.DOCS_ORIGIN ?? SITE;
const LOCALES = ['en', 'pt', 'es'] as const;
type Locale = (typeof LOCALES)[number];
const PROTOCOL_VERSIONS = ['2025-06-18', '2025-03-26', '2024-11-05'];

// `body` is the page's normalized text, for search only.
type Page = {title: string; url: string; path: string; description: string; body?: string};

const localeArg = {
  type: 'string',
  enum: LOCALES,
  description: `Documentation language: en (English), pt (Portuguese), es (Spanish). Defaults to ${DEFAULT_LOCALE}.`,
};

const TOOLS = [
  {
    name: 'list_pages',
    title: 'List documentation pages',
    description: `List every page of the ${PRODUCT} documentation with its path and summary.`,
    inputSchema: {type: 'object', properties: {locale: localeArg}},
    annotations: {readOnlyHint: true, openWorldHint: false},
  },
  {
    name: 'search_docs',
    title: 'Search the documentation',
    description: `Search the ${PRODUCT} documentation by keywords. Returns the best matching pages with path and summary; read one with get_page.`,
    inputSchema: {
      type: 'object',
      properties: {
        query: {type: 'string', description: 'Keywords, e.g. "webhook signature" or "authentication".'},
        locale: localeArg,
        limit: {type: 'integer', minimum: 1, maximum: 25, description: 'Maximum results (default 8).'},
      },
      required: ['query'],
    },
    annotations: {readOnlyHint: true, openWorldHint: false},
  },
  {
    name: 'get_page',
    title: 'Read a documentation page',
    description: 'Return the full Markdown of one documentation page, by the path from list_pages/search_docs (e.g. "/en/api/overview/authentication") or its full URL.',
    inputSchema: {
      type: 'object',
      properties: {path: {type: 'string', description: 'Page path or URL on this site.'}},
      required: ['path'],
    },
    annotations: {readOnlyHint: true, openWorldHint: false},
  },
];

const indexCache = new Map<Locale, Promise<Page[]>>();

const fetchText = (path: string) =>
  fetch(`${ORIGIN}${path}`).then((r) => {
    if (!r.ok) throw new Error(`${path}: HTTP ${r.status}`);
    return r.text();
  });

// Warm invocations reuse the index; a new deploy starts a new instance.
function pagesOf(locale: Locale): Promise<Page[]> {
  let cached = indexCache.get(locale);
  if (!cached) {
    cached = Promise.all([
      fetchText(`/${locale}/llms.txt`),
      // Full text makes search find words that only appear in the body.
      // Optional: without it, search falls back to title/path/summary.
      fetchText(`/${locale}/llms-full.txt`).catch(() => ''),
    ]).then(([index, full]) => attachBodies(parseIndex(index), full));
    cached.catch(() => indexCache.delete(locale));
    indexCache.set(locale, cached);
  }
  return cached;
}

function parseIndex(text: string): Page[] {
  const pages: Page[] = [];
  const line = /^- \[([^\]]+)\]\((\S+?)\)(?::\s*(.*))?$/;
  for (const raw of text.split('\n')) {
    const m = raw.trim().match(line);
    if (!m || !m[2].startsWith(SITE)) continue;
    const path = m[2].slice(SITE.length).replace(/\.md$/, '').replace(/\/index$/, '/');
    if (/\/search$/.test(path)) continue;
    pages.push({title: m[1], url: `${SITE}${path}`, path, description: m[3] ?? ''});
  }
  return pages;
}

// llms-full.txt lists the same pages in the index order, each as `---` then
// `# <title>` and its Markdown, with no URL. Pair them by title, in order.
function attachBodies(pages: Page[], full: string): Page[] {
  const marker = full.indexOf('\n# Full Documentation Content');
  if (marker < 0) return pages;
  const sections = full.slice(marker).split(/\n---\n/).slice(1);
  let next = 0;
  for (const section of sections) {
    const title = section.match(/^\s*# (.+)$/m)?.[1]?.trim();
    if (!title) continue;
    const at = pages.findIndex((p, i) => i >= next && p.title === title);
    if (at < 0) continue;
    pages[at].body = normalize(section);
    next = at + 1;
  }
  return pages;
}

const normalize = (s: string) =>
  s
    .toLowerCase()
    .normalize('NFD')
    .replace(/[̀-ͯ]/g, '');

// Crude stemming so "instalar" also finds "instalação"/"instale": long words
// match by their first five letters.
const stem = (t: string) => (t.length >= 6 ? t.slice(0, 5) : t);

const count = (haystack: string, needle: string) => {
  let n = 0;
  for (let i = haystack.indexOf(needle); i >= 0 && n < 10; i = haystack.indexOf(needle, i + needle.length)) n++;
  return n;
};

function search(pages: Page[], query: string, limit: number): Page[] {
  const terms = [...new Set(normalize(query).split(/[^a-z0-9]+/).filter((t) => t.length > 2))].map(stem);
  if (terms.length === 0) return [];
  return pages
    .map((p) => {
      const title = normalize(p.title);
      const path = normalize(p.path);
      const desc = normalize(p.description);
      const body = p.body ?? '';
      let score = 0;
      let matched = 0;
      for (const t of terms) {
        const s =
          (title.includes(t) ? 6 : 0) +
          (path.includes(t) ? 3 : 0) +
          (desc.includes(t) ? 2 : 0) +
          Math.min(count(body, t), 10) * 0.4;
        if (s > 0) matched++;
        score += s;
      }
      // Pages that cover every term beat pages that repeat one term a lot.
      if (matched === terms.length && terms.length > 1) score *= 1.5;
      return {p, score};
    })
    .filter((x) => x.score > 0)
    .sort((a, b) => b.score - a.score)
    .slice(0, limit)
    .map((x) => x.p);
}

const asLocale = (v: unknown): Locale => (LOCALES.includes(v as Locale) ? (v as Locale) : DEFAULT_LOCALE);
const text = (t: string, isError = false) => ({content: [{type: 'text', text: t}], ...(isError ? {isError} : {})});
const listing = (pages: Page[]) =>
  pages.map((p) => `- ${p.title} — ${p.path}${p.description ? `\n  ${p.description}` : ''}`).join('\n');

async function callTool(name: string, args: Record<string, unknown>) {
  if (name === 'list_pages') {
    return text(listing(await pagesOf(asLocale(args.locale))));
  }
  if (name === 'search_docs') {
    const query = typeof args.query === 'string' ? args.query : '';
    const limit = Math.min(Math.max(Number(args.limit) || 8, 1), 25);
    const hits = search(await pagesOf(asLocale(args.locale)), query, limit);
    if (!hits.length) return text(`No page matches "${query}". Try other keywords or list_pages.`);
    const ranked = hits
      .map((p, i) => `${i + 1}. ${p.title} — ${p.path}${p.description ? `\n   ${p.description}` : ''}`)
      .join('\n');
    return text(`Results ranked by relevance; #1 is the best match. Read it with get_page(path).\n\n${ranked}`);
  }
  if (name === 'get_page') {
    let path = typeof args.path === 'string' ? args.path.trim() : '';
    if (path.startsWith(SITE)) path = path.slice(SITE.length);
    // Only pages of this site: a locale prefix and plain path segments.
    if (!/^\/(en|pt|es)(\/[\w.-]+)*\/?$/.test(path) || path.includes('..')) {
      return text(`Invalid path "${path}". Use a path from list_pages, e.g. /en/api/overview/introduction.`, true);
    }
    // `/pt/` and `/pt` are the locale home (`index.md`); any other page is
    // `<route>.md`, with or without a trailing slash.
    path = path.replace(/\.md$/, '').replace(/\/+$/, '');
    const md = /^\/(en|pt|es)$/.test(path) ? `${path}/index.md` : `${path}.md`;
    const r = await fetch(`${ORIGIN}${md}`);
    if (!r.ok) return text(`Page not found: ${path}. Use list_pages or search_docs to find it.`, true);
    return text(await r.text());
  }
  return null;
}

type RpcRequest = {jsonrpc: '2.0'; id?: string | number | null; method: string; params?: any};

async function handle(msg: RpcRequest) {
  const ok = (result: unknown) => ({jsonrpc: '2.0', id: msg.id, result});
  const fail = (code: number, message: string) => ({jsonrpc: '2.0', id: msg.id ?? null, error: {code, message}});

  switch (msg.method) {
    case 'initialize': {
      const requested = msg.params?.protocolVersion;
      return ok({
        protocolVersion: PROTOCOL_VERSIONS.includes(requested) ? requested : PROTOCOL_VERSIONS[0],
        capabilities: {tools: {listChanged: false}},
        serverInfo: SERVER,
        instructions:
          `Documentation of ${PRODUCT} in en, pt and es. Use search_docs to find pages and get_page to read them.`,
      });
    }
    case 'ping':
      return ok({});
    case 'tools/list':
      return ok({tools: TOOLS});
    case 'tools/call': {
      const result = await callTool(msg.params?.name, msg.params?.arguments ?? {});
      return result ? ok(result) : fail(-32602, `Unknown tool: ${msg.params?.name}`);
    }
    default:
      return fail(-32601, `Method not found: ${msg.method}`);
  }
}

const CORS = {
  'Access-Control-Allow-Origin': '*',
  'Access-Control-Allow-Methods': 'POST, GET, OPTIONS',
  'Access-Control-Allow-Headers': 'Content-Type, Accept, Mcp-Protocol-Version, Mcp-Session-Id',
};
const json = (body: unknown, status = 200) =>
  new Response(JSON.stringify(body), {status, headers: {'Content-Type': 'application/json', ...CORS}});

export async function POST(request: Request): Promise<Response> {
  let body: unknown;
  try {
    body = await request.json();
  } catch {
    return json({jsonrpc: '2.0', id: null, error: {code: -32700, message: 'Parse error'}}, 400);
  }
  const batch = Array.isArray(body) ? body : [body];
  const replies = [];
  for (const msg of batch as RpcRequest[]) {
    if (!msg || msg.jsonrpc !== '2.0' || typeof msg.method !== 'string') {
      replies.push({jsonrpc: '2.0', id: null, error: {code: -32600, message: 'Invalid Request'}});
      continue;
    }
    // Notifications (no id) get no reply.
    if (msg.id === undefined) continue;
    try {
      replies.push(await handle(msg));
    } catch (err) {
      replies.push({jsonrpc: '2.0', id: msg.id, error: {code: -32603, message: (err as Error).message}});
    }
  }
  if (replies.length === 0) return new Response(null, {status: 202, headers: CORS});
  return json(Array.isArray(body) ? replies : replies[0]);
}

// Stateless server: no SSE stream to open, no session to end.
export function GET(): Response {
  return new Response('Method Not Allowed', {status: 405, headers: {Allow: 'POST', ...CORS}});
}

export function OPTIONS(): Response {
  return new Response(null, {status: 204, headers: CORS});
}

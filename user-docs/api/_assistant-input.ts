// Request validation for api/assistant.ts, with no dependencies so it can be
// unit-tested without the AI SDK. Files starting with `_` under api/ are not
// deployed as Vercel functions.

// The endpoint is public and every call costs tokens: keep requests small.
export const MAX_MESSAGES = 20;
export const MAX_USER_CHARS = 2000;

export type Language = {name: string; locale: 'pt' | 'en' | 'es'};

export const LANGUAGES: Record<string, Language> = {
  pt: {name: 'Brazilian Portuguese', locale: 'pt'},
  en: {name: 'English', locale: 'en'},
  es: {name: 'Spanish', locale: 'es'},
};

// The subset of an AI SDK UIMessage this file reads.
type InputMessage = {role?: unknown; parts?: Array<{type?: unknown; text?: unknown}>};

export type AssistantInput<M> = {ok: true; messages: M[]; lang: Language} | {ok: false; error: string};

const textOf = (m: InputMessage) =>
  (Array.isArray(m.parts) ? m.parts : []).map((p) => (p?.type === 'text' && typeof p.text === 'string' ? p.text : '')).join('');

// `defaultLocale` is the site's source language, used when the page sent none.
export function parseAssistantInput<M extends InputMessage>(body: unknown, defaultLocale: keyof typeof LANGUAGES): AssistantInput<M> {
  const {messages, locale} = (body ?? {}) as {messages?: unknown; locale?: unknown};
  if (!Array.isArray(messages) || messages.length === 0) return {ok: false, error: 'messages required'};
  if (messages.length > MAX_MESSAGES) return {ok: false, error: 'conversation too long, start a new one'};
  for (const m of messages as InputMessage[]) {
    if (m?.role !== 'user' && m?.role !== 'assistant') return {ok: false, error: 'invalid role'};
    if (m.role === 'user' && textOf(m).length > MAX_USER_CHARS) return {ok: false, error: 'message too long'};
  }
  const lang = (typeof locale === 'string' && LANGUAGES[locale]) || LANGUAGES[defaultLocale];
  return {ok: true, messages: messages as M[], lang};
}

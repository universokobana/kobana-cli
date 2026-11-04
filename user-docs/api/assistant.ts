// Docs assistant: answers questions about this documentation with an LLM that
// reads the site through the docs MCP server (api/mcp.ts, served at /mcp).
// Streams AI SDK UI messages to the chat panel (src/components/Assistant).
//
// The model goes through the Vercel AI Gateway: on Vercel it authenticates by
// the project's OIDC token; locally it needs AI_GATEWAY_API_KEY.

import {createMCPClient} from '@ai-sdk/mcp';
import {convertToModelMessages, isStepCount, streamText, type UIMessage} from 'ai';
import {parseAssistantInput, type Language} from './_assistant-input.js';
import {allowedOrigin, clientIp, RateLimiter} from './_rate-limit.js';

// ---- Per docs site: the only block that differs between apps. ----
const SITE = 'https://cli.kobana.com.br';
const PRODUCT = 'Kobana CLI (a ferramenta de linha de comando kobana, para as APIs dos produtos Kobana)';
// Language of the answer when the page sends none (the site's source locale).
const DEFAULT_LOCALE = 'pt';
// --------------------------------------------------------------------

// Latest Gemini Flash on the AI Gateway ($0.75 in / $3.00+ out per 1M
// tokens, 2026-11). The cheaper 2.5-flash-lite and 2.5-flash failed the same
// questions (skipped the tools, read the wrong page, broke links); 3-flash
// passed. Overridable to compare models locally.
const MODEL = process.env.ASSISTANT_MODEL ?? 'google/gemini-3.8-flash';
// The function reaches the MCP over HTTP; overridable to test locally.
const ORIGIN = process.env.DOCS_ORIGIN ?? SITE;

// The endpoint is public and every call costs tokens (request limits live
// in _assistant-input.ts).
const MAX_OUTPUT_TOKENS = 1500;
// Tool steps before the model must write; one more step for the answer.
const RESEARCH_STEPS = 5;
const MAX_STEPS = RESEARCH_STEPS + 1;


const system = (lang: Language) => `You are the documentation assistant of ${PRODUCT}, on ${SITE}.
Answer only from this documentation, which you read with the tools: search_docs to find pages, get_page to read them, list_pages to browse.
- Always search before answering. Build the query from the user's own key words and keep their qualifiers (for "MCP da documentação", search "MCP documentação", not just "MCP").
- Then read with get_page the result whose title best matches the question (usually the first), and a second one if needed. Never answer from the summaries alone.
- Do at most two searches and read at most three pages, then answer. If they don't answer the question, say the documentation doesn't seem to cover it and link the closest pages.
- Answer in ${lang.name}, concise and practical, in Markdown. Put commands, JSON and code in fenced code blocks with the language.
- Cite the pages you used as Markdown links to their full URL on ${SITE} (without the .md suffix).
- If the documentation does not cover the question, say so plainly and do not invent endpoints, fields, prices or behavior.
- You cannot access the user's account or data; for that, point to the product APIs and their documentation sites.
- Decline questions unrelated to ${PRODUCT} or Kobana in one sentence.`;

// Cap on the research handed to the final step (~15k tokens).
const MAX_RESEARCH_CHARS = 60_000;

// MCP tool output is `{content: [{type: 'text', text}]}`.
const outputText = (output: unknown): string => {
  const content = (output as {content?: Array<{type: string; text?: string}>})?.content;
  return Array.isArray(content) ? content.map((c) => c.text ?? '').join('\n') : JSON.stringify(output);
};

// One JSON line per failure, never the message text of the user.
const logError = (message: string, err: unknown) =>
  console.error(
    JSON.stringify({
      severity: 'error',
      message,
      caused_by: err instanceof Error ? err.constructor.name : 'UnknownError',
      error: err instanceof Error ? err.message : String(err),
    }),
  );

const jsonError = (status: number, error: string, headers: Record<string, string> = {}) =>
  new Response(JSON.stringify({error}), {status, headers: {'Content-Type': 'application/json', ...headers}});
const badRequest = (message: string) => jsonError(400, message);

// Per instance; see _rate-limit.ts for what it does and does not stop.
const limiter = new RateLimiter();
// The site itself, plus the local origin when testing against a local build.
const ALLOWED_ORIGINS = [...new Set([SITE, ORIGIN])];

export async function POST(request: Request): Promise<Response> {
  // Cheap checks first: nothing below runs (or costs) for a refused request.
  if (!allowedOrigin(request.headers, ALLOWED_ORIGINS, request.url)) return jsonError(403, 'forbidden');
  const ip = clientIp(request.headers);
  const limit = limiter.check(ip);
  if (limit.ok === false) {
    console.warn(JSON.stringify({severity: 'warn', message: 'assistant rate limited', scope: limit.scope}));
    return jsonError(429, 'rate_limited', {'Retry-After': String(limit.retryAfterSeconds)});
  }

  let body: unknown;
  try {
    body = await request.json();
  } catch {
    return badRequest('invalid JSON');
  }
  const input = parseAssistantInput<UIMessage>(body, DEFAULT_LOCALE);
  if (input.ok === false) return badRequest(input.error);
  const {messages, lang} = input;

  const mcp = await createMCPClient({
    transport: {type: 'http', url: `${ORIGIN}/mcp`},
    // The docs MCP is a plain stateless server: go straight to `initialize`.
    protocolVersionDiscovery: false,
  });
  const close = () => mcp.close().catch(() => {});

  try {
    // The model often omits `locale`; the MCP would then fall back to its own
    // default. Fill in the reader's language unless the model chose one.
    const tools = Object.fromEntries(
      Object.entries(await mcp.tools()).map(([name, t]) => [
        name,
        {
          ...t,
          execute: (input: Record<string, unknown>, options: any) =>
            t.execute!(name === 'get_page' || input.locale ? input : {...input, locale: lang.locale}, options),
        },
      ]),
    ) as Awaited<ReturnType<typeof mcp.tools>>;
    const history = await convertToModelMessages(messages, {ignoreIncompleteToolCalls: true});
    const result = streamText({
      model: MODEL,
      system: system(lang),
      messages: history,
      tools,
      stopWhen: isStepCount(MAX_STEPS),
      // Flash-lite happily answers from nothing ("the docs don't cover it").
      // Ground it: search first, then read (or search again) before writing.
      prepareStep: ({stepNumber, steps}) => {
        if (stepNumber === 0) return {toolChoice: {type: 'tool', toolName: 'search_docs'}};
        if (stepNumber === 1) return {toolChoice: 'required', activeTools: ['get_page', 'search_docs']};
        // Research budget spent: answer with what was read instead of ending
        // silent. Gemini 3 still calls a tool under `toolChoice: 'none'` (the
        // stream aborts with ToolChoiceViolationError) and even with no tools
        // offered, so the step also gets an explicit instruction.
        // Gemini 3 keeps calling tools while its history is full of tool
        // calls, so the last step sees the research as plain text instead.
        if (stepNumber >= RESEARCH_STEPS) {
          const research = steps
            .flatMap((step) => step.toolResults)
            .map((r) => `### ${r.toolName} ${JSON.stringify(r.input)}\n${outputText(r.output)}`)
            .join('\n\n')
            .slice(0, MAX_RESEARCH_CHARS);
          return {
            activeTools: [],
            system: system(lang),
            messages: [
              ...history,
              {
                role: 'user',
                content: `Documentation you gathered for my last question:\n\n${research}\n\nAnswer my last question now, only from this material. Do not call any tool.`,
              },
            ],
          };
        }
        return {};
      },
      maxOutputTokens: MAX_OUTPUT_TOKENS,
      temperature: 0.2,
      onFinish: close,
      onAbort: close,
      onError: ({error}) => {
        logError('assistant stream failed', error);
        return close();
      },
    });
    return result.toUIMessageStreamResponse({
      // Never leak provider or gateway internals to the page.
      onError: () => 'The assistant is unavailable right now. Please try again.',
    });
  } catch (err) {
    logError('assistant setup failed', err);
    await close();
    throw err;
  }
}

#!/usr/bin/env node
/**
 * Generates the command manual (docs/manual) in pt-BR, en and es from the
 * `kobana` binary itself — the same source the CLI builds its commands from.
 *
 *   - the command tree, the usage lines and the one-line summaries come from
 *     `kobana <product> ... --help`, in each language (`--lang`);
 *   - the HTTP method, path and parameters come from `kobana schema`;
 *   - the request body fields come from the OpenAPI spec embedded in the
 *     binary (crates/kobana-cli/specs), with its text translated through
 *     specs/i18n/<spec>.{en,es}.json, as the CLI does.
 *
 * Nothing is written by hand: when the specs change, rebuild the binary and
 * run `npm run gen-manual`. `--check` regenerates in memory and fails when the
 * committed files differ (CI).
 *
 *   KOBANA_BIN=/path/to/kobana npm run gen-manual     # default: ../target/release/kobana
 */
import {execFile} from 'node:child_process';
import {existsSync, mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync} from 'node:fs';
import {dirname, join, relative, resolve} from 'node:path';
import {fileURLToPath} from 'node:url';

const ROOT = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const REPO = resolve(ROOT, '..');
const BIN = process.env.KOBANA_BIN ?? join(REPO, 'target/release/kobana');
const SPECS = join(REPO, 'crates/kobana-cli/specs');
const CHECK = process.argv.includes('--check');
const CONCURRENCY = 8;

const LOCALES = [
  {code: 'pt', locale: 'pt-BR', dir: 'docs'},
  {code: 'en', locale: 'en', dir: 'i18n/en/docusaurus-plugin-content-docs/current'},
  {code: 'es', locale: 'es', dir: 'i18n/es/docusaurus-plugin-content-docs/current'},
];

// Words that are not in the binary's help: the manual's own labels.
const T = {
  pt: {
    commands: 'Comandos', usage: 'Uso', options: 'Opções', arguments: 'Argumentos',
    command: 'Comando', http: 'HTTP', description: 'Descrição', name: 'Nome', type: 'Tipo',
    required: 'Obrigatório', yes: 'sim', no: 'não', resources: 'Recursos', resource: 'Recurso',
    pathArgs: 'Argumentos do caminho', query: 'Parâmetros de consulta', headers: 'Cabeçalhos',
    body: 'Corpo da requisição', responses: 'Respostas', example: 'Exemplo',
    bodyMore: 'Outros campos e objetos aninhados: `kobana schema {key}`.',
    bodyOther: 'O corpo não é um objeto simples (ou tem variantes). Veja `kobana schema {key}`.',
    multipart: 'Enviado como formulário `multipart/form-data`: os arquivos vão em `--file CAMPO=CAMINHO`.',
    values: 'Valores', defaultRun: 'Roda pelo próprio nome', helpers: 'Helpers',
    helpersIntro: 'Atalhos `+verbo` que montam a requisição a partir de flags nomeadas. Veja [Helpers](/usage/helpers).',
    specific: 'Opções do helper', prodIntro: 'Recursos do produto `{p}`. A credencial é a `{env}` — veja [Autenticação](/getting-started/authentication) e [Produtos](/reference/products).',
    resourceIntro: 'Comandos de `kobana {p} {r}`. Toda opção global ([Opções globais](/manual/commands/global-options)) vale aqui.',
    manualTitle: 'Manual', globalTitle: 'Opções globais', globalIntro: 'Valem para todos os comandos, antes ou depois do nome do recurso.',
    nested: 'Subrecursos', helpHeading: 'Saída de `--help`', site: 'Conheça o produto',
  },
  en: {
    commands: 'Commands', usage: 'Usage', options: 'Options', arguments: 'Arguments',
    command: 'Command', http: 'HTTP', description: 'Description', name: 'Name', type: 'Type',
    required: 'Required', yes: 'yes', no: 'no', resources: 'Resources', resource: 'Resource',
    pathArgs: 'Path arguments', query: 'Query parameters', headers: 'Headers',
    body: 'Request body', responses: 'Responses', example: 'Example',
    bodyMore: 'More fields and nested objects: `kobana schema {key}`.',
    bodyOther: 'The body is not a plain object (or has variants). See `kobana schema {key}`.',
    multipart: 'Sent as a `multipart/form-data` form: files go in `--file FIELD=PATH`.',
    values: 'Values', defaultRun: 'Runs by its own name', helpers: 'Helpers',
    helpersIntro: '`+verb` shortcuts that build the request from named flags. See [Helpers](/usage/helpers).',
    specific: 'Helper options', prodIntro: 'Resources of the `{p}` product. Its credential is `{env}` — see [Authentication](/getting-started/authentication) and [Products](/reference/products).',
    resourceIntro: 'Commands of `kobana {p} {r}`. Every global option ([Global options](/manual/commands/global-options)) applies here.',
    manualTitle: 'Manual', globalTitle: 'Global options', globalIntro: 'They apply to every command, before or after the resource name.',
    nested: 'Subresources', helpHeading: '`--help` output', site: 'Learn about the product',
  },
  es: {
    commands: 'Comandos', usage: 'Uso', options: 'Opciones', arguments: 'Argumentos',
    command: 'Comando', http: 'HTTP', description: 'Descripción', name: 'Nombre', type: 'Tipo',
    required: 'Obligatorio', yes: 'sí', no: 'no', resources: 'Recursos', resource: 'Recurso',
    pathArgs: 'Argumentos de la ruta', query: 'Parámetros de consulta', headers: 'Encabezados',
    body: 'Cuerpo de la solicitud', responses: 'Respuestas', example: 'Ejemplo',
    bodyMore: 'Otros campos y objetos anidados: `kobana schema {key}`.',
    bodyOther: 'El cuerpo no es un objeto simple (o tiene variantes). Mira `kobana schema {key}`.',
    multipart: 'Se envía como formulario `multipart/form-data`: los archivos van en `--file CAMPO=RUTA`.',
    values: 'Valores', defaultRun: 'Se ejecuta por su propio nombre', helpers: 'Helpers',
    helpersIntro: 'Atajos `+verbo` que arman la solicitud a partir de flags con nombre. Mira [Helpers](/usage/helpers).',
    specific: 'Opciones del helper', prodIntro: 'Recursos del producto `{p}`. Su credencial es `{env}` — mira [Autenticación](/getting-started/authentication) y [Productos](/reference/products).',
    resourceIntro: 'Comandos de `kobana {p} {r}`. Toda opción global ([Opciones globales](/manual/commands/global-options)) vale aquí.',
    manualTitle: 'Manual', globalTitle: 'Opciones globales', globalIntro: 'Valen para todos los comandos, antes o después del nombre del recurso.',
    nested: 'Subrecursos', helpHeading: 'Salida de `--help`', site: 'Conoce el producto',
  },
};

// ---------------------------------------------------------------- process

let running = 0;
const waiting = [];
const slot = () =>
  running < CONCURRENCY
    ? (running++, Promise.resolve())
    : new Promise((ok) => waiting.push(ok)).then(() => running++);
const release = () => {
  running--;
  waiting.shift()?.();
};

function run(args, lang) {
  return slot().then(
    () =>
      new Promise((ok, fail) => {
        execFile(
          BIN,
          [...args, '--lang', lang],
          {
            maxBuffer: 64 * 1024 * 1024,
            env: {...process.env, KOBANA_LANG: lang, NO_COLOR: '1', KOBANA_NO_UPDATE_NOTIFIER: '1', KOBANA_CONFIG_DIR: join(ROOT, '.docusaurus/kobana-config')},
          },
          (err, stdout) => {
            release();
            if (err && !stdout) fail(new Error(`${BIN} ${args.join(' ')}: ${err.message}`));
            else ok(stdout);
          },
        );
      }),
  );
}

// ---------------------------------------------------------------- help parser

/** Splits `--help` into about / usage / sections (name → [{name, about}]). */
function parseHelp(text, lang) {
  const lines = text.split('\n');
  const aboutLines = [];
  let i = 0;
  while (i < lines.length && !lines[i].startsWith(`${T[lang].usage}:`)) aboutLines.push(lines[i++]);
  const usage = (lines[i] ?? '').slice(T[lang].usage.length + 1).trim();
  const sections = {};
  let current = null;
  for (i += 1; i < lines.length; i++) {
    const l = lines[i];
    const head = l.match(/^(\S[^:]*):$/);
    if (head) {
      current = head[1];
      sections[current] = [];
    } else if (current && /^\s+\S/.test(l)) {
      const m = l.match(/^\s+(.*?)(?:\s{2,}(.*))?$/);
      sections[current].push({name: m[1].trim(), about: (m[2] ?? '').trim(), raw: l});
    }
  }
  return {about: aboutLines.join(' ').trim(), usage, sections, text};
}

const isGroup = (help) => /[<[](COMMAND|COMANDO)[>\]]/.test(help.usage);
/** A node that has subcommands and also runs by its own name (`plans items [ID] [COMMAND]`). */
const runsItself = (help) => /\[(COMMAND|COMANDO)\]/.test(help.usage);

// ---------------------------------------------------------------- text helpers

/**
 * Escapes what MDX would read as JSX/expressions, outside of code. Relative
 * links in spec text point into each product's own docs site, so they are
 * reduced to their label.
 */
function mdx(text) {
  return String(text ?? '')
    .split(/(```[\s\S]*?```|`[^`\n]*`)/)
    .map((part, i) =>
      i % 2 ? part : part.replace(/\[([^\]]+)\]\((?!https?:)[^)]*\)/g, '$1').replace(/\{/g, '\\{').replace(/\}/g, '\\}').replace(/</g, '&lt;'),
    )
    .join('');
}
/** One line, safe inside a table cell. */
const cell = (text) => mdx(String(text ?? '').replace(/\s*\n+\s*/g, ' ').trim()).replace(/\|/g, '\\|');
/** A description block: headings become bold, so the page outline stays ours. */
const block = (text) =>
  mdx(String(text ?? '').trim()).replace(/^#{1,6}\s+(.*)$/gm, '**$1**');
const yamlStr = (s) => JSON.stringify(String(s));
const fence = (code, lang = 'bash') => `\`\`\`${lang}\n${code}\n\`\`\``;

// ---------------------------------------------------------------- specs

const specCache = new Map();
function specsOf(product) {
  if (!specCache.has(product)) {
    const files = readdirSync(SPECS).filter((f) => f.startsWith(`${product}-v`) && f.endsWith('.json'));
    specCache.set(
      product,
      files.map((f) => {
        const base = f.replace(/\.json$/, '');
        const tr = (l) => {
          const p = join(SPECS, 'i18n', `${base}.${l}.json`);
          return existsSync(p) ? JSON.parse(readFileSync(p, 'utf8')) : {};
        };
        return {doc: JSON.parse(readFileSync(join(SPECS, f), 'utf8')), i18n: {en: tr('en'), es: tr('es')}};
      }),
    );
  }
  return specCache.get(product);
}

const tx = (spec, lang, text) => (text && lang !== 'pt' ? spec.i18n[lang][text] ?? text : text);

function deref(spec, node, depth = 0) {
  if (!node || depth > 8) return node;
  if (node.$ref) {
    const target = node.$ref.replace(/^#\//, '').split('/').reduce((a, k) => a?.[k], spec.doc);
    return deref(spec, target, depth + 1);
  }
  if (node.allOf) {
    const merged = {type: 'object', properties: {}, required: []};
    for (const part of node.allOf) {
      const p = deref(spec, part, depth + 1) ?? {};
      Object.assign(merged.properties, p.properties ?? {});
      merged.required.push(...(p.required ?? []));
    }
    return {...node, ...merged};
  }
  return node;
}

function typeOf(spec, schema) {
  const s = deref(spec, schema);
  if (!s) return '';
  let t = s.type;
  if (Array.isArray(t)) t = t.filter((x) => x !== 'null').join(' | ');
  if (!t && s.oneOf) t = 'oneOf';
  if (!t && s.anyOf) t = 'anyOf';
  if (t === 'array') t = `array<${typeOf(spec, s.items) || 'any'}>`;
  if (s.format && t === 'string') t = `${t} (${s.format})`;
  return t || 'object';
}

/** Rows for the top-level fields of an operation's JSON body. */
function bodyOf(spec, lang, op, key) {
  const t = T[lang];
  const content = op.requestBody?.content ?? {};
  const mime = content['application/json'] ? 'application/json' : Object.keys(content)[0];
  if (!mime) return null;
  const root = deref(spec, content[mime].schema);
  const multipart = mime.startsWith('multipart/');
  const props = root?.properties;
  if (!props || !Object.keys(props).length) return {multipart, other: true, rows: []};
  const required = new Set(root.required ?? []);
  const rows = Object.entries(props).map(([name, raw]) => {
    const s = deref(spec, raw) ?? {};
    let d = tx(spec, lang, raw.description ?? s.description ?? '');
    if (s.enum) d = `${d} ${t.values}: ${s.enum.map((v) => `\`${v}\``).join(', ')}.`.trim();
    return {name, type: typeOf(spec, raw), required: required.has(name), description: d};
  });
  rows.sort((a, b) => Number(b.required) - Number(a.required));
  return {multipart, other: false, rows};
}

function findOperation(product, method, path) {
  for (const spec of specsOf(product)) {
    const op = spec.doc.paths?.[path]?.[method.toLowerCase()];
    if (op) return {spec, op};
  }
  return null;
}

// ---------------------------------------------------------------- command tree

// Answered by every endpoint; stated once in the manual's overview.
const flagOf = (name) => name.split(/\s+/).find((w) => w.startsWith('--'));
let globalFlags = new Set();
const COMMON_ERRORS = new Set(['401', '403', '429', '500']);
const SKIP_ROOT = new Set(['schema', 'completions', 'update', 'accessibility', 'help']);

/** Walks `kobana <product> ...` in one language. */
async function walk(lang, path) {
  const help = parseHelp(await run([...path, '--help'], lang), lang);
  const kids = help.sections[T[lang].commands] ?? [];
  if (!isGroup(help)) return {path, help, children: []};
  const children = await Promise.all(
    kids.filter((k) => k.name !== 'help').map((k) => walk(lang, [...path, k.name])),
  );
  return {path, help, children, self: runsItself(help), kidsAbout: Object.fromEntries(kids.map((k) => [k.name, k.about]))};
}

const leafList = (node) =>
  node.children.length ? [...(node.self ? [node] : []), ...node.children.flatMap(leafList)] : [node];

async function loadLeaf(product, node, lang) {
  // `schema` takes the dotted path; names with a hyphen stay as they are.
  const key = node.path.join('.');
  let schema = null;
  try {
    schema = JSON.parse(await run(['schema', key], lang));
    if (!schema.method) schema = null; // an error object: the command is not an API endpoint
  } catch {
    /* helpers and non-API commands have no schema */
  }
  return {node, key, schema};
}

// ---------------------------------------------------------------- rendering

function renderLeaf(product, lang, rel, leaf, depthPrefix) {
  const t = T[lang];
  const {node, key, schema} = leaf;
  const title = rel.join(' ');
  const out = [];
  const spec = schema ? findOperation(product, schema.method, schema.path) : null;

  out.push(`## ${title}`, '');
  const summary = schema?.summary || node.help.about;
  if (summary) out.push(mdx(summary), '');
  out.push(fence(node.help.usage, 'text'), '');
  if (!schema) {
    const own = Object.values(node.help.sections)
      .flat()
      .filter((o) => o.name.startsWith('-') && !globalFlags.has(flagOf(o.name)) && !o.name.startsWith('-h'));
    if (own.length) {
      out.push(`| ${t.options} | ${t.description} |`, '|---|---|');
      for (const o of own) out.push(`| \`${o.name.replace(/\s+/g, ' ')}\` | ${cell(o.about)} |`);
      out.push('');
    }
  }
  if (schema) {
    out.push(`\`${schema.method} ${schema.path}\``, '');
    if (schema.description && schema.description !== summary) out.push(block(schema.description), '');

    const argsRows = schema.path_params ?? [];
    if (argsRows.length) {
      out.push(`**${t.pathArgs}:** ${argsRows.map((a) => `\`${a}\``).join(', ')}`, '');
    }
    const q = (schema.parameters ?? []).map((p) => ({
      name: p.name,
      type: p.schema?.type ?? '',
      required: p.required,
      description: [p.description || p.schema?.description || '', p.schema?.enum ? `${t.values}: ${p.schema.enum.map((v) => `\`${v}\``).join(', ')}.` : ''].join(' ').trim(),
    }));
    if (q.length) out.push(`**${t.query}**`, '', table(t, q), '');
    const h = (schema.header_params ?? []).filter((p) => !/^X-Request-Source/i.test(p.name)).map((p) => ({name: p.name, type: 'string', required: p.required, description: p.description ?? ''}));
    if (h.length) out.push(`**${t.headers}**`, '', table(t, h), '');

    if (spec) {
      const body = bodyOf(spec.spec, lang, spec.op, key);
      if (body) {
        out.push(`**${t.body}**`, '');
        if (body.multipart) out.push(t.multipart, '');
        if (body.other) out.push(t.bodyOther.replace('{key}', key), '');
        else out.push(table(t, body.rows), '', t.bodyMore.replace('{key}', key), '');
      }
    }
    const resp = Object.entries(schema.responses ?? {});
    if (resp.some(([c]) => !COMMON_ERRORS.has(c))) {
      out.push(`**${t.responses}**`, '');
      out.push(`| ${t.http} | ${t.description} |`, '|---|---|');
      for (const [code, r] of resp.filter(([c]) => !COMMON_ERRORS.has(c))) out.push(`| \`${code}\` | ${cell(r.description)} |`);
      out.push('');
    }
  }
  return out.join('\n');
}

function table(t, rows) {
  const out = [`| ${t.name} | ${t.type} | ${t.required} | ${t.description} |`, '|---|---|---|---|'];
  for (const r of rows) {
    out.push(`| \`${r.name}\` | ${r.type ? `\`${r.type}\`` : ''} | ${r.required ? t.yes : t.no} | ${cell(r.description)} |`);
  }
  return out.join('\n');
}

function front({id, title, label, description, position}) {
  return [
    '---',
    `id: ${id}`,
    `title: ${yamlStr(title)}`,
    label ? `sidebar_label: ${yamlStr(label)}` : null,
    position ? `sidebar_position: ${position}` : null,
    description ? `description: ${yamlStr(description)}` : null,
    '---',
    '',
  ]
    .filter((l) => l !== null)
    .join('\n');
}

// Product pages on the Kobana marketing site, per language (inbox has none).
const PRODUCT_SITE = {
  "pt": {
    "banking": "https://www.kobana.com.br/produtos/gateway-bancario",
    "billing": "https://www.kobana.com.br/produtos/faturamento-automatico",
    "finance": "https://www.kobana.com.br/produtos/financeiro-inteligente",
    "dunning": "https://www.kobana.com.br/produtos/regua-de-cobranca",
    "dashboards": "https://www.kobana.com.br/produtos/dashboard-magico",
    "kadu": "https://www.kobana.com.br/ia/kadu"
  },
  "en": {
    "banking": "https://www.kobana.com.br/en/products/banking-gateway",
    "billing": "https://www.kobana.com.br/en/products/automatic-billing",
    "finance": "https://www.kobana.com.br/en/products/smart-finance",
    "dunning": "https://www.kobana.com.br/en/products/dunning",
    "dashboards": "https://www.kobana.com.br/en/products/magic-dashboard",
    "kadu": "https://www.kobana.com.br/en/ia/kadu"
  },
  "es": {
    "banking": "https://www.kobana.com.br/es/productos/gateway-bancario",
    "billing": "https://www.kobana.com.br/es/productos/facturacion-automatica",
    "finance": "https://www.kobana.com.br/es/productos/financiero-inteligente",
    "dunning": "https://www.kobana.com.br/es/productos/cobranza",
    "dashboards": "https://www.kobana.com.br/es/productos/dashboard-magico",
    "kadu": "https://www.kobana.com.br/es/ia/kadu"
  }
};

// ---------------------------------------------------------------- main

const out = Object.fromEntries(LOCALES.map((l) => [l.code, new Map()]));
const sidebar = {products: [], commands: []};

async function main() {
  // The tree is the same in every language: walk it in pt, then read each
  // language's help for the summaries and usage lines.
  const rootPt = parseHelp(await run(['--help'], 'pt'), 'pt');
  const products = rootPt.sections[T.pt.commands].filter((c) => !SKIP_ROOT.has(c.name)).map((c) => c.name);

  const trees = {};
  for (const l of LOCALES) {
    trees[l.code] = {};
    const root = parseHelp(await run(['--help'], l.code), l.code);
    trees[l.code].root = root;
    for (const p of products) trees[l.code][p] = await walk(l.code, [p]);
    process.stderr.write(`walked ${l.code}\n`);
  }

  for (const l of LOCALES) {
    const t = T[l.code];
    const dest = out[l.code];
    const root = trees[l.code].root;

    // ---- commands/* (global options, schema, completions, update)
    const globalOpts = root.sections[t.options] ?? [];
    dest.set(
      'manual/commands/global-options.md',
      [
        front({id: 'global-options', title: t.globalTitle, label: t.globalTitle, position: 1}),
        `# ${t.globalTitle}`,
        '',
        t.globalIntro,
        '',
        `| ${t.options} | ${t.description} |`,
        '|---|---|',
        ...globalOpts.map((o) => `| \`${o.name.replace(/\s+/g, ' ')}\` | ${cell(o.about)} |`),
        '',
      ].join('\n'),
    );
    const globalNames = new Set(globalOpts.map((o) => flagOf(o.name)));
    globalFlags = globalNames;
    let pos = 2;
    for (const name of ['schema', 'completions', 'update', 'accessibility']) {
      const help = parseHelp(await run([name, '--help'], l.code), l.code);
      const own = (help.sections[t.options] ?? []).filter((o) => !globalNames.has(o.name.split(/\s+/).find((w) => w.startsWith('--'))));
      const args = help.sections[t.arguments] ?? [];
      dest.set(
        `manual/commands/${name}.md`,
        [
          front({id: name, title: `kobana ${name}`, label: name, description: help.about, position: pos++}),
          `# kobana ${name}`,
          mdx(help.about),
          fence(help.usage, 'text'),
          args.length ? `**${t.arguments}**\n\n| | ${t.description} |\n|---|---|\n${args.map((a) => `| \`${a.name}\` | ${cell(a.about)} |`).join('\n')}\n` : '',
          own.length ? `**${t.options}**\n\n| | ${t.description} |\n|---|---|\n${own.map((o) => `| \`${o.name.replace(/\s+/g, ' ')}\` | ${cell(o.about)} |`).join('\n')}\n` : '',
        ].filter((x) => x !== '').join('\n\n'),
      );
    }

    // ---- products
    for (const p of products) {
      const tree = trees[l.code][p];
      const rootNode = trees.pt[p];
      const productLabel = (root.sections[t.commands].find((c) => c.name === p)?.about ?? p).split(' — ')[0];
      const resources = rootNode.children.filter((c) => !c.path.at(-1).startsWith('+'));
      const helpers = rootNode.children.filter((c) => c.path.at(-1).startsWith('+'));
      const env = `KOBANA_${p.toUpperCase()}_TOKEN`;

      const rows = [];
      for (const rnode of resources) {
        const name = rnode.path[1];
        const localized = tree.children.find((c) => c.path[1] === name);
        const kidAbout = tree.kidsAbout?.[name] ?? '';
        rows.push({name, about: kidAbout, node: localized, ptNode: rnode});
      }

      // resource pages (loading schema in this language)
      for (const r of rows) {
        const loaded = await Promise.all(leafList(r.node).map((n) => loadLeaf(p, n, l.code)));
        const parts = loaded.map((leaf) => {
          const rel = leaf.node.path.slice(1);
          return {rel, text: renderLeaf(p, l.code, rel, leaf)};
        });
        const toc = loaded.map((leaf) => {
          const rel = leaf.node.path.slice(1);
          const sch = leaf.schema;
          return `| [\`${rel.join(' ')}\`](#${slug(rel.join(' '))}) | ${sch ? `\`${sch.method} ${sch.path}\`` : ''} | ${cell(sch?.summary || leaf.node.help.about)} |`;
        });
        dest.set(
          `manual/${p}/${r.name}.md`,
          [
            front({
              id: r.name,
              title: `kobana ${p} ${r.name}`,
              label: r.name,
              description: r.about || undefined,
            }),
            `# kobana ${p} ${r.name}`,
            '',
            r.about ? `${mdx(r.about)}\n` : '',
            t.resourceIntro.replace('{p}', p).replace('{r}', r.name),
            '',
            `| ${t.command} | ${t.http} | ${t.description} |`,
            '|---|---|---|',
            ...toc,
            '',
            ...parts.map((x) => x.text),
          ].join('\n'),
        );
      }

      // helpers page
      const helperIds = [];
      if (helpers.length) {
        const sections = [];
        for (const h of helpers) {
          const hn = tree.children.find((c) => c.path[1] === h.path[1]);
          const own = Object.values(hn.help.sections).flat().filter((o) => o.name.startsWith('-') && !globalNames.has(o.name.split(/\s+/).find((w) => w.startsWith('--'))) && !o.name.startsWith('-h'));
          sections.push(
            [
              `## ${h.path[1]}`,
              '',
              mdx(hn.help.about),
              '',
              fence(hn.help.usage, 'text'),
              '',
              own.length ? `| ${t.specific} | ${t.description} |\n|---|---|\n${own.map((o) => `| \`${o.name.replace(/\s+/g, ' ')}\` | ${cell(o.about)} |`).join('\n')}\n` : '',
            ].join('\n'),
          );
        }
        dest.set(
          `manual/${p}/helpers.md`,
          [front({id: 'helpers', title: `kobana ${p} +helpers`, label: t.helpers}), `# ${t.helpers} — ${p}`, '', t.helpersIntro, '', ...sections].join('\n'),
        );
        helperIds.push('helpers');
      }

      // product index
      dest.set(
        `manual/${p}/index.md`,
        [
          front({id: 'index', title: `kobana ${p}`, label: productLabel, description: tree.help.about, position: 1}),
          `# kobana ${p}`,
          '',
          mdx(tree.help.about),
          '',
          ...(PRODUCT_SITE[l.code][p] ? [`[${t.site} →](${PRODUCT_SITE[l.code][p]})`, ''] : []),
          t.prodIntro.replace('{p}', p).replace('{env}', env),
          '',
          `| ${t.resource} | ${t.description} |`,
          '|---|---|',
          ...rows.map((r) => `| [\`${r.name}\`](/manual/${p}/${r.name}) | ${cell(r.about)} |`),
          ...(helpers.length ? [`| [\`+helpers\`](/manual/${p}/helpers) | ${cell(t.helpers)} |`] : []),
          '',
        ].join('\n'),
      );

      if (l.code === 'pt') {
        sidebar.products.push({key: p, resources: [...rows.map((r) => r.name), ...helperIds]});
      }
      sidebarLabels[l.locale] ??= {};
      sidebarLabels[l.locale][p] = productLabel;
      sidebarLabels[l.locale].__commands = t.commands;
      process.stderr.write(`rendered ${l.code} ${p}\n`);
    }
  }
}

const sidebarLabels = {};
const slug = (s) => s.toLowerCase().replace(/[^\p{L}\p{N}\s-]/gu, '').trim().replace(/\s+/g, '-');

await main();

// ---------------------------------------------------------------- write

sidebar.labels = sidebarLabels;
sidebar.commands = ['global-options', 'schema', 'completions', 'update', 'accessibility'];
out.pt.set('__sidebar__', JSON.stringify(sidebar, null, 2) + '\n');

let stale = 0;
for (const l of LOCALES) {
  const base = join(ROOT, l.dir, 'manual');
  const wanted = new Map();
  for (const [rel, content] of out[l.code]) {
    if (rel === '__sidebar__') continue;
    wanted.set(join(ROOT, l.dir, rel), content.endsWith('\n') ? content : content + '\n');
  }
  // Existing generated files this run no longer produces.
  const existing = [];
  const scan = (d) => {
    if (!existsSync(d)) return;
    for (const e of readdirSync(d, {withFileTypes: true})) {
      const p = join(d, e.name);
      if (e.isDirectory()) scan(p);
      else if (e.name.endsWith('.md') && relative(base, p).includes('/')) existing.push(p);
    }
  };
  scan(base);
  if (CHECK) {
    for (const [p, c] of wanted) {
      if (!existsSync(p) || readFileSync(p, 'utf8') !== c) (stale++, console.error(`stale: ${relative(ROOT, p)}`));
    }
    for (const p of existing) if (!wanted.has(p)) (stale++, console.error(`orphan: ${relative(ROOT, p)}`));
  } else {
    for (const p of existing) if (!wanted.has(p)) rmSync(p);
    for (const [p, c] of wanted) {
      mkdirSync(dirname(p), {recursive: true});
      writeFileSync(p, c);
    }
  }
}
const sidebarPath = join(ROOT, 'manual-sidebar.json');
if (CHECK) {
  if (!existsSync(sidebarPath) || readFileSync(sidebarPath, 'utf8') !== out.pt.get('__sidebar__')) (stale++, console.error('stale: manual-sidebar.json'));
  if (stale) {
    console.error(`${stale} file(s) out of date — run: npm run gen-manual`);
    process.exit(1);
  }
  console.log('manual is up to date');
} else {
  writeFileSync(sidebarPath, out.pt.get('__sidebar__'));
  console.log(`manual written: ${[...out.pt.keys()].length - 1} pages per language`);
}

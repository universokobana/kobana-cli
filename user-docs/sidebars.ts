import type {SidebarsConfig} from '@docusaurus/plugin-content-docs';
import manual from './manual-sidebar.json';

const LOCALE = process.env.DOCUSAURUS_CURRENT_LOCALE ?? 'pt-BR';

// Category labels are not documents, so they live here, per locale
// (kobana-next-apps/docs/standards/user-docs-i18n.md).
const LABELS: Record<string, Record<string, string>> = {
  'pt-BR': {
    gettingStarted: 'Primeiros passos',
    usage: 'Uso',
    guides: 'Guias',
    reference: 'Referência',
    commands: 'Comandos',
    products: 'Produtos',
  },
  en: {
    gettingStarted: 'Getting started',
    usage: 'Usage',
    guides: 'Guides',
    reference: 'Reference',
    commands: 'Commands',
    products: 'Products',
  },
  es: {
    gettingStarted: 'Primeros pasos',
    usage: 'Uso',
    guides: 'Guías',
    reference: 'Referencia',
    commands: 'Comandos',
    products: 'Productos',
  },
};
const L = LABELS[LOCALE] ?? LABELS['pt-BR'];

// Product names come from the CLI's own help in each language
// (scripts/generate-manual.mjs → manual-sidebar.json).
const productLabels = (manual.labels as Record<string, Record<string, string>>)[LOCALE] ?? {};

const sidebars: SidebarsConfig = {
  docsSidebar: [
    {
      type: 'category',
      label: L.gettingStarted,
      collapsible: false,
      items: [
        'getting-started/installation',
        'getting-started/authentication',
        'getting-started/quickstart',
        'getting-started/ai-setup',
      ],
    },
    'docs-mcp',
    {
      type: 'category',
      label: L.usage,
      collapsible: false,
      items: [
        'usage/syntax',
        'usage/output',
        'usage/pagination',
        'usage/safe-writes',
        'usage/schema',
        'usage/helpers',
        'usage/languages',
        'usage/scripting',
      ],
    },
    {
      type: 'category',
      label: L.guides,
      collapsible: false,
      items: ['guides/recipes', 'guides/agent-skills', 'guides/troubleshooting'],
    },
    {
      type: 'category',
      label: L.reference,
      collapsible: false,
      items: [
        'reference/products',
        'reference/environment-variables',
        'reference/exit-codes',
        'reference/how-it-works',
      ],
    },
  ],

  manualSidebar: [
    'manual/index',
    {
      type: 'category',
      label: L.commands,
      collapsible: false,
      items: manual.commands.map((id) => `manual/commands/${id}`),
    },
    {
      type: 'category',
      label: L.products,
      collapsible: false,
      items: manual.products.map((p) => ({
        type: 'category' as const,
        label: productLabels[p.key] ?? p.key,
        collapsed: true,
        link: {type: 'doc' as const, id: `manual/${p.key}/index`},
        items: p.resources.map((r) => `manual/${p.key}/${r}`),
      })),
    },
  ],
};

export default sidebars;

import {themes as prismThemes} from 'prism-react-renderer';
import type {Config} from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';

const CURRENT_LOCALE = process.env.DOCUSAURUS_CURRENT_LOCALE ?? 'pt-BR';

const LOCALIZED_TITLE: Record<string, string> = {
  'pt-BR': 'Kobana CLI',
  en: 'Kobana CLI',
  es: 'Kobana CLI',
};

const LOCALIZED_TAGLINE: Record<string, string> = {
  'pt-BR': 'A Kobana no terminal: as APIs de todos os produtos em um único comando',
  en: 'Kobana in your terminal: the APIs of every product in a single command',
  es: 'Kobana en la terminal: las APIs de todos los productos en un solo comando',
};

type LocalePaths = {pt: string; en: string; es: string};
const KB_BASE = 'https://www.kobana.com.br';
const kb = (paths: LocalePaths | string): string => {
  if (typeof paths === 'string') return `${KB_BASE}${paths}`;
  const key = CURRENT_LOCALE === 'en' ? 'en' : CURRENT_LOCALE === 'es' ? 'es' : 'pt';
  return `${KB_BASE}${paths[key]}`;
};

// Computed once so both `baseUrl` and the favicon `headTags` below agree —
// each locale is a separate build with its own baseUrl (`/pt/`, `/en/`,
// `/es/`), and a hardcoded `/favicon/...` href would 404 on all of them.
const BASE_URL = (() => {
  const l = process.env.BUILD_LOCALE;
  if (l === 'pt') return '/pt/';
  if (l === 'en') return '/en/';
  if (l === 'es') return '/es/';
  return '/';
})();
const FAVICON_BASE = `${BASE_URL}favicon/`;

const config: Config = {
  title: LOCALIZED_TITLE[CURRENT_LOCALE] ?? LOCALIZED_TITLE['pt-BR'],
  tagline: LOCALIZED_TAGLINE[CURRENT_LOCALE] ?? LOCALIZED_TAGLINE['pt-BR'],
  favicon: 'favicon.ico',

  future: {
    v4: true,
    faster: {
      swcJsLoader: true,
      swcJsMinimizer: true,
      swcHtmlMinimizer: true,
      lightningCssMinimizer: true,
      mdxCrossCompilerCache: true,
      rspackBundler: true,
      rspackPersistentCache: true,
      ssgWorkerThreads: true,
    },
  },

  url: 'https://cli.kobana.com.br',
  baseUrl: BASE_URL,

  organizationName: 'universokobana',
  projectName: 'kobana-cli',

  onBrokenLinks: 'warn',

  markdown: {
    hooks: {
      onBrokenMarkdownLinks: 'warn',
    },
  },

  i18n: {
    defaultLocale: 'pt-BR',
    locales: ['pt-BR', 'en', 'es'],
    localeConfigs: {
      'pt-BR': { label: 'Português', htmlLang: 'pt-BR', path: 'pt' },
      en: { label: 'English', htmlLang: 'en', path: 'en' },
      es: { label: 'Español', htmlLang: 'es', path: 'es' },
    },
  },

  // Kobana favicon standard (kobana-next-apps/docs/standards/favicon.md).
  // `favicon` above already emits the favicon.ico <link>; everything else
  // in the canonical pack (static/favicon/) is wired here. The SVG's own
  // <link> (no `sizes`) is what makes browsers prefer it over the .ico.
  headTags: [
    {
      tagName: 'link',
      attributes: { rel: 'icon', href: `${FAVICON_BASE}favicon.svg`, type: 'image/svg+xml' },
    },
    {
      tagName: 'link',
      attributes: {
        rel: 'icon',
        type: 'image/png',
        sizes: '16x16',
        href: `${FAVICON_BASE}favicon-16x16.png`,
      },
    },
    {
      tagName: 'link',
      attributes: {
        rel: 'icon',
        type: 'image/png',
        sizes: '32x32',
        href: `${FAVICON_BASE}favicon-32x32.png`,
      },
    },
    {
      tagName: 'link',
      attributes: {
        rel: 'apple-touch-icon',
        sizes: '180x180',
        href: `${FAVICON_BASE}apple-touch-icon.png`,
      },
    },
    {
      tagName: 'link',
      attributes: { rel: 'manifest', href: `${FAVICON_BASE}site.webmanifest` },
    },
    {
      tagName: 'link',
      attributes: {
        rel: 'mask-icon',
        href: `${FAVICON_BASE}safari-pinned-tab.svg`,
        color: '#D3FD54',
      },
    },
    {
      tagName: 'meta',
      attributes: { name: 'msapplication-TileColor', content: '#D3FD54' },
    },
    {
      tagName: 'meta',
      attributes: { name: 'msapplication-config', content: `${FAVICON_BASE}browserconfig.xml` },
    },
  ],

  presets: [
    [
      'classic',
      {
        docs: {
          sidebarPath: './sidebars.ts',
          routeBasePath: '/',
        },
        blog: false,
        theme: {
          customCss: './src/css/custom.css',
        },
      } satisfies Preset.Options,
    ],
  ],

  plugins: [
    [
      '@signalwire/docusaurus-plugin-llms-txt',
      {
        siteTitle: 'Kobana CLI',
        siteDescription:
          'Guia e manual de comandos do kobana, a linha de comando das APIs de todos os produtos Kobana.',
        depth: 3,
        // Quiet mode — the plugin warns once per route conversion failure
        // and once per exclusion summary. Errors still surface.
        logLevel: 0,
        content: {
          enableMarkdownFiles: true,
          enableLlmsFullTxt: true,
          relativePaths: false,
        },
      },
    ],
  ],

  themes: [
    [
      // Offline, i18n-aware search — no external service (Algolia/DocSearch).
      // Builds a hashed lunr index per locale at build time and adds the
      // search bar to the navbar. Docs: github.com/easyops-cn/docusaurus-search-local
      require.resolve('@easyops-cn/docusaurus-search-local'),
      {
        hashed: true,
        // lunr language codes matching the site locales (pt-BR/en/es).
        language: ['pt', 'en', 'es'],
        indexDocs: true,
        indexBlog: false,
        docsRouteBasePath: '/',
        highlightSearchTermsOnTargetPage: true,
        searchResultLimits: 8,
        searchBarShortcutHint: true,
      },
    ],
  ],

  themeConfig: {
    docs: {
      // Collapse button on the left sidebar (swizzled as icon buttons in
      // theme/DocSidebar/Desktop/CollapseButton and DocRoot/.../ExpandButton).
      sidebar: {hideable: true},
    },
    colorMode: {
      defaultMode: 'light',
      respectPrefersColorScheme: true,
      // The toggle is the `custom-colorMode` navbar item (see items above).
      disableSwitch: true,
    },
    navbar: {
      title: 'Kobana CLI',
      logo: {
        // 144x28 keeps the source's 411x80 ratio (5.14). The previous 120x32
        // squashed the mark ~17% horizontally; these values only reserve the box
        // against layout shift — custom.css pins the rendered height.
        alt: 'Kobana',
        src: 'img/kobana-logo-hor-positivo.png',
        srcDark: 'img/kobana-logo-hor-negativo.png',
        width: 144,
        height: 28,
      },
      items: [
        {
          type: 'docSidebar',
          sidebarId: 'docsSidebar',
          position: 'left',
          label: 'Documentação',
        },
        {
          type: 'docSidebar',
          sidebarId: 'manualSidebar',
          position: 'left',
          label: 'Manual',
        },
        {
          // Docs assistant (src/components/Assistant, api/assistant.ts).
          // First of the right group; language and theme sit together after it.
          type: 'custom-askAi',
          position: 'right',
        },
        {
          type: 'localeDropdown',
          position: 'right',
        },
        // Theme toggle and apps menu as items, so they sit before the search
        // (theme/NavbarItem/ComponentTypes; colorMode.disableSwitch below).
        {
          type: 'custom-colorMode',
          position: 'right',
        },
        {
          type: 'custom-appSwitcher',
          position: 'right',
        },
        {
          // Mobile only: the icons above, gathered in a "⋮" menu.
          type: 'custom-mobileActions',
          position: 'right',
        },
        {
          type: 'search',
          position: 'right',
        },
      ],
    },
    footer: {
      style: 'dark',
      logo: {
        alt: 'Kobana',
        src: 'img/kobana-logo-light.png',
        href: kb(''),
        width: 120,
      },
      links: [],
      copyright: `© 2008/${new Date().getFullYear()} Kobana Tecnologia Ltda. | CNPJ: 05.813.794/0001-26 | Calçada das Margaridas, 163, Sala 02 — Centro Comercial Alphaville — Barueri, SP — 06453-038`,
    },
    prism: {
      theme: prismThemes.github,
      darkTheme: prismThemes.dracula,
      additionalLanguages: ['json', 'bash', 'powershell', 'toml'],
    },
  } satisfies Preset.ThemeConfig,
};

export default config;

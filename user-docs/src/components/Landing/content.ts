// Texts of the landing page (src/pages/index.tsx), in the three site locales.

export type Locale = 'pt-BR' | 'en' | 'es';

export const INSTALL_SCRIPT =
  'curl -fsSL https://raw.githubusercontent.com/universokobana/kobana-cli/main/scripts/install.sh | bash';

export interface Product {
  key: string;
  name: Record<Locale, string>;
  text: Record<Locale, string>;
  example: string;
}

export const PRODUCTS: Product[] = [
  {
    key: 'banking',
    name: {'pt-BR': 'Gateway Bancário', en: 'Banking Gateway', es: 'Gateway Bancario'},
    text: {
      'pt-BR': 'Boletos, Pix, TED, extratos e conciliação em mais de 40 bancos.',
      en: 'Bank slips, Pix, wire transfers, statements and reconciliation across 40+ banks.',
      es: 'Boletos, Pix, TED, extractos y conciliación en más de 40 bancos.',
    },
    example: 'kobana banking bank-billets list',
  },
  {
    key: 'finance',
    name: {'pt-BR': 'Financeiro Inteligente', en: 'Smart Finance', es: 'Financiero Inteligente'},
    text: {
      'pt-BR': 'Contas a pagar e a receber, conciliação e fluxo de caixa.',
      en: 'Accounts payable and receivable, reconciliation and cash flow.',
      es: 'Cuentas por pagar y por cobrar, conciliación y flujo de caja.',
    },
    example: 'kobana finance payables list',
  },
  {
    key: 'billing',
    name: {'pt-BR': 'Faturamento Automático', en: 'Automatic Billing', es: 'Facturación Automática'},
    text: {
      'pt-BR': 'Assinaturas, planos, faturas e notas fiscais.',
      en: 'Subscriptions, plans, invoices and tax documents.',
      es: 'Suscripciones, planes, facturas y documentos fiscales.',
    },
    example: 'kobana billing subscriptions list',
  },
  {
    key: 'inbox',
    name: {'pt-BR': 'Inbox Autônomo', en: 'Autonomous Inbox', es: 'Inbox Autónomo'},
    text: {
      'pt-BR': 'Caixas de entrada com agentes que processam cada e-mail.',
      en: 'Inboxes with agents that process every email.',
      es: 'Bandejas de entrada con agentes que procesan cada correo.',
    },
    example: 'kobana inbox emails list',
  },
  {
    key: 'dunning',
    name: {'pt-BR': 'Régua de Cobrança', en: 'Dunning', es: 'Cobranza'},
    text: {
      'pt-BR': 'Cobranças, réguas, acordos, negativações e protestos.',
      en: 'Charges, dunning rules, agreements, credit reporting and protests.',
      es: 'Cobros, reglas de cobranza, acuerdos y reportes a burós.',
    },
    example: 'kobana dunning charges list',
  },
  {
    key: 'dashboards',
    name: {'pt-BR': 'Dashboard Mágico', en: 'Magic Dashboard', es: 'Dashboard Mágico'},
    text: {
      'pt-BR': 'Dashboards e portais sobre as suas fontes de dados.',
      en: 'Dashboards and portals on top of your data sources.',
      es: 'Dashboards y portales sobre tus fuentes de datos.',
    },
    example: 'kobana dashboards data-sources list',
  },
  {
    key: 'kadu',
    name: {'pt-BR': 'Agente Kadu', en: 'Kadu Agent', es: 'Agente Kadu'},
    text: {
      'pt-BR': 'O agente de IA que executa as rotinas financeiras.',
      en: 'The AI agent that runs your financial routines.',
      es: 'El agente de IA que ejecuta tus rutinas financieras.',
    },
    example: 'kobana kadu tasks list',
  },
];

interface Copy {
  title: string;
  description: string;
  eyebrow: string;
  headline: string;
  lead: string;
  ctaDocs: string;
  ctaManual: string;
  copy: string;
  copied: string;
  terminalTitle: string;
  installTitle: string;
  installLead: string;
  installOther: string;
  installMore: string;
  productsTitle: string;
  productsLead: string;
  featuresTitle: string;
  features: Array<{title: string; text: string}>;
  agentsTitle: string;
  agentsText: string;
  agentsLinks: [string, string, string, string, string, string]; // label, href x3
  prompt: {title: string; lead: string; termTitle: string; termLead: string; webTitle: string; webLead: string; copy: string; copied: string; cursor: string; note: string; pasted: string; agent: string; web: string};
  mapTitle: string;
  mapHead: [string, string, string];
  map: Array<[string, string, string, string]>; // name, href, for whom, what
  ctaTitle: string;
  ctaText: string;
}

export const COPY: Record<Locale, Copy> = {
  'pt-BR': {
    title: 'Kobana CLI',
    description: 'A Kobana no terminal: as APIs de todos os produtos em um único comando.',
    eyebrow: 'Kobana CLI',
    headline: 'A Kobana no seu terminal.',
    lead: 'Um único comando para as APIs de todos os produtos Kobana. Feito para pessoas, scripts e agentes de IA: saída em JSON, schema à mão e simulação antes de gravar.',
    ctaDocs: 'Começar a usar',
    ctaManual: 'Ver o manual',
    copy: 'Copiar',
    copied: 'Copiado',
    terminalTitle: 'terminal',
    installTitle: 'Instale em um minuto',
    installLead: 'Um binário só, sem dependências. Depois, entre no produto que vai usar.',
    installOther: 'Homebrew, Nix, binários prontos e código-fonte',
    installMore: 'Todas as formas de instalar',
    productsTitle: 'Todos os produtos, um comando',
    productsLead: 'O produto é sempre o primeiro segmento: kobana <produto> <recurso> <método>. Cada produto tem a sua credencial.',
    featuresTitle: 'Pensado para automatizar',
    features: [
      {title: 'Sempre em sincronia com a API', text: 'Os comandos nascem das especificações OpenAPI embutidas no binário. API nova, comando novo — sem esperar uma versão do CLI escrita à mão.'},
      {title: 'Saída para máquinas', text: 'JSON por padrão, NDJSON na paginação automática, tabela e CSV quando você quiser. Erros com código de saída próprio.'},
      {title: 'Schema no terminal', text: 'kobana schema mostra parâmetros, corpo e respostas de qualquer endpoint, sem abrir a documentação.'},
      {title: 'Teste antes de gravar', text: '--dry-run mostra a requisição sem enviá-la, e a chave de idempotência evita duplicar uma alteração repetida.'},
      {title: 'Português, inglês e espanhol', text: 'Ajuda, mensagens e descrições dos endpoints no idioma do seu sistema.'},
      {title: 'Para agentes de IA', text: 'Entradas validadas contra injeção, --fields para poupar contexto e skills prontas por produto.'},
    ],
    agentsTitle: 'Feito também para agentes de IA',
    agentsText: 'Skills prontas por produto, extensão do Gemini CLI, schema no terminal e --dry-run em tudo que grava. O agente descobre o que fazer sem ler a API inteira.',
    agentsLinks: ['Skills para agentes', '/guides/agent-skills', 'Receitas', '/guides/recipes', 'Como funciona', '/reference/how-it-works'],
    prompt: {title: "Deixe sua IA preparar o ambiente", lead: "Ela instala o MCP da documentação, o CLI e as skills, e guia o login — tudo pedindo a sua confirmação.", termTitle: "Agente com terminal (recomendado)", termLead: "Instalar o CLI e fazer o login acontece na sua máquina. Rode no terminal:", webTitle: "Chat na web (só orientação)", webLead: "Não executam comandos: explicam o que fazer e você roda.", copy: "Copiar", copied: "Copiado", cursor: "Abrir no Cursor", note: "O Gemini não aceita o prompt pela URL: o clique copia o texto e abre o site — é só colar.", pasted: "Prompt copiado — cole no Gemini (Ctrl/⌘ + V).", agent: "Siga as instruções de https://cli.kobana.com.br/pt/getting-started/ai-setup.md para preparar meu ambiente do Kobana CLI.", web: "Leia https://cli.kobana.com.br/pt/getting-started/ai-setup.md e me ajude a instalar e usar o Kobana CLI. Você não executa comandos na minha máquina: me passe os comandos, um passo por vez, para eu rodar."},
    mapTitle: 'O que tem nesta documentação',
    mapHead: ['Seção', 'Para quem', 'O que você encontra'],
    map: [
      ['Documentação', '/getting-started/installation', 'Quem vai usar o kobana no terminal, em scripts ou com agentes.', 'Instalação, autenticação, sintaxe, saída, paginação, schema, helpers e referência.'],
      ['Manual', '/manual', 'Quem procura um comando, uma flag ou um parâmetro.', 'Todos os comandos dos sete produtos, com método HTTP, parâmetros, corpo e respostas.'],
    ],
    ctaTitle: 'Pronto para o primeiro comando?',
    ctaText: 'Instale, entre no produto e liste o que já existe na sua conta.',
  },
  en: {
    title: 'Kobana CLI',
    description: 'Kobana in your terminal: the APIs of every product in a single command.',
    eyebrow: 'Kobana CLI',
    headline: 'Kobana in your terminal.',
    lead: 'One command for the APIs of every Kobana product. Built for people, scripts and AI agents: JSON output, schema at hand, and a dry run before you write.',
    ctaDocs: 'Get started',
    ctaManual: 'Read the manual',
    copy: 'Copy',
    copied: 'Copied',
    terminalTitle: 'terminal',
    installTitle: 'Install in a minute',
    installLead: 'A single binary, no dependencies. Then log in to the product you will use.',
    installOther: 'Homebrew, Nix, ready-made binaries and source',
    installMore: 'All ways to install',
    productsTitle: 'Every product, one command',
    productsLead: 'The product is always the first segment: kobana <product> <resource> <method>. Each product has its own credential.',
    featuresTitle: 'Made for automation',
    features: [
      {title: 'Always in sync with the API', text: 'Commands are built from the OpenAPI specs embedded in the binary. New API, new command — no hand-written CLI release to wait for.'},
      {title: 'Output for machines', text: 'JSON by default, NDJSON when paginating, table and CSV when you want them. Errors have their own exit code.'},
      {title: 'Schema in the terminal', text: 'kobana schema shows the parameters, body and responses of any endpoint without opening the docs.'},
      {title: 'Try before you write', text: '--dry-run shows the request without sending it, and the idempotency key keeps a repeated change from duplicating.'},
      {title: 'Portuguese, English and Spanish', text: 'Help, messages and endpoint descriptions in your system language.'},
      {title: 'For AI agents', text: 'Inputs validated against injection, --fields to save context, and ready-made skills per product.'},
    ],
    agentsTitle: 'Built for AI agents too',
    agentsText: 'Ready-made skills per product, a Gemini CLI extension, schema in the terminal and --dry-run on everything that writes. An agent learns what to do without reading the whole API.',
    agentsLinks: ['Agent skills', '/guides/agent-skills', 'Recipes', '/guides/recipes', 'How it works', '/reference/how-it-works'],
    prompt: {title: "Let your AI set up the environment", lead: "It installs the docs MCP, the CLI and the skills, and guides the login — asking for your confirmation along the way.", termTitle: "Agent with a terminal (recommended)", termLead: "Installing the CLI and logging in happens on your machine. Run in your terminal:", webTitle: "Web chat (guidance only)", webLead: "They do not run commands: they explain what to do and you run it.", copy: "Copy", copied: "Copied", cursor: "Open in Cursor", note: "Gemini does not take the prompt from the URL: the click copies the text and opens the site — just paste it.", pasted: "Prompt copied — paste it into Gemini (Ctrl/⌘ + V).", agent: "Follow the instructions at https://cli.kobana.com.br/en/getting-started/ai-setup.md to set up my Kobana CLI environment.", web: "Read https://cli.kobana.com.br/en/getting-started/ai-setup.md and help me install and use the Kobana CLI. You do not run commands on my machine: give me the commands, one step at a time, for me to run."},
    mapTitle: 'What is in this documentation',
    mapHead: ['Section', 'For whom', 'What you will find'],
    map: [
      ['Documentation', '/getting-started/installation', 'Anyone using kobana in the terminal, in scripts or with agents.', 'Installation, authentication, syntax, output, pagination, schema, helpers and reference.'],
      ['Manual', '/manual', 'Anyone looking for a command, a flag or a parameter.', 'Every command of the seven products, with HTTP method, parameters, body and responses.'],
    ],
    ctaTitle: 'Ready for your first command?',
    ctaText: 'Install it, log in to a product and list what is already in your account.',
  },
  es: {
    title: 'Kobana CLI',
    description: 'Kobana en la terminal: las APIs de todos los productos en un solo comando.',
    eyebrow: 'Kobana CLI',
    headline: 'Kobana en tu terminal.',
    lead: 'Un solo comando para las APIs de todos los productos Kobana. Hecho para personas, scripts y agentes de IA: salida en JSON, schema a mano y simulación antes de escribir.',
    ctaDocs: 'Empezar',
    ctaManual: 'Ver el manual',
    copy: 'Copiar',
    copied: 'Copiado',
    terminalTitle: 'terminal',
    installTitle: 'Instala en un minuto',
    installLead: 'Un solo binario, sin dependencias. Luego, entra al producto que vas a usar.',
    installOther: 'Homebrew, Nix, binarios listos y código fuente',
    installMore: 'Todas las formas de instalar',
    productsTitle: 'Todos los productos, un comando',
    productsLead: 'El producto es siempre el primer segmento: kobana <producto> <recurso> <método>. Cada producto tiene su credencial.',
    featuresTitle: 'Pensado para automatizar',
    features: [
      {title: 'Siempre en sintonía con la API', text: 'Los comandos nacen de las especificaciones OpenAPI embebidas en el binario. API nueva, comando nuevo — sin esperar una versión del CLI escrita a mano.'},
      {title: 'Salida para máquinas', text: 'JSON por defecto, NDJSON en la paginación automática, tabla y CSV cuando quieras. Los errores tienen su propio código de salida.'},
      {title: 'Schema en la terminal', text: 'kobana schema muestra parámetros, cuerpo y respuestas de cualquier endpoint, sin abrir la documentación.'},
      {title: 'Prueba antes de escribir', text: '--dry-run muestra la solicitud sin enviarla, y la clave de idempotencia evita duplicar un cambio repetido.'},
      {title: 'Portugués, inglés y español', text: 'Ayuda, mensajes y descripciones de los endpoints en el idioma de tu sistema.'},
      {title: 'Para agentes de IA', text: 'Entradas validadas contra inyección, --fields para ahorrar contexto y skills listas por producto.'},
    ],
    agentsTitle: 'Hecho también para agentes de IA',
    agentsText: 'Skills listas por producto, extensión de Gemini CLI, schema en la terminal y --dry-run en todo lo que escribe. El agente descubre qué hacer sin leer toda la API.',
    agentsLinks: ['Skills para agentes', '/guides/agent-skills', 'Recetas', '/guides/recipes', 'Cómo funciona', '/reference/how-it-works'],
    prompt: {title: "Deja que tu IA prepare el entorno", lead: "Instala el MCP de la documentación, el CLI y las skills, y guía el inicio de sesión — pidiendo tu confirmación.", termTitle: "Agente con terminal (recomendado)", termLead: "Instalar el CLI e iniciar sesión ocurre en tu máquina. Ejecuta en la terminal:", webTitle: "Chat en la web (solo orientación)", webLead: "No ejecutan comandos: explican qué hacer y tú lo ejecutas.", copy: "Copiar", copied: "Copiado", cursor: "Abrir en Cursor", note: "Gemini no acepta el prompt por la URL: el clic copia el texto y abre el sitio — solo pégalo.", pasted: "Prompt copiado — pégalo en Gemini (Ctrl/⌘ + V).", agent: "Sigue las instrucciones de https://cli.kobana.com.br/es/getting-started/ai-setup.md para preparar mi entorno del Kobana CLI.", web: "Lee https://cli.kobana.com.br/es/getting-started/ai-setup.md y ayúdame a instalar y usar el Kobana CLI. No ejecutas comandos en mi máquina: pásame los comandos, un paso a la vez, para que los ejecute."},
    mapTitle: 'Qué hay en esta documentación',
    mapHead: ['Sección', 'Para quién', 'Qué encuentras'],
    map: [
      ['Documentación', '/getting-started/installation', 'Quien va a usar kobana en la terminal, en scripts o con agentes.', 'Instalación, autenticación, sintaxis, salida, paginación, schema, helpers y referencia.'],
      ['Manual', '/manual', 'Quien busca un comando, una flag o un parámetro.', 'Todos los comandos de los siete productos, con método HTTP, parámetros, cuerpo y respuestas.'],
    ],
    ctaTitle: '¿Listo para el primer comando?',
    ctaText: 'Instálalo, entra a un producto y lista lo que ya existe en tu cuenta.',
  },
};

const q = encodeURIComponent;

/** Terminal agents: the first argument is the initial prompt (checked with each CLI's --help). */
export const TERMINAL_AGENTS = [
  {bin: 'claude', label: 'Claude Code'},
  {bin: 'codex', label: 'Codex'},
  {bin: 'gemini', label: 'Gemini CLI'},
];

export const cursorLink = (prompt: string): string =>
  `cursor://anysphere.cursor-deeplink/prompt?text=${q(prompt)}`;

/** Web chats. `prefill: false` takes no prompt in the URL: the page copies it and opens the site. */
export interface WebChat {
  name: string;
  icon: 'chatgpt' | 'claude' | 'gemini';
  url: (prompt: string) => string;
  prefill: boolean;
}

export const WEB_CHATS: WebChat[] = [
  {name: 'ChatGPT', icon: 'chatgpt', url: (p) => `https://chatgpt.com/?hints=search&q=${q(p)}`, prefill: true},
  {name: 'Claude', icon: 'claude', url: (p) => `https://claude.ai/new?q=${q(p)}`, prefill: true},
  {name: 'Gemini', icon: 'gemini', url: () => 'https://gemini.google.com/app', prefill: false},
];

import {
  Bot,
  Building2,
  Receipt,
  // Sparkles,
  Wallet,
  WandSparkles,
  Waypoints,
  type LucideIcon,
} from 'lucide-react';

export type Locale = 'pt-BR' | 'en' | 'es';
type Group = 'ai' | 'management' | 'infrastructure';

interface AppDef {
  key: string;
  group: Group;
  icon: LucideIcon;
  label: Record<Locale, string>;
  subtitle: Record<Locale, string>;
  /** Marketing site ("Conhecer"). */
  site: string;
  /** Product app ("Acessar"), production. */
  app: string;
  /** Host of the product's user docs: docs.{host}.kobana.com.br. */
  docsHost: string;
  /** Full docs base URL, for docs outside *.kobana.com.br (Kadu). Wins over docsHost. */
  docsUrl?: string;
}

/** Same products, order and copy as the in-app switcher (kobana-apps.ts). */
export const GROUP_LABELS: Record<Group, Record<Locale, string>> = {
  ai: {'pt-BR': 'Inteligência Artificial', en: 'Artificial Intelligence', es: 'Inteligencia Artificial'},
  management: {'pt-BR': 'Gestão', en: 'Management', es: 'Gestión'},
  infrastructure: {'pt-BR': 'Infraestrutura', en: 'Infrastructure', es: 'Infraestructura'},
};

export const ACTIONS: Record<Locale, {docs: string; know: string; open: string; current: string; title: string}> = {
  'pt-BR': {docs: 'Documentação', know: 'Conhecer', open: 'Acessar', current: 'App atual', title: 'Aplicativos Kobana'},
  en: {docs: 'Documentation', know: 'Learn more', open: 'Open', current: 'Current app', title: 'Kobana apps'},
  es: {docs: 'Documentación', know: 'Conocer', open: 'Abrir', current: 'App actual', title: 'Aplicaciones Kobana'},
};

export const CURRENT_APP: string | null = null;

export const APPS: AppDef[] = [
  // Kia ainda não tem documentação — reativar quando publicar.
  // {
  //   key: 'kia',
  //   group: 'ai',
  //   icon: Sparkles,
  //   label: {'pt-BR': 'Kia', en: 'Kia', es: 'Kia'},
  //   subtitle: {
  //     'pt-BR': 'O ambiente integrado do financeiro',
  //     en: 'The integrated environment of finance',
  //     es: 'El entorno integrado de finanzas',
  //   },
  //   site: 'https://www.kia.ia.br',
  //   app: 'https://app.kia.ia.br',
  //   docsHost: 'kia',
  // },
  {
    key: 'kadu',
    group: 'ai',
    icon: Bot,
    label: {'pt-BR': 'Kadu', en: 'Kadu', es: 'Kadu'},
    subtitle: {
      'pt-BR': 'O agente de IA que ajuda a automatizar tarefas',
      en: 'The AI agent that helps automate tasks',
      es: 'El agente de IA que ayuda a automatizar tareas',
    },
    site: 'https://www.kadu.ia.br',
    app: 'https://app.kadu.ia.br',
    docsHost: 'kadu',
    // The Kadu docs live on its own domain, not on docs.kadu.kobana.com.br.
    docsUrl: 'https://docs.kadu.ia.br',
  },
  {
    key: 'billing',
    group: 'management',
    icon: Receipt,
    label: {'pt-BR': 'Faturamento Automático', en: 'Automatic Billing', es: 'Facturación Automática'},
    subtitle: {
      'pt-BR': 'Billing recorrente e flexível',
      en: 'Recurring and flexible billing',
      es: 'Facturación recurrente y flexible',
    },
    site: 'https://billing.kobana.com.br',
    app: 'https://app.billing.kobana.com.br',
    docsHost: 'billing',
  },
  {
    key: 'dashboards',
    group: 'management',
    icon: WandSparkles,
    label: {'pt-BR': 'Dashboard Mágico', en: 'Magic Dashboard', es: 'Dashboard Mágico'},
    subtitle: {
      'pt-BR': 'Portais para os seus clientes',
      en: 'Portals for your customers',
      es: 'Portales para tus clientes',
    },
    site: 'https://dashboards.kobana.com.br',
    app: 'https://app.dashboards.kobana.com.br',
    docsHost: 'dashboards',
  },
  {
    key: 'dunning',
    group: 'management',
    icon: Waypoints,
    label: {'pt-BR': 'Régua de Cobrança', en: 'Dunning', es: 'Reglas de Cobranza'},
    subtitle: {
      'pt-BR': 'Cobrança automática do vencimento ao pagamento',
      en: 'Automatic collection from due date to payment',
      es: 'Cobranza automática desde la fecha de vencimiento hasta el pago',
    },
    site: 'https://dunning.kobana.com.br',
    app: 'https://app.dunning.kobana.com.br',
    docsHost: 'dunning',
  },
  {
    key: 'finance',
    group: 'management',
    icon: Wallet,
    label: {'pt-BR': 'Financeiro Inteligente', en: 'Smart Finance', es: 'Finanzas Inteligentes'},
    subtitle: {
      'pt-BR': 'Back-office financeiro para o Brasil',
      en: 'Financial back-office for Brazil',
      es: 'Back-office financiero para Brasil',
    },
    site: 'https://finance.kobana.com.br',
    app: 'https://app.finance.kobana.com.br',
    docsHost: 'finance',
  },
  {
    key: 'banking',
    group: 'infrastructure',
    icon: Building2,
    label: {'pt-BR': 'Gateway Bancário', en: 'Banking Gateway', es: 'Gateway Bancario'},
    subtitle: {
      'pt-BR': 'Operações bancárias unificadas',
      en: 'Unified banking operations',
      es: 'Operaciones bancarias unificadas',
    },
    site: 'https://www.kobana.com.br/produtos/gateway-bancario',
    app: 'https://app.kobana.com.br',
    docsHost: 'banking',
  },
];

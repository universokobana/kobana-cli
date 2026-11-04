import React, {useEffect, useRef, useState, type ReactNode} from 'react';
import {useLocation} from '@docusaurus/router';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import {ArrowUpRight, Check, ChevronDown, ClipboardCopy, Copy, Sparkles, SquareTerminal} from 'lucide-react';
import {PAGE_ACTIONS} from './config';
import {ChatGptIcon, ClaudeIcon, CursorIcon, MarkdownIcon, VsCodeIcon} from './icons';

import styles from './styles.module.css';

type Locale = 'pt-BR' | 'en' | 'es';

const LABELS = {
  'pt-BR': {
    copyPage: 'Copiar página',
    askAi: 'Perguntar à IA',
    copied: 'Copiado',
    more: 'Mais ações da página',
    mcpSection: 'MCP da Documentação',
    openChatGpt: 'Abrir no ChatGPT',
    openClaude: 'Abrir no Claude',
    connectCursor: 'Conectar ao Cursor',
    connectVsCode: 'Conectar ao VS Code',
    copyMcpConfig: 'Copiar config do MCP',
    copyMcpCommand: 'Copiar comando do MCP',
    viewMarkdown: 'Ver como Markdown',
    prompt: (url: string) => `Leia ${url} e me ajude a entender e usar o conteúdo desta página.`,
  },
  en: {
    copyPage: 'Copy page',
    askAi: 'Ask AI',
    copied: 'Copied',
    mcpSection: 'Docs MCP',
    more: 'More page actions',
    openChatGpt: 'Open in ChatGPT',
    openClaude: 'Open in Claude',
    connectCursor: 'Connect to Cursor',
    connectVsCode: 'Connect to VS Code',
    copyMcpConfig: 'Copy MCP config',
    copyMcpCommand: 'Copy MCP command',
    viewMarkdown: 'View as Markdown',
    prompt: (url: string) => `Read ${url} and help me understand and use the content of this page.`,
  },
  es: {
    copyPage: 'Copiar página',
    askAi: 'Preguntar a la IA',
    copied: 'Copiado',
    more: 'Más acciones de la página',
    mcpSection: 'MCP de la Documentación',
    openChatGpt: 'Abrir en ChatGPT',
    openClaude: 'Abrir en Claude',
    connectCursor: 'Conectar a Cursor',
    connectVsCode: 'Conectar a VS Code',
    copyMcpConfig: 'Copiar config del MCP',
    copyMcpCommand: 'Copiar comando del MCP',
    viewMarkdown: 'Ver como Markdown',
    prompt: (url: string) => `Lee ${url} y ayúdame a entender y usar el contenido de esta página.`,
  },
} as const;

// The llms-txt plugin writes `<route>.md` next to every HTML page, and
// `index.md` only for the locale home. Sites without `trailingSlash: false`
// serve routes as `/x/y/`; that is still `/x/y.md`, never `/x/y/index.md`.
const markdownPath = (pathname: string, baseUrl: string) => {
  if (pathname === baseUrl || `${pathname}/` === baseUrl) return `${baseUrl}index.md`;
  return `${pathname.replace(/\/+$/, '')}.md`;
};

// Safari only lets a page write to the clipboard inside the click; handing it
// a ClipboardItem whose content is still a promise keeps the write in the
// gesture while the fetch runs. Other browsers accept the same call.
async function copyText(text: Promise<string>): Promise<void> {
  if (typeof ClipboardItem !== 'undefined' && navigator.clipboard?.write) {
    const blob = text.then((t) => new Blob([t], {type: 'text/plain'}));
    await navigator.clipboard.write([new ClipboardItem({'text/plain': blob})]);
    return;
  }
  await navigator.clipboard.writeText(await text);
}

type Item = {
  key: string;
  label: string;
  icon: ReactNode;
  external?: boolean;
  href?: string;
  // Copies something: the item shows "Copied" when it resolves.
  onSelect?: () => Promise<void>;
  // Plain action, no feedback.
  action?: () => void;
};

export default function PageActions(): ReactNode {
  const {
    siteConfig: {url: siteUrl, baseUrl},
    i18n: {currentLocale},
  } = useDocusaurusContext();
  const {pathname} = useLocation();
  const t = LABELS[(currentLocale in LABELS ? currentLocale : 'en') as Locale];

  const [open, setOpen] = useState(false);
  // Opened from the keyboard: focus goes to the first item (menu-button pattern).
  const viaKeyboard = useRef(false);
  const [copied, setCopied] = useState<string | null>(null);
  const rootRef = useRef<HTMLDivElement>(null);
  const toggleRef = useRef<HTMLButtonElement>(null);
  const menuRef = useRef<HTMLDivElement>(null);

  const hasMarkdown = !PAGE_ACTIONS.markdownExcluded.some((re) => re.test(pathname));
  const pageUrl = `${siteUrl}${pathname}`;
  const mdUrl = `${siteUrl}${markdownPath(pathname, baseUrl)}`;
  const {mcpServerName: name, mcpUrl} = PAGE_ACTIONS;

  useEffect(() => {
    if (!open) return;
    if (viaKeyboard.current) menuRef.current?.querySelector<HTMLElement>('[role="menuitem"]')?.focus();
    const onPointer = (e: MouseEvent) => {
      if (rootRef.current && !rootRef.current.contains(e.target as Node)) setOpen(false);
    };
    // Escape closes from anywhere, not only with focus inside the menu.
    const onKey = (e: KeyboardEvent) => {
      if (e.key !== 'Escape') return;
      setOpen(false);
      toggleRef.current?.focus();
    };
    document.addEventListener('mousedown', onPointer);
    document.addEventListener('keydown', onKey);
    return () => {
      document.removeEventListener('mousedown', onPointer);
      document.removeEventListener('keydown', onKey);
    };
  }, [open]);

  useEffect(() => {
    if (!copied) return;
    const id = window.setTimeout(() => setCopied(null), 2000);
    return () => window.clearTimeout(id);
  }, [copied]);

  // Pages the llms-txt plugin skips (API reference) have no `.md`: copy the
  // rendered article text instead, so the main button still works there.
  const pageText = (): Promise<string> =>
    hasMarkdown
      ? fetch(markdownPath(pathname, baseUrl)).then((r) => {
          if (!r.ok) throw new Error(`HTTP ${r.status}`);
          return r.text();
        })
      : Promise.resolve(document.querySelector('article')?.innerText ?? '');

  const run = async (key: string, action: () => Promise<void>) => {
    try {
      await action();
      setCopied(key);
    } catch {
      // Clipboard denied or `.md` missing (dev server): nothing to confirm.
    }
  };

  const prompt = encodeURIComponent(t.prompt(hasMarkdown ? mdUrl : pageUrl));
  const mcpConfig = mcpUrl && JSON.stringify({mcpServers: {[name]: {type: 'http', url: mcpUrl}}}, null, 2);

  const sections: Item[][] = [
    PAGE_ACTIONS.assistant
      ? [
          {
            key: 'ask-ai',
            label: t.askAi,
            icon: <Sparkles size={16} aria-hidden />,
            // Root (components/Assistant) listens and opens the panel.
            action: () => {
              setOpen(false);
              window.dispatchEvent(new CustomEvent('kobana:ask-ai'));
            },
          },
        ]
      : [],
    [
      {key: 'chatgpt', label: t.openChatGpt, icon: <ChatGptIcon />, external: true, href: `https://chatgpt.com/?hints=search&q=${prompt}`},
      {key: 'claude', label: t.openClaude, icon: <ClaudeIcon />, external: true, href: `https://claude.ai/new?q=${prompt}`},
    ],
    mcpUrl
      ? [
          {
            key: 'cursor',
            label: t.connectCursor,
            icon: <CursorIcon />,
            href: `cursor://anysphere.cursor-deeplink/mcp/install?name=${encodeURIComponent(name)}&config=${encodeURIComponent(btoaSafe(JSON.stringify({url: mcpUrl})))}`,
          },
          {
            key: 'vscode',
            label: t.connectVsCode,
            icon: <VsCodeIcon />,
            href: `vscode:mcp/install?${encodeURIComponent(JSON.stringify({name, type: 'http', url: mcpUrl}))}`,
          },
          {
            key: 'mcp-config',
            label: t.copyMcpConfig,
            icon: <ClipboardCopy size={16} aria-hidden />,
            onSelect: () => copyText(Promise.resolve(mcpConfig!)),
          },
          {
            key: 'mcp-command',
            label: t.copyMcpCommand,
            icon: <SquareTerminal size={16} aria-hidden />,
            onSelect: () => copyText(Promise.resolve(`npx add-mcp "${mcpUrl}"`)),
          },
        ]
      : [],
    hasMarkdown
      ? [{key: 'markdown', label: t.viewMarkdown, icon: <MarkdownIcon />, external: true, href: markdownPath(pathname, baseUrl)}]
      : [],
  ].filter((s) => s.length > 0);

  const onMenuKey = (e: React.KeyboardEvent) => {
    const items = Array.from(menuRef.current?.querySelectorAll<HTMLElement>('[role="menuitem"]') ?? []);
    const i = items.indexOf(document.activeElement as HTMLElement);
    if (e.key === 'ArrowDown' || e.key === 'ArrowUp') {
      e.preventDefault();
      const next = e.key === 'ArrowDown' ? (i + 1) % items.length : (i - 1 + items.length) % items.length;
      items[next]?.focus();
    } else if (e.key === 'Home' || e.key === 'End') {
      e.preventDefault();
      items[e.key === 'Home' ? 0 : items.length - 1]?.focus();
    }
  };

  return (
    <div
      ref={rootRef}
      className={styles.root}
      onBlur={(e) => {
        if (!e.currentTarget.contains(e.relatedTarget as Node | null)) setOpen(false);
      }}>
      <div className={styles.group}>
        <button type="button" className={styles.main} onClick={() => run('page', () => copyText(pageText()))}>
          {copied === 'page' ? <Check size={16} aria-hidden /> : <Copy size={16} aria-hidden />}
          <span aria-live="polite">{copied === 'page' ? t.copied : t.copyPage}</span>
        </button>
        <button
          ref={toggleRef}
          type="button"
          className={styles.toggle}
          aria-haspopup="menu"
          aria-expanded={open}
          aria-label={t.more}
          title={t.more}
          onMouseDown={(e) => e.preventDefault()}
          onClick={(e) => {
            viaKeyboard.current = e.detail === 0;
            setOpen(!open);
          }}>
          <ChevronDown size={16} aria-hidden />
        </button>
      </div>
      {open && (
        <div ref={menuRef} className={styles.menu} role="menu" aria-label={t.more} onKeyDown={onMenuKey}>
          {sections.map((items, si) => (
            <div key={si} role="group" className={si > 0 ? styles.section : undefined}>
              {items[0]?.key === 'cursor' && <div className={styles.sectionLabel}>{t.mcpSection}</div>}
              {items.map((item) => {
                const done = copied === item.key;
                const content = (
                  <>
                    <span className={styles.itemIcon}>{done ? <Check size={16} aria-hidden /> : item.icon}</span>
                    <span>{done ? t.copied : item.label}</span>
                    {item.external && <ArrowUpRight size={14} className={styles.external} aria-hidden />}
                  </>
                );
                return item.href ? (
                  <a
                    key={item.key}
                    role="menuitem"
                    className={styles.item}
                    href={item.href}
                    {...(item.external ? {target: '_blank', rel: 'noopener noreferrer'} : {})}
                    onClick={() => setOpen(false)}>
                    {content}
                  </a>
                ) : (
                  <button
                    key={item.key}
                    type="button"
                    role="menuitem"
                    className={styles.item}
                    onClick={() => {
                      if (item.action) item.action();
                      else if (item.onSelect) run(item.key, item.onSelect);
                    }}>
                    {content}
                  </button>
                );
              })}
            </div>
          ))}
        </div>
      )}
    </div>
  );
}

// `btoa` exists only in the browser; the static render never opens the menu,
// but the href is computed on every render.
function btoaSafe(s: string): string {
  return typeof btoa === 'function' ? btoa(s) : Buffer.from(s).toString('base64');
}

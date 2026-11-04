import React, {useEffect, useRef, useState, type ReactNode} from 'react';
import type {UIMessage} from 'ai';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import CodeBlock from '@theme/CodeBlock';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import {ArrowUp, BookOpen, Check, Copy, Loader2, Search, Sparkles, Square, Trash2, X} from 'lucide-react';
import {useAssistant} from './context';
import ResizeHandle from './ResizeHandle';
import {LABELS, toLocale} from './labels';

import styles from './styles.module.css';

type Labels = (typeof LABELS)[keyof typeof LABELS];

// Code fences become Docusaurus code blocks (highlight + copy button); inline
// code stays inline. Links open outside the panel's scroll context.
const markdownComponents = {
  pre: ({children}: {children?: ReactNode}) => <>{children}</>,
  code: ({className, children}: {className?: string; children?: ReactNode}) => {
    const text = String(children ?? '');
    const lang = /language-(\w+)/.exec(className ?? '')?.[1];
    if (!lang && !text.includes('\n')) return <code>{children}</code>;
    return <CodeBlock language={lang ?? 'text'}>{text.replace(/\n$/, '')}</CodeBlock>;
  },
  a: ({href, children}: {href?: string; children?: ReactNode}) => (
    <a href={href} target={href?.startsWith('http') ? '_blank' : undefined} rel="noopener noreferrer">
      {children}
    </a>
  ),
};

function ToolChip({part, t}: {part: any; t: Labels}): ReactNode {
  const name: string = part.toolName ?? String(part.type).replace(/^tool-/, '');
  const input = part.input ?? {};
  const running = part.state !== 'output-available' && part.state !== 'output-error';
  const [icon, label] =
    name === 'get_page'
      ? [<BookOpen size={13} key="i" />, `${t.reading} ${input.path ?? ''}`]
      : name === 'list_pages'
        ? [<BookOpen size={13} key="i" />, t.listing]
        : [<Search size={13} key="i" />, `${t.searching}${input.query ? `: "${input.query}"` : ''}`];
  return (
    <div className={styles.tool}>
      {running ? <Loader2 size={13} className={styles.spin} /> : icon}
      <span>{label}</span>
    </div>
  );
}

function CopyButton({text, t}: {text: string; t: Labels}): ReactNode {
  const [copied, setCopied] = useState(false);
  useEffect(() => {
    if (!copied) return;
    const id = window.setTimeout(() => setCopied(false), 2000);
    return () => window.clearTimeout(id);
  }, [copied]);
  return (
    <button
      type="button"
      className={styles.messageAction}
      aria-label={copied ? t.copied : t.copy}
      title={copied ? t.copied : t.copy}
      onClick={() =>
        navigator.clipboard
          .writeText(text)
          .then(() => setCopied(true))
          .catch(() => {})
      }>
      {copied ? <Check size={15} /> : <Copy size={15} />}
    </button>
  );
}

function Message({message, t, streaming}: {message: UIMessage; t: Labels; streaming: boolean}): ReactNode {
  if (message.role === 'user') {
    const text = message.parts.map((p) => (p.type === 'text' ? p.text : '')).join('');
    return <div className={styles.user}>{text}</div>;
  }
  const answer = message.parts
    .map((p) => (p.type === 'text' ? p.text : ''))
    .join('\n\n')
    .trim();
  return (
    <div className={styles.assistant}>
      {message.parts.map((part, i) => {
        if (part.type === 'text') {
          return (
            <div key={i} className={styles.markdown}>
              <ReactMarkdown remarkPlugins={[remarkGfm]} components={markdownComponents as any}>
                {part.text}
              </ReactMarkdown>
            </div>
          );
        }
        if (part.type === 'dynamic-tool' || part.type.startsWith('tool-')) {
          return <ToolChip key={i} part={part} t={t} />;
        }
        return null;
      })}
      {/* The answer as Markdown, once it is complete. */}
      {!streaming && answer && (
        <div className={styles.messageActions}>
          <CopyButton text={answer} t={t} />
        </div>
      )}
    </div>
  );
}

export default function AssistantPanel(): ReactNode {
  const {
    i18n: {currentLocale},
  } = useDocusaurusContext();
  const locale = toLocale(currentLocale);
  const t = LABELS[locale];
  const {open, setOpen, chat} = useAssistant();
  const {messages, sendMessage, status, stop, setMessages, error} = chat;
  const [input, setInput] = useState('');
  const scrollRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLTextAreaElement>(null);
  const busy = status === 'submitted' || status === 'streaming';

  const send = (text: string) => {
    const q = text.trim();
    if (!q || busy) return;
    setInput('');
    sendMessage({text: q});
  };

  useEffect(() => {
    if (open) inputRef.current?.focus();
  }, [open]);

  useEffect(() => {
    scrollRef.current?.scrollTo({top: scrollRef.current.scrollHeight});
  }, [messages, status]);

  useEffect(() => {
    document.documentElement.classList.toggle('assistant-open', open);
    return () => document.documentElement.classList.remove('assistant-open');
  }, [open]);

  useEffect(() => {
    if (!open) return;
    const onKey = (e: KeyboardEvent) => e.key === 'Escape' && setOpen(false);
    document.addEventListener('keydown', onKey);
    return () => document.removeEventListener('keydown', onKey);
  }, [open, setOpen]);

  if (!open) return null;

  const waiting = status === 'submitted' && messages[messages.length - 1]?.role === 'user';

  return (
    <aside className={styles.panel} aria-label={t.title}>
      <ResizeHandle label={t.resize} />
      <header className={styles.header}>
        <Sparkles size={16} aria-hidden />
        <span className={styles.headerTitle}>{t.title}</span>
        {/* Nothing to clear until the conversation has started. */}
        {messages.length > 0 && (
          <button
            type="button"
            className={styles.iconButton}
            onClick={() => {
              stop();
              setMessages([]);
            }}
            aria-label={t.clear}
            title={t.clear}>
            <Trash2 size={16} />
          </button>
        )}
        <button type="button" className={styles.iconButton} onClick={() => setOpen(false)} aria-label={t.close} title={t.close}>
          <X size={16} />
        </button>
      </header>

      <div ref={scrollRef} className={styles.body}>
        {messages.length === 0 ? (
          <div className={styles.empty}>
            <p>{t.empty}</p>
            {t.suggestions.map((s) => (
              <button key={s} type="button" className={styles.suggestion} onClick={() => send(s)}>
                {s}
              </button>
            ))}
          </div>
        ) : (
          messages.map((m, i) => (
            <Message key={m.id} message={m} t={t} streaming={busy && i === messages.length - 1} />
          ))
        )}
        {waiting && (
          <div className={styles.tool}>
            <Loader2 size={13} className={styles.spin} />
          </div>
        )}
        {error && (
          <div className={styles.error} role="alert">
            {/* The transport puts the response body in the message: 429 says rate_limited. */}
            {error.message.includes('rate_limited') ? t.rateLimited : t.error}
          </div>
        )}
      </div>

      <form
        className={styles.form}
        onSubmit={(e) => {
          e.preventDefault();
          send(input);
        }}>
        <textarea
          ref={inputRef}
          className={styles.input}
          value={input}
          maxLength={2000}
          rows={2}
          placeholder={t.placeholder}
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === 'Enter' && !e.shiftKey && !e.nativeEvent.isComposing) {
              e.preventDefault();
              send(input);
            }
          }}
        />
        {busy ? (
          <button type="button" className={styles.send} onClick={() => stop()} aria-label={t.stop} title={t.stop}>
            <Square size={14} />
          </button>
        ) : (
          <button type="submit" className={styles.send} disabled={!input.trim()} aria-label={t.send} title={t.send}>
            <ArrowUp size={16} />
          </button>
        )}
      </form>
      <p className={styles.disclaimer}>{t.disclaimer}</p>
    </aside>
  );
}

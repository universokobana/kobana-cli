import React, {useState, type ReactNode} from 'react';
import Layout from '@theme/Layout';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import {Check, Copy, ExternalLink} from 'lucide-react';
import {COPY, INSTALL_SCRIPT, PRODUCTS, TERMINAL_AGENTS, WEB_CHATS, cursorLink, type Locale} from '../components/Landing/content';
import {ChatGptIcon, ClaudeIcon, CursorIcon, GeminiIcon} from '../components/Landing/icons';
import AnimatedTerminal from '../components/Landing/AnimatedTerminal';
import styles from './index.module.css';

function CopyLine({command, copy, copied}: {command: string; copy: string; copied: string}): ReactNode {
  const [done, setDone] = useState(false);
  const onCopy = () => {
    navigator.clipboard
      ?.writeText(command)
      .then(() => {
        setDone(true);
        setTimeout(() => setDone(false), 2000);
      })
      .catch(() => undefined);
  };
  return (
    <div className={styles.copyLine}>
      <code>
        <span className={styles.prompt}>$</span> {command}
      </code>
      <button type="button" className={styles.copyButton} onClick={onCopy} aria-label={done ? copied : copy}>
        {done ? <Check size={16} aria-hidden /> : <Copy size={16} aria-hidden />}
      </button>
    </div>
  );
}

function copyText(text: string): Promise<void> {
  return navigator.clipboard ? navigator.clipboard.writeText(text) : Promise.reject(new Error('no clipboard'));
}

const ICONS: Record<string, (props: {size?: number}) => ReactNode> = {
  claude: ClaudeIcon,
  codex: ChatGptIcon,
  gemini: GeminiIcon,
  chatgpt: ChatGptIcon,
};

function PromptCard({copy}: {copy: (typeof COPY)['en']['prompt']}): ReactNode {
  const [done, setDone] = useState<string | null>(null);
  const [pasted, setPasted] = useState(false);
  const onCopy = (id: string, text: string) => {
    copyText(text)
      .then(() => {
        setDone(id);
        setTimeout(() => setDone(null), 2000);
      })
      .catch(() => undefined);
  };
  const onCopyAndOpen = () => {
    copyText(copy.web)
      .then(() => {
        setPasted(true);
        setTimeout(() => setPasted(false), 6000);
      })
      .catch(() => undefined);
  };
  return (
    <div className={styles.promptCard}>
      <h2 className={styles.promptTitle}>{copy.title}</h2>
      <p className={styles.promptLead}>{copy.lead}</p>

      <h3 className={styles.promptSub}>{copy.termTitle}</h3>
      <p className={styles.promptHint}>{copy.termLead}</p>
      {TERMINAL_AGENTS.map(({bin, label}) => {
        const command = `${bin} "${copy.agent}"`;
        const Icon = ICONS[bin];
        return (
          <div key={bin} className={styles.agentRow}>
            <div className={styles.agentName}>
              <Icon size={16} />
              {label}
            </div>
            <div className={styles.copyLine}>
              <code>
                <span className={styles.prompt}>$</span> {command}
              </code>
              <button
                type="button"
                className={styles.copyButton}
                onClick={() => onCopy(bin, command)}
                aria-label={done === bin ? copy.copied : `${copy.copy} ${label}`}
              >
                {done === bin ? <Check size={16} aria-hidden /> : <Copy size={16} aria-hidden />}
              </button>
            </div>
          </div>
        );
      })}
      <div className={styles.chips}>
        <a className={styles.chip} href={cursorLink(copy.agent)}>
          <CursorIcon size={14} />
          {copy.cursor}
          <ExternalLink size={12} aria-hidden />
        </a>
      </div>

      <h3 className={styles.promptSub}>{copy.webTitle}</h3>
      <p className={styles.promptHint}>{copy.webLead}</p>
      <div className={styles.chips}>
        {WEB_CHATS.map((c) => {
          const Icon = ICONS[c.icon];
          return (
            <a
              key={c.name}
              className={styles.chip}
              href={c.url(copy.web)}
              target="_blank"
              rel="noopener noreferrer"
              onClick={c.prefill ? undefined : onCopyAndOpen}
            >
              <Icon size={14} />
              {c.name}
              {c.prefill ? <ExternalLink size={12} aria-hidden /> : <Copy size={12} aria-hidden />}
            </a>
          );
        })}
      </div>
      <p className={pasted ? styles.promptPasted : styles.promptNote} role="status">
        {pasted ? copy.pasted : copy.note}
      </p>
    </div>
  );
}

export default function Home(): ReactNode {
  const {i18n} = useDocusaurusContext();
  const locale = (i18n.currentLocale in COPY ? i18n.currentLocale : 'pt-BR') as Locale;
  const c = COPY[locale];

  return (
    <Layout title={c.title} description={c.description}>
      <main className={`kobana-landing ${styles.page}`}>
        <section className={styles.hero}>
          <div className={styles.container}>
            <div className={styles.heroGrid}>
              <div>
                <p className={styles.eyebrow}>{c.eyebrow}</p>
                <h1 className={styles.headline}>{c.headline}</h1>
                <p className={styles.lead}>{c.lead}</p>
                <div className={styles.actions}>
                  <Link className={styles.primary} to="/getting-started/installation">
                    {c.ctaDocs}
                  </Link>
                  <Link className={styles.secondary} to="/manual">
                    {c.ctaManual}
                  </Link>
                </div>

                <AnimatedTerminal title={c.terminalTitle} pause={c.pause} play={c.play} />
              </div>
              <PromptCard copy={c.prompt} />
            </div>

          </div>
        </section>

        <section className={styles.section}>
          <div className={styles.container}>
            <h2>{c.installTitle}</h2>
            <p className={styles.sub}>{c.installLead}</p>
            <CopyLine command={INSTALL_SCRIPT} copy={c.copy} copied={c.copied} />
            <CopyLine command="kobana billing auth login" copy={c.copy} copied={c.copied} />
            <p className={styles.note}>
              {c.installOther}.{' '}
              <Link to="/getting-started/installation">{c.installMore} →</Link>
            </p>
          </div>
        </section>

        <section className={`${styles.section} ${styles.alt}`}>
          <div className={styles.container}>
            <h2>{c.productsTitle}</h2>
            <p className={styles.sub}>{c.productsLead}</p>
            <div className={styles.grid}>
              {PRODUCTS.map((p) => (
                <Link key={p.key} to={`/manual/${p.key}`} className={styles.card}>
                  <h3>{p.name[locale]}</h3>
                  <p>{p.text[locale]}</p>
                  <code>{p.example}</code>
                </Link>
              ))}
            </div>
          </div>
        </section>

        <section className={styles.section}>
          <div className={styles.container}>
            <h2>{c.featuresTitle}</h2>
            <div className={styles.features}>
              {c.features.map((f) => (
                <div key={f.title} className={styles.feature}>
                  <h3>{f.title}</h3>
                  <p>{f.text}</p>
                </div>
              ))}
            </div>
          </div>
        </section>

        <section className={`${styles.section} ${styles.alt}`}>
          <div className={styles.container}>
            <h2>{c.agentsTitle}</h2>
            <p className={styles.sub}>{c.agentsText}</p>
            <div className={styles.actions}>
              {[0, 2, 4].map((i) => (
                <Link key={i} className={i === 0 ? styles.primary : styles.secondary} to={c.agentsLinks[i + 1]}>
                  {c.agentsLinks[i]}
                </Link>
              ))}
            </div>
          </div>
        </section>

        <section className={styles.section}>
          <div className={styles.container}>
            <h2>{c.mapTitle}</h2>
            <table className={styles.map}>
              <thead>
                <tr>
                  {c.mapHead.map((h) => (
                    <th key={h}>{h}</th>
                  ))}
                </tr>
              </thead>
              <tbody>
                {c.map.map(([name, href, who, what]) => (
                  <tr key={name}>
                    <td>
                      <Link to={href}>
                        <strong>{name}</strong>
                      </Link>
                    </td>
                    <td>{who}</td>
                    <td>{what}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </section>

        <section className={styles.cta}>
          <div className={styles.container}>
            <h2>{c.ctaTitle}</h2>
            <p>{c.ctaText}</p>
            <Link className={styles.ctaButton} to="/getting-started/quickstart">
              {c.ctaDocs}
            </Link>
          </div>
        </section>
      </main>
    </Layout>
  );
}

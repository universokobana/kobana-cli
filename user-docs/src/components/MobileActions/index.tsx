import React, {useEffect, useRef, useState, type ReactNode} from 'react';
import {useLocation} from '@docusaurus/router';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import {useColorMode} from '@docusaurus/theme-common';
import {Check, EllipsisVertical, Moon, Sparkles, Sun} from 'lucide-react';
import {ACTIONS, APPS, CURRENT_APP, type Locale} from '../../theme/NavbarItem/AppSwitcher/apps';

import styles from './styles.module.css';

// Below 997px the header icons (Ask AI, language, theme, apps) don't fit next
// to the search: this "⋮" button gathers them in one menu. Navbar item
// `custom-mobileActions`, rendered only at that width (styles.module.css).

const LABELS = {
  'pt-BR': {more: 'Mais opções', askAi: 'Perguntar à IA', language: 'Idioma', toLight: 'Modo claro', toDark: 'Modo escuro'},
  en: {more: 'More options', askAi: 'Ask AI', language: 'Language', toLight: 'Light mode', toDark: 'Dark mode'},
  es: {more: 'Más opciones', askAi: 'Preguntar a la IA', language: 'Idioma', toLight: 'Modo claro', toDark: 'Modo oscuro'},
} as const;

// Same locales, flags and prefix swap as the desktop language menu
// (theme/NavbarItem/LocaleDropdownNavbarItem): stay on the same page.
const LOCALES: {locale: Locale; path: string; flag: string; label: string}[] = [
  {locale: 'pt-BR', path: 'pt', flag: '🇧🇷', label: 'Português'},
  {locale: 'en', path: 'en', flag: '🇺🇸', label: 'English'},
  {locale: 'es', path: 'es', flag: '🇪🇸', label: 'Español'},
];
const PREFIX = /^\/(pt|en|es)(\/|$)/;

const docsUrl = (app: {docsHost: string; docsUrl?: string}, path: string) =>
  `${app.docsUrl ?? `https://docs.${app.docsHost}.kobana.com.br`}/${path}/`;

export default function MobileActions(): ReactNode {
  const {
    i18n: {currentLocale},
  } = useDocusaurusContext();
  const locale = (currentLocale in LABELS ? currentLocale : 'en') as Locale;
  const t = LABELS[locale];
  const apps = ACTIONS[locale];
  const {pathname, search, hash} = useLocation();
  const {colorMode, setColorMode} = useColorMode();
  const [open, setOpen] = useState(false);
  const rootRef = useRef<HTMLDivElement>(null);
  const localePath = LOCALES.find((l) => l.locale === locale)?.path ?? 'pt';

  useEffect(() => {
    if (!open) return;
    const onPointer = (e: MouseEvent) => {
      if (rootRef.current && !rootRef.current.contains(e.target as Node)) setOpen(false);
    };
    const onKey = (e: KeyboardEvent) => e.key === 'Escape' && setOpen(false);
    document.addEventListener('mousedown', onPointer);
    document.addEventListener('keydown', onKey);
    return () => {
      document.removeEventListener('mousedown', onPointer);
      document.removeEventListener('keydown', onKey);
    };
  }, [open]);

  const otherLocaleHref = (path: string) => `/${path}${pathname.replace(PREFIX, '/')}${search}${hash}`;

  return (
    <div ref={rootRef} className={styles.root}>
      <button
        type="button"
        className={styles.trigger}
        aria-haspopup="menu"
        aria-expanded={open}
        aria-label={t.more}
        title={t.more}
        onClick={() => setOpen(!open)}>
        <EllipsisVertical size={18} aria-hidden />
      </button>
      {open && (
        <div className={styles.menu} role="menu" aria-label={t.more}>
          <button
            type="button"
            role="menuitem"
            className={styles.item}
            onClick={() => {
              setOpen(false);
              // The assistant (components/Assistant) listens for this.
              window.dispatchEvent(new CustomEvent('kobana:ask-ai'));
            }}>
            <Sparkles size={16} aria-hidden />
            <span>{t.askAi}</span>
          </button>
          <button
            type="button"
            role="menuitem"
            className={styles.item}
            onClick={() => setColorMode(colorMode === 'dark' ? 'light' : 'dark')}>
            {colorMode === 'dark' ? <Sun size={16} aria-hidden /> : <Moon size={16} aria-hidden />}
            <span>{colorMode === 'dark' ? t.toLight : t.toDark}</span>
          </button>

          <div className={styles.label}>{t.language}</div>
          {LOCALES.map((l) => (
            // Plain <a>: every locale is a separate build, not a client route.
            <a key={l.locale} role="menuitem" className={styles.item} href={otherLocaleHref(l.path)} aria-current={l.locale === locale ? 'true' : undefined}>
              <span className={styles.flag}>{l.flag}</span>
              <span>{l.label}</span>
              {l.locale === locale && <Check size={14} aria-hidden className={styles.check} />}
            </a>
          ))}

          <div className={styles.label}>{apps.title}</div>
          {APPS.map((app) => {
            const Icon = app.icon;
            const current = app.key === CURRENT_APP;
            return (
              <a
                key={app.key}
                role="menuitem"
                className={styles.item}
                href={docsUrl(app, localePath)}
                aria-current={current ? 'true' : undefined}
                {...(current ? {} : {target: '_blank', rel: 'noopener noreferrer'})}>
                <Icon size={16} aria-hidden />
                <span>{app.label[locale]}</span>
                {current && <Check size={14} aria-hidden className={styles.check} />}
              </a>
            );
          })}
        </div>
      )}
    </div>
  );
}

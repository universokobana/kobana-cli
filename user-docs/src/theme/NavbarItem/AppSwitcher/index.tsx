import React, {useEffect, useRef, useState, type ReactNode} from 'react';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import {Check, Grid3X3} from 'lucide-react';
import {ACTIONS, APPS, CURRENT_APP, GROUP_LABELS, type Locale} from './apps';

import styles from './styles.module.css';

// Each docs site is built per locale under /pt, /en and /es; landing on the
// root would re-detect the language instead of keeping the reader's.
const LOCALE_PATH: Record<Locale, string> = {'pt-BR': 'pt', en: 'en', es: 'es'};
const docsUrl = (app: {docsHost: string; docsUrl?: string}, locale: Locale) =>
  `${app.docsUrl ?? `https://docs.${app.docsHost}.kobana.com.br`}/${LOCALE_PATH[locale]}/`;

// Navbar item `custom-appSwitcher` (theme/NavbarItem/ComponentTypes), between
// the theme toggle and the search; not rendered in the mobile menu.
export default function AppSwitcherNavbarItem(): ReactNode {
  const {
    i18n: {currentLocale},
  } = useDocusaurusContext();
  const locale = (currentLocale in ACTIONS ? currentLocale : 'en') as Locale;
  const t = ACTIONS[locale];
  const [open, setOpen] = useState(false);
  const ref = useRef<HTMLDivElement>(null);
  const triggerRef = useRef<HTMLButtonElement>(null);
  const panelRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!open) return;
    panelRef.current?.focus();
    const onPointer = (e: MouseEvent) => {
      if (ref.current && !ref.current.contains(e.target as Node)) setOpen(false);
    };
    const onKey = (e: KeyboardEvent) => {
      if (e.key !== 'Escape') return;
      setOpen(false);
      triggerRef.current?.focus();
    };
    document.addEventListener('mousedown', onPointer);
    document.addEventListener('keydown', onKey);
    return () => {
      document.removeEventListener('mousedown', onPointer);
      document.removeEventListener('keydown', onKey);
    };
  }, [open]);

  const groups = (Object.keys(GROUP_LABELS) as (keyof typeof GROUP_LABELS)[])
    .map((group) => ({group, apps: APPS.filter((a) => a.group === group)}))
    .filter((g) => g.apps.length > 0);

  return (
    <div
      ref={ref}
      className={styles.root}
      onBlur={(e) => {
        // Tabbing out of the panel closes it, as a click outside does.
        if (!e.currentTarget.contains(e.relatedTarget as Node | null)) setOpen(false);
      }}>
      <button
        ref={triggerRef}
        type="button"
        className={styles.trigger}
        aria-haspopup="dialog"
        // Safari and Firefox on macOS don't focus a clicked button, so the
        // click would blur the open panel (closing it) and the toggle would
        // reopen it. Keeping focus where it is leaves the toggle in charge.
        onMouseDown={(e) => e.preventDefault()}
        aria-expanded={open}
        aria-label={t.title}
        title={t.title}
        onClick={() => {
          // Closing unmounts the focused panel; keep focus on the button, as Escape does.
          if (open) triggerRef.current?.focus();
          setOpen(!open);
        }}>
        <Grid3X3 size={16} aria-hidden />
      </button>
      {open && (
        <div ref={panelRef} className={styles.panel} role="dialog" aria-label={t.title} tabIndex={-1}>
          {groups.map(({group, apps}, i) => (
            <section key={group} className={i > 0 ? styles.groupSep : undefined}>
              <h6 className={styles.groupLabel}>{GROUP_LABELS[group][locale]}</h6>
              {apps.map((app) => {
                const Icon = app.icon;
                const isCurrent = app.key === CURRENT_APP;
                return (
                  <div key={app.key} className={styles.card} data-current={isCurrent}>
                    <span className={styles.icon}>
                      <Icon size={20} aria-hidden />
                    </span>
                    <div className={styles.body}>
                      <div className={styles.name}>
                        {app.label[locale]}
                        {isCurrent && <Check size={14} aria-hidden />}
                      </div>
                      <div className={styles.subtitle}>{app.subtitle[locale]}</div>
                      <div className={styles.actions}>
                        {isCurrent ? (
                          <span className={styles.muted}>{t.docs}</span>
                        ) : (
                          <a href={docsUrl(app, locale)} target="_blank" rel="noopener noreferrer">
                            {t.docs}
                          </a>
                        )}
                        <span className={styles.sep}>|</span>
                        <a href={app.site} target="_blank" rel="noopener noreferrer">
                          {t.know}
                        </a>
                        <span className={styles.sep}>|</span>
                        {isCurrent ? (
                          <span className={styles.muted}>{t.current}</span>
                        ) : (
                          <a href={app.app} target={`kobana-${app.key}`} rel="noopener noreferrer">
                            {t.open}
                          </a>
                        )}
                      </div>
                    </div>
                  </div>
                );
              })}
            </section>
          ))}
        </div>
      )}
    </div>
  );
}

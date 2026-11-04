import React, {useEffect, useRef, useState, type ReactNode} from 'react';
import Link from '@docusaurus/Link';
import {useLocation} from '@docusaurus/router';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import {useThemeConfig} from '@docusaurus/theme-common';
import {useActiveDocContext, useLayoutDocsSidebar} from '@docusaurus/plugin-content-docs/client';
import {Check, ChevronDown} from 'lucide-react';

import styles from './styles.module.css';

// Mobile replacement for the top menu, which Docusaurus hides in the burger
// below 997px: the current section's name with a dropdown to the others.
// Reads the left-side navbar items from themeConfig, so new sections (and
// their translated labels) show up with no change here.

type SectionItem = {
  type?: string;
  sidebarId?: string;
  to?: string;
  label?: string;
  position?: 'left' | 'right';
  activeBaseRegex?: string;
};

const isSection = (i: SectionItem) => i.position !== 'right' && (i.type === 'docSidebar' || (!i.type && i.to));

function SidebarLink({item, active, onPick}: {item: SectionItem; active: boolean; onPick: () => void}): ReactNode {
  const link = useLayoutDocsSidebar(item.sidebarId!).link;
  return <SectionLink to={link?.path ?? '/'} label={item.label ?? link?.label ?? ''} active={active} onPick={onPick} />;
}

function SectionLink({to, label, active, onPick}: {to: string; label: string; active: boolean; onPick: () => void}): ReactNode {
  return (
    <Link to={to} className={styles.item} aria-current={active ? 'page' : undefined} onClick={onPick}>
      <span>{label}</span>
      {active && <Check size={14} aria-hidden />}
    </Link>
  );
}

export default function SectionSwitcher(): ReactNode {
  const items = (useThemeConfig().navbar.items as SectionItem[]).filter(isSection);
  const {activeDoc} = useActiveDocContext(undefined);
  const {pathname} = useLocation();
  const {
    siteConfig: {baseUrl},
  } = useDocusaurusContext();
  // Remember on which page the menu was opened: navigating closes it with no
  // effect (setState in an effect would re-render in cascade).
  const [openOn, setOpenOn] = useState<string | null>(null);
  const rootRef = useRef<HTMLDivElement>(null);

  const isActive = (i: SectionItem) => {
    if (i.type === 'docSidebar') return activeDoc?.sidebar === i.sidebarId;
    if (i.activeBaseRegex) return new RegExp(i.activeBaseRegex).test(pathname);
    const target = `${baseUrl}${i.to!.replace(/^\//, '')}`;
    return pathname === target || pathname.startsWith(`${target}/`);
  };
  const current = items.find(isActive);
  const open = openOn === pathname;
  const setOpen = (value: boolean) => setOpenOn(value ? pathname : null);

  useEffect(() => {
    if (!open) return;
    const onPointer = (e: MouseEvent) => {
      if (rootRef.current && !rootRef.current.contains(e.target as Node)) setOpenOn(null);
    };
    const onKey = (e: KeyboardEvent) => e.key === 'Escape' && setOpenOn(null);
    document.addEventListener('mousedown', onPointer);
    document.addEventListener('keydown', onKey);
    return () => {
      document.removeEventListener('mousedown', onPointer);
      document.removeEventListener('keydown', onKey);
    };
  }, [open]);

  if (items.length < 2) return null;

  return (
    <div ref={rootRef} className={styles.root}>
      <button type="button" className={styles.trigger} aria-haspopup="true" aria-expanded={open} onClick={() => setOpen(!open)}>
        <span className={styles.current}>{current?.label ?? items[0].label}</span>
        <ChevronDown size={14} aria-hidden />
      </button>
      {open && (
        <nav className={styles.menu}>
          {items.map((i) =>
            i.type === 'docSidebar' ? (
              <SidebarLink key={i.sidebarId} item={i} active={i === current} onPick={() => setOpen(false)} />
            ) : (
              <SectionLink key={i.to} to={i.to!} label={i.label ?? ''} active={i === current} onPick={() => setOpen(false)} />
            ),
          )}
        </nav>
      )}
    </div>
  );
}

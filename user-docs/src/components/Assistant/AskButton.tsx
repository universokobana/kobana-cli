import React, {type ReactNode} from 'react';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import {Sparkles} from 'lucide-react';
import {useAssistant} from './context';
import {LABELS, toLocale} from './labels';

import styles from './styles.module.css';

// Navbar item `custom-askAi` (theme/NavbarItem/ComponentTypes).
export default function AskAiNavbarItem({mobile}: {mobile?: boolean}): ReactNode {
  const {
    i18n: {currentLocale},
  } = useDocusaurusContext();
  const t = LABELS[toLocale(currentLocale)];
  const {open, setOpen} = useAssistant();
  return (
    <button
      type="button"
      className={mobile ? 'menu__link' : styles.askButton}
      aria-expanded={open}
      aria-label={t.askAi}
      title={t.askAi}
      onClick={() => setOpen(!open)}>
      <Sparkles size={15} aria-hidden />
      <span className={mobile ? undefined : styles.askLabel}>{t.askAi}</span>
    </button>
  );
}

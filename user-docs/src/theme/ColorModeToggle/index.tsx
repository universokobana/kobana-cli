import React, {type ReactNode} from 'react';
import clsx from 'clsx';
import useIsBrowser from '@docusaurus/useIsBrowser';
import {useColorMode} from '@docusaurus/theme-common';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import {Moon, Sun} from 'lucide-react';
import type {Props} from '@theme/ColorModeToggle';

import styles from './styles.module.css';

// Same control as the apps' dashboard header: Sun in light, Moon in dark,
// swapped by CSS on `data-theme` (works before hydration), and a plain
// light/dark toggle with no "system" third state.
const LABELS: Record<string, {toLight: string; toDark: string}> = {
  'pt-BR': {toLight: 'Modo claro', toDark: 'Modo escuro'},
  en: {toLight: 'Light mode', toDark: 'Dark mode'},
  es: {toLight: 'Modo claro', toDark: 'Modo oscuro'},
};

export default function ColorModeToggle({className, buttonClassName}: Props): ReactNode {
  const isBrowser = useIsBrowser();
  const {colorMode, setColorMode} = useColorMode();
  const {
    i18n: {currentLocale},
  } = useDocusaurusContext();
  const t = LABELS[currentLocale] ?? LABELS.en;
  const label = colorMode === 'dark' ? t.toLight : t.toDark;

  return (
    <div className={className}>
      <button
        type="button"
        className={clsx('clean-btn', styles.toggle, buttonClassName)}
        disabled={!isBrowser}
        onClick={() => setColorMode(colorMode === 'dark' ? 'light' : 'dark')}
        title={label}
        aria-label={label}>
        <Sun size={16} aria-hidden className={styles.sun} />
        <Moon size={16} aria-hidden className={styles.moon} />
      </button>
    </div>
  );
}

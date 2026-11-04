import React, {type ReactNode} from 'react';
import {translate} from '@docusaurus/Translate';
import {PanelLeftOpen} from 'lucide-react';
import type {Props} from '@theme/DocRoot/Layout/Sidebar/ExpandButton';

import styles from './styles.module.css';

// The collapsed sidebar stays a clickable strip (as in Docusaurus), with the
// apps' "open panel" icon at the top instead of an arrow in the middle.
export default function ExpandButton({toggleSidebar}: Props): ReactNode {
  const label = translate({
    id: 'theme.docs.sidebar.expandButtonTitle',
    message: 'Expand sidebar',
    description: 'The ARIA label and title attribute for expand button of doc sidebar',
  });
  return (
    <div
      className={styles.expand}
      title={label}
      aria-label={label}
      tabIndex={0}
      role="button"
      onKeyDown={(e) => {
        if (e.key === 'Enter' || e.key === ' ') {
          e.preventDefault();
          toggleSidebar();
        }
      }}
      onClick={toggleSidebar}>
      <span className={styles.icon}>
        <PanelLeftOpen size={16} aria-hidden />
      </span>
    </div>
  );
}

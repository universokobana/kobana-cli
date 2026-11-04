import React, {type ReactNode} from 'react';
import {translate} from '@docusaurus/Translate';
import {PanelLeftClose} from 'lucide-react';
import type {Props} from '@theme/DocSidebar/Desktop/CollapseButton';

import styles from './styles.module.css';

// Icon button at the top of the sidebar, like the apps' sidebar toggle,
// instead of Docusaurus' full-width bar at the bottom.
export default function CollapseButton({onClick}: Props): ReactNode {
  const label = translate({
    id: 'theme.docs.sidebar.collapseButtonTitle',
    message: 'Collapse sidebar',
    description: 'The title attribute for collapse button of doc sidebar',
  });
  return (
    <button type="button" title={label} aria-label={label} className={styles.collapse} onClick={onClick}>
      <PanelLeftClose size={16} aria-hidden />
    </button>
  );
}

import React, {type ReactNode} from 'react';
import Logo from '@theme-original/Navbar/Logo';
import SectionSwitcher from '../../../components/SectionSwitcher';

import styles from './styles.module.css';

// Logo + product name, plus the mobile section switcher. The apps menu is a
// navbar item on the right (custom-appSwitcher), next to the search.
export default function NavbarLogoWithSwitcher(): ReactNode {
  return (
    <div className={styles.group}>
      <Logo />
      {/* Mobile only: the top menu as "current section" + dropdown. */}
      <SectionSwitcher />
    </div>
  );
}

import React, {type ReactNode} from 'react';
import ComponentTypes from '@theme-original/NavbarItem/ComponentTypes';
import ColorModeToggle from '@theme/ColorModeToggle';
import {useColorMode} from '@docusaurus/theme-common';
import AskAiNavbarItem from '../../components/Assistant/AskButton';
import AppSwitcherNavbarItem from './AppSwitcher';
import MobileActions from '../../components/MobileActions';

import styles from './ComponentTypes.module.css';

// The right side of the navbar is declared item by item, so the order is
// ours: Ask AI, language, theme, apps menu, search. Docusaurus' own theme
// toggle always comes after every item, so the stock one is nulled
// (theme/Navbar/ColorModeToggle) and this one is rendered as `custom-colorMode`.

function ColorModeNavbarItem({mobile}: {mobile?: boolean}): ReactNode {
  const {colorMode, setColorMode} = useColorMode();
  return (
    <ColorModeToggle
      className={mobile ? 'menu__list-item' : styles.desktopOnly}
      value={colorMode}
      onChange={setColorMode}
      respectPrefersColorScheme={false}
    />
  );
}

// The apps menu has no place in the mobile menu (it was hidden there before).
function AppSwitcherItem({mobile}: {mobile?: boolean}): ReactNode {
  return mobile ? null : <AppSwitcherNavbarItem />;
}

// Below 997px: one "⋮" menu with the icons above (components/MobileActions).
// Not inside the burger menu, which has its own copies of these controls.
function MobileActionsItem({mobile}: {mobile?: boolean}): ReactNode {
  return mobile ? null : <MobileActions />;
}

const ComponentTypesWithCustomItems = {
  ...ComponentTypes,
  'custom-askAi': AskAiNavbarItem,
  'custom-colorMode': ColorModeNavbarItem,
  'custom-appSwitcher': AppSwitcherItem,
  'custom-mobileActions': MobileActionsItem,
};

export default ComponentTypesWithCustomItems;

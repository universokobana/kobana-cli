import type {ReactNode} from 'react';

// The toggle is the `custom-colorMode` navbar item (theme/NavbarItem/
// ComponentTypes), so the stock one is suppressed here instead of through
// colorMode.disableSwitch — that flag also wipes the stored choice on every
// mount, which made the light/dark pick revert on reload.
export default function NavbarColorModeToggle(): ReactNode {
  return null;
}

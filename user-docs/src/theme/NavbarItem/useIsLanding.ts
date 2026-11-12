import {useLocation} from '@docusaurus/router';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';

// The landing (src/pages/index.tsx) is the locale's root page. It is dark in
// both color modes, so the theme toggle only belongs on the docs and manual.
export default function useIsLanding(): boolean {
  const {
    siteConfig: {baseUrl},
  } = useDocusaurusContext();
  const {pathname} = useLocation();
  const trim = (p: string) => p.replace(/\/+$/, '');
  return trim(pathname) === trim(baseUrl);
}

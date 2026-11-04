import React, {type ReactNode} from 'react';
import clsx from 'clsx';
import {ThemeClassNames, useThemeConfig} from '@docusaurus/theme-common';
import {useHomePageRoute} from '@docusaurus/theme-common/internal';
import {useDocsSidebar, useLayoutDocsSidebar, useSidebarBreadcrumbs} from '@docusaurus/plugin-content-docs/client';
import Link from '@docusaurus/Link';
import {translate} from '@docusaurus/Translate';
import HomeBreadcrumbItem from '@theme/DocBreadcrumbs/Items/Home';
import DocBreadcrumbsStructuredData from '@theme/DocBreadcrumbs/StructuredData';
import PageActions from '../../components/PageActions';

import styles from './styles.module.css';

// Docusaurus' breadcrumbs (same markup and classes), plus two things:
// - the top-menu section the page belongs to (Documentação, API, MCP, CLI)
//   right after Home, linking to the start of that section;
// - the page actions menu (components/PageActions) on the same row.
// Both DocItem and the OpenAPI ApiItem layout render DocBreadcrumbs, so this
// covers every doc page.

function Crumb({children, href, active}: {children: ReactNode; href?: string; active?: boolean}): ReactNode {
  return (
    <li className={clsx('breadcrumbs__item', {'breadcrumbs__item--active': active})}>
      {href && !active ? (
        <Link className="breadcrumbs__link" href={href}>
          <span>{children}</span>
        </Link>
      ) : (
        <span className="breadcrumbs__link">{children}</span>
      )}
    </li>
  );
}

type NavItem = {type?: string; sidebarId?: string; label?: string; position?: string};

// The navbar item (label, already translated) whose sidebar holds this page.
function useSectionItem(): NavItem | undefined {
  const sidebar = useDocsSidebar();
  const items = useThemeConfig().navbar.items as NavItem[];
  return sidebar ? items.find((i) => i.type === 'docSidebar' && i.sidebarId === sidebar.name) : undefined;
}

function SectionCrumb({item, active}: {item: NavItem; active: boolean}): ReactNode {
  const link = useLayoutDocsSidebar(item.sidebarId!).link;
  return (
    <Crumb href={link?.path} active={active}>
      {item.label ?? link?.label}
    </Crumb>
  );
}

export default function DocBreadcrumbsWithSection(): ReactNode {
  const breadcrumbs = useSidebarBreadcrumbs();
  const homePageRoute = useHomePageRoute();
  const section = useSectionItem();

  return (
    <div className={styles.row}>
      {breadcrumbs && (
        <>
          <DocBreadcrumbsStructuredData breadcrumbs={breadcrumbs} />
          <nav
            className={clsx(ThemeClassNames.docs.docBreadcrumbs, styles.breadcrumbsContainer)}
            aria-label={translate({
              id: 'theme.docs.breadcrumbs.navAriaLabel',
              message: 'Breadcrumbs',
              description: 'The ARIA label for the breadcrumbs',
            })}>
            <ul className="breadcrumbs">
              {homePageRoute && <HomeBreadcrumbItem />}
              {section && <SectionCrumb item={section} active={breadcrumbs.length === 0} />}
              {breadcrumbs.map((item, idx) => {
                const isLast = idx === breadcrumbs.length - 1;
                const href = item.type === 'category' && item.linkUnlisted ? undefined : item.href;
                return (
                  <Crumb key={idx} href={href} active={isLast}>
                    {item.label}
                  </Crumb>
                );
              })}
            </ul>
          </nav>
        </>
      )}
      <PageActions />
    </div>
  );
}

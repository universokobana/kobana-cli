import React, {type ReactNode} from 'react';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import {translate} from '@docusaurus/Translate';
import {mergeSearchStrings, useHistorySelector} from '@docusaurus/theme-common';
import DropdownNavbarItem from '@theme/NavbarItem/DropdownNavbarItem';
import {Globe} from 'lucide-react';
import type {LinkLikeNavbarItemProps} from '@theme/NavbarItem';
import type {Props} from '@theme/NavbarItem/LocaleDropdownNavbarItem';

import styles from './styles.module.css';

function useLocaleDropdownUtils() {
  const {
    i18n: {localeConfigs},
  } = useDocusaurusContext();
  const search = useHistorySelector((history) => history.location.search);
  const hash = useHistorySelector((history) => history.location.hash);
  const pathname = useHistorySelector((history) => history.location.pathname);

  const getLocaleConfig = (locale: string) => {
    const localeConfig = localeConfigs[locale];
    if (!localeConfig) {
      throw new Error(
        `Docusaurus bug, no locale config found for locale=${locale}`,
      );
    }
    return localeConfig;
  };

  // Per-locale builds each ship with their own baseUrl, and Docusaurus'
  // default alternate-URL helper resolves entries against the current
  // build's baseUrl — so the dropdown would dead-end on the same locale.
  // Rebuild the target URL by swapping the locale prefix in the current
  // pathname (doc slugs are identical across locales).
  const LOCALE_PREFIX: Record<string, string> = {
    'pt-BR': '/pt',
    en: '/en',
    es: '/es',
  };
  const PREFIX_PATTERN = /^\/(pt|en|es)(\/|$)/;
  const getBaseURLForLocale = (locale: string) => {
    const target = LOCALE_PREFIX[locale] ?? '';
    const stripped = pathname.replace(PREFIX_PATTERN, '/');
    const next = `${target}${stripped === '/' ? '/' : stripped}`;
    return `pathname://${next}`;
  };

  return {
    getURL: (locale: string, options: {queryString: string | undefined}) => {
      // We have 2 query strings because
      // - there's the current one
      // - there's one user can provide through navbar config
      // see https://github.com/facebook/docusaurus/pull/8915
      const finalSearch = mergeSearchStrings(
        [search, options.queryString],
        'append',
      );
      return `${getBaseURLForLocale(locale)}${finalSearch}${hash}`;
    },
    getLabel: (locale: string) => {
      return getLocaleConfig(locale).label;
    },
    getLang: (locale: string) => {
      return getLocaleConfig(locale).htmlLang;
    },
  };
}

// Same menu as the apps' dashboard header: a Globe icon button (no caret,
// no current-language text) opening flag + name items.
const FLAGS: Record<string, string> = {'pt-BR': '🇧🇷', en: '🇺🇸', es: '🇪🇸'};

export default function LocaleDropdownNavbarItem({
  mobile,
  dropdownItemsBefore,
  dropdownItemsAfter,
  queryString,
  ...props
}: Props): ReactNode {
  const utils = useLocaleDropdownUtils();

  const {
    i18n: {currentLocale, locales},
  } = useDocusaurusContext();
  const localeItems = locales.map((locale): LinkLikeNavbarItemProps => {
    return {
      label: `${FLAGS[locale] ?? ''} ${utils.getLabel(locale)}`.trim(),
      lang: utils.getLang(locale),
      to: utils.getURL(locale, {queryString}),
      target: '_self',
      autoAddBaseUrl: false,
      className:
        locale === currentLocale
          ? // Similar idea as DefaultNavbarItem: select the right Infima active
            // class name. This cannot be substituted with isActive, because the
            // target URLs contain `pathname://` and therefore are not NavLinks!
            mobile
            ? 'menu__link--active'
            : 'dropdown__link--active'
          : '',
    };
  });

  const items = [...dropdownItemsBefore, ...localeItems, ...dropdownItemsAfter];

  // Mobile keeps Docusaurus' text label inside the burger menu.
  const mobileLabel = translate({
    message: 'Languages',
    id: 'theme.navbar.mobileLanguageDropdown.label',
    description: 'The label for the mobile language switcher dropdown',
  });

  return (
    <DropdownNavbarItem
      {...props}
      mobile={mobile}
      className={mobile ? undefined : styles.trigger}
      aria-label={mobile ? undefined : mobileLabel}
      label={mobile ? mobileLabel : <Globe size={16} aria-hidden className={styles.globe} />}
      items={items}
    />
  );
}

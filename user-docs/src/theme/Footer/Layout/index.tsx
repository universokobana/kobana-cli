import React, {type ReactNode} from 'react';
import clsx from 'clsx';
import {ThemeClassNames} from '@docusaurus/theme-common';
import useBaseUrl from '@docusaurus/useBaseUrl';
import Translate from '@docusaurus/Translate';
import type {Props} from '@theme/Footer/Layout';

export default function FooterLayout({
  style,
  links,
  copyright,
}: Props): ReactNode {
  const kobanaLogo = useBaseUrl('/img/kobana-logo-site.webp');
  const cegonha = useBaseUrl('/img/kobana-cegonha.webp');
  const badgeCarbono = useBaseUrl('/img/badge-neutra-carbono.webp');
  const badgeNatureza = useBaseUrl('/img/badge-parceira-natureza.webp');

  return (
    <footer
      className={clsx(ThemeClassNames.layout.footer.container, 'footer footer-kobana', {
        'footer--dark': style === 'dark',
      })}>
      <div className="container container-fluid">
        {links}

        <div className="footer-kobana__row">
          <div className="footer-kobana__badges">
            {/* eslint-disable-next-line @next/next/no-img-element */}
            <img src={badgeCarbono} alt="Empresa Neutra em Carbono" width="100" height="100" loading="lazy" decoding="async" />
            {/* eslint-disable-next-line @next/next/no-img-element */}
            <img src={badgeNatureza} alt="Empresa Parceira da Natureza" width="100" height="100" loading="lazy" decoding="async" />
          </div>
          <div className="footer-kobana__commercial">
            <span className="footer-kobana__commercial-label">
              <Translate id="footer.commercial.label" description="Footer commercial label">
                Comercial
              </Translate>
            </span>
            <a className="footer-kobana__phone" href="tel:+551130030386">
              3003-0386
            </a>
            <div className="footer-kobana__contact-icons">
              <a
                href="https://wa.me/551130030386"
                aria-label="WhatsApp"
                target="_blank"
                rel="noopener noreferrer">
                <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                  <path d="M21 11.5a8.38 8.38 0 0 1-.9 3.8 8.5 8.5 0 0 1-7.6 4.7 8.38 8.38 0 0 1-3.8-.9L3 21l1.9-5.7a8.38 8.38 0 0 1-.9-3.8 8.5 8.5 0 0 1 4.7-7.6 8.38 8.38 0 0 1 3.8-.9h.5a8.48 8.48 0 0 1 8 8v.5z" />
                </svg>
              </a>
              <a href="mailto:comercial@kobana.com.br" aria-label="E-mail">
                <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                  <path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z" />
                  <polyline points="22,6 12,13 2,6" />
                </svg>
              </a>
            </div>
          </div>
        </div>

        <div className="footer-kobana__row footer-kobana__row--bottom">
          <div className="footer-kobana__brand">
            {/* eslint-disable-next-line @next/next/no-img-element */}
            <img src={kobanaLogo} alt="Kobana" className="footer-kobana__logo" width="120" height="28" loading="lazy" decoding="async" />
            {/* eslint-disable-next-line @next/next/no-img-element */}
            <img src={cegonha} alt="" className="footer-kobana__bird" width="142" height="80" loading="lazy" decoding="async" />
          </div>
          <div className="footer-kobana__copyright">{copyright}</div>
        </div>
      </div>
    </footer>
  );
}

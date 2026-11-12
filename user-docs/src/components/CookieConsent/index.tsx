import {useEffect, type ReactNode} from 'react';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import 'vanilla-cookieconsent/dist/cookieconsent.css';
import type * as CookieConsentType from 'vanilla-cookieconsent';

// Same banner as www.kobana.com.br (kobana-site-2026: components/cookie-consent):
// vanilla-cookieconsent, same copy and categories. Policy and contact links
// point at the main site, which owns those pages.

type Translation = CookieConsentType.Translation;

const ptTranslations: Translation = {
  consentModal: {
    title: "Utilizamos cookies",
    description:
      "Usamos cookies para melhorar sua experiência, analisar o tráfego do site e personalizar conteúdo. Você pode escolher quais cookies aceitar. <a href='https://www.kobana.com.br/privacidade' class='cc-link'>Política de Privacidade</a>",
    acceptAllBtn: "Aceitar todos",
    acceptNecessaryBtn: "Apenas necessários",
    showPreferencesBtn: "Gerenciar preferências",
  },
  preferencesModal: {
    title: "Preferências de cookies",
    acceptAllBtn: "Aceitar todos",
    acceptNecessaryBtn: "Apenas necessários",
    savePreferencesBtn: "Salvar preferências",
    closeIconLabel: "Fechar",
    serviceCounterLabel: "Serviço(s)",
    sections: [
      {
        title: "Suas escolhas de privacidade",
        description:
          "Nesta seção você pode gerenciar suas preferências de cookies. Clique nas categorias abaixo para saber mais e ajustar suas configurações. Bloquear algumas categorias pode impactar sua experiência no site.",
      },
      {
        title: "Cookies estritamente necessários",
        description:
          "Esses cookies são essenciais para o funcionamento do site. Eles permitem funcionalidades básicas como navegação entre páginas e acesso a áreas seguras. O site não funciona corretamente sem esses cookies.",
        linkedCategory: "necessary",
      },
      {
        title: "Cookies de análise e desempenho",
        description:
          "Esses cookies nos permitem entender como os visitantes interagem com o site, coletando informações de forma anônima. Isso nos ajuda a melhorar continuamente o site.",
        linkedCategory: "analytics",
        cookieTable: {
          headers: {
            name: "Nome",
            description: "Descrição",
            duration: "Duração",
          },
          body: [
            {
              name: "_ga",
              description: "Usado pelo Google Analytics para distinguir usuários",
              duration: "2 anos",
            },
            {
              name: "_gid",
              description: "Usado pelo Google Analytics para distinguir usuários",
              duration: "24 horas",
            },
          ],
        },
      },
      {
        title: "Cookies de marketing e publicidade",
        description:
          "Esses cookies são usados para rastrear visitantes em diferentes sites e exibir anúncios mais relevantes. Também são utilizados para limitar o número de vezes que um anúncio é exibido.",
        linkedCategory: "marketing",
        cookieTable: {
          headers: {
            name: "Nome",
            description: "Descrição",
            duration: "Duração",
          },
          body: [
            {
              name: "_fbp",
              description: "Usado pelo Facebook para entrega de anúncios",
              duration: "3 meses",
            },
          ],
        },
      },
      {
        title: "Mais informações",
        description:
          'Para dúvidas sobre nossa política de cookies, entre em <a class="cc-link" href="https://www.kobana.com.br/contato">contato conosco</a>.',
      },
    ],
  },
};

const enTranslations: Translation = {
  consentModal: {
    title: "We use cookies",
    description:
      "We use cookies to enhance your experience, analyze site traffic, and personalize content. You can choose which cookies to accept. <a href='https://www.kobana.com.br/en/privacy' class='cc-link'>Privacy Policy</a>",
    acceptAllBtn: "Accept all",
    acceptNecessaryBtn: "Necessary only",
    showPreferencesBtn: "Manage preferences",
  },
  preferencesModal: {
    title: "Cookie preferences",
    acceptAllBtn: "Accept all",
    acceptNecessaryBtn: "Necessary only",
    savePreferencesBtn: "Save preferences",
    closeIconLabel: "Close",
    serviceCounterLabel: "Service(s)",
    sections: [
      {
        title: "Your privacy choices",
        description:
          "In this section you can manage your cookie preferences. Click on the categories below to learn more and adjust your settings. Blocking some categories may impact your experience on the site.",
      },
      {
        title: "Strictly necessary cookies",
        description:
          "These cookies are essential for the website to function. They enable basic features like page navigation and access to secure areas. The website cannot function properly without these cookies.",
        linkedCategory: "necessary",
      },
      {
        title: "Analytics and performance cookies",
        description:
          "These cookies allow us to understand how visitors interact with the website by collecting information anonymously. This helps us continuously improve the site.",
        linkedCategory: "analytics",
        cookieTable: {
          headers: {
            name: "Name",
            description: "Description",
            duration: "Duration",
          },
          body: [
            {
              name: "_ga",
              description: "Used by Google Analytics to distinguish users",
              duration: "2 years",
            },
            {
              name: "_gid",
              description: "Used by Google Analytics to distinguish users",
              duration: "24 hours",
            },
          ],
        },
      },
      {
        title: "Marketing and advertising cookies",
        description:
          "These cookies are used to track visitors across websites and display more relevant ads. They are also used to limit the number of times an ad is shown.",
        linkedCategory: "marketing",
        cookieTable: {
          headers: {
            name: "Name",
            description: "Description",
            duration: "Duration",
          },
          body: [
            {
              name: "_fbp",
              description: "Used by Facebook for ad delivery",
              duration: "3 months",
            },
          ],
        },
      },
      {
        title: "More information",
        description:
          'For questions about our cookie policy, please <a class="cc-link" href="https://www.kobana.com.br/en/contact">contact us</a>.',
      },
    ],
  },
};

const esTranslations: Translation = {
  consentModal: {
    title: "Usamos cookies",
    description:
      "Usamos cookies para mejorar tu experiencia, analizar el tráfico del sitio y personalizar el contenido. Puedes elegir qué cookies aceptar. <a href='https://www.kobana.com.br/es/privacidad' class='cc-link'>Política de Privacidad</a>",
    acceptAllBtn: "Aceptar todas",
    acceptNecessaryBtn: "Solo necesarias",
    showPreferencesBtn: "Gestionar preferencias",
  },
  preferencesModal: {
    title: "Preferencias de cookies",
    acceptAllBtn: "Aceptar todas",
    acceptNecessaryBtn: "Solo necesarias",
    savePreferencesBtn: "Guardar preferencias",
    closeIconLabel: "Cerrar",
    serviceCounterLabel: "Servicio(s)",
    sections: [
      {
        title: "Tus opciones de privacidad",
        description:
          "En esta sección puedes gestionar tus preferencias de cookies. Haz clic en las categorías a continuación para obtener más información y ajustar tu configuración. Bloquear algunas categorías puede afectar tu experiencia en el sitio.",
      },
      {
        title: "Cookies estrictamente necesarias",
        description:
          "Estas cookies son esenciales para el funcionamiento del sitio. Permiten funcionalidades básicas como la navegación entre páginas y el acceso a áreas seguras. El sitio no funciona correctamente sin estas cookies.",
        linkedCategory: "necessary",
      },
      {
        title: "Cookies de análisis y rendimiento",
        description:
          "Estas cookies nos permiten entender cómo interactúan los visitantes con el sitio, recopilando información de forma anónima. Esto nos ayuda a mejorar continuamente el sitio.",
        linkedCategory: "analytics",
        cookieTable: {
          headers: {
            name: "Nombre",
            description: "Descripción",
            duration: "Duración",
          },
          body: [
            {
              name: "_ga",
              description: "Usado por Google Analytics para distinguir usuarios",
              duration: "2 años",
            },
            {
              name: "_gid",
              description: "Usado por Google Analytics para distinguir usuarios",
              duration: "24 horas",
            },
          ],
        },
      },
      {
        title: "Cookies de marketing y publicidad",
        description:
          "Estas cookies se utilizan para rastrear a los visitantes en diferentes sitios y mostrar anuncios más relevantes. También se utilizan para limitar el número de veces que se muestra un anuncio.",
        linkedCategory: "marketing",
        cookieTable: {
          headers: {
            name: "Nombre",
            description: "Descripción",
            duration: "Duración",
          },
          body: [
            {
              name: "_fbp",
              description: "Usado por Facebook para la entrega de anuncios",
              duration: "3 meses",
            },
          ],
        },
      },
      {
        title: "Más información",
        description:
          'Para consultas sobre nuestra política de cookies, <a class="cc-link" href="https://www.kobana.com.br/es/contacto">ponte en contacto con nosotros</a>.',
      },
    ],
  },
};


let started = false;

export default function CookieConsent(): ReactNode {
  const {
    i18n: {currentLocale},
  } = useDocusaurusContext();
  const lang = currentLocale === 'en' ? 'en' : currentLocale === 'es' ? 'es' : 'pt';

  useEffect(() => {
    let cancelled = false;
    // Off the critical path, like the main site: the banner is not the LCP.
    const idle = (cb: () => void) =>
      typeof window.requestIdleCallback === 'function'
        ? window.requestIdleCallback(cb, {timeout: 2000})
        : window.setTimeout(cb, 1500);

    idle(() => {
      void run().catch((e) => console.error('[cookie-consent]', e));
    });

    async function run() {
      const CC = await import('vanilla-cookieconsent');
      // Layout remounts on navigation; the banner must be started only once.
      if (cancelled || started) return;
      started = true;
      await CC.run({
        categories: {
          necessary: {enabled: true, readOnly: true},
          analytics: {autoClear: {cookies: [{name: /^_ga/}, {name: '_gid'}]}},
          marketing: {autoClear: {cookies: [{name: '_fbp'}, {name: /^_fbc/}]}},
        },
        guiOptions: {
          consentModal: {layout: 'box inline', position: 'bottom right', equalWeightButtons: true, flipButtons: false},
          preferencesModal: {layout: 'box', equalWeightButtons: true, flipButtons: false},
        },
        language: {
          default: lang,
          translations: {pt: ptTranslations, en: enTranslations, es: esTranslations},
        },
      });
    }
    return () => {
      cancelled = true;
    };
  }, [lang]);

  return null;
}

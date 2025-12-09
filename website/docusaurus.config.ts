import {themes as prismThemes} from 'prism-react-renderer';
import type {Config} from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';

// This runs in Node.js - Don't use client-side code here (browser APIs, JSX...)

const config: Config = {
  title: 'Ygégé',
  tagline: 'High-Performance Indexer for YGG Torrent',
  favicon: 'img/ygege-logo.png',

  // Future flags, see https://docusaurus.io/docs/api/docusaurus-config#future
  future: {
    v4: true, // Improve compatibility with the upcoming Docusaurus v4
  },

  // Set the production url of your site here
  url: 'https://ygege.lila.ws',
  // Set the /<baseUrl>/ pathname under which your site is served
  // For custom domain, use root path
  baseUrl: '/',

  // GitHub pages deployment config.
  // If you aren't using GitHub pages, you don't need these.
  organizationName: 'UwUDev', // Usually your GitHub org/user name.
  projectName: 'ygege', // Usually your repo name.

  onBrokenLinks: 'throw',

  // Internationalization - French as default, English available
  i18n: {
    defaultLocale: 'fr',
    locales: ['fr', 'en'],
    localeConfigs: {
      fr: {
        label: 'Français',
        direction: 'ltr',
        htmlLang: 'fr-FR',
      },
      en: {
        label: 'English',
        direction: 'ltr',
        htmlLang: 'en-US',
      },
    },
  },

  themes: [
    [
      '@easyops-cn/docusaurus-search-local',
      {
        hashed: true,
        language: ['fr', 'en'],
        indexDocs: true,
        indexBlog: false,
        indexPages: false,
        docsRouteBasePath: '/',
        searchResultLimits: 8,
        searchResultContextMaxLength: 50,
        explicitSearchResultPath: true,
        ignoreFiles: [],
      },
    ],
  ],

  presets: [
    [
      'classic',
      {
        docs: {
          sidebarPath: './sidebars.ts',
          editUrl: 'https://github.com/UwUDev/ygege/tree/develop/website/',
          routeBasePath: '/', // Docs en racine
        },
        blog: false, // Désactiver le blog
        theme: {
          customCss: './src/css/custom.css',
        },
      } satisfies Preset.Options,
    ],
  ],

  themeConfig: {
    announcementBar: {
      id: 'support_ygege',
      content:
        '⭐️ Si vous aimez Ygégé, donnez-lui une étoile sur <a target="_blank" rel="noopener noreferrer" href="https://github.com/UwUDev/ygege">GitHub</a> !',
      backgroundColor: '#fafbfc',
      textColor: '#091E42',
      isCloseable: true,
    },
    image: 'img/ygege-logo-text.png',
    colorMode: {
      defaultMode: 'dark',
      disableSwitch: false,
      respectPrefersColorScheme: true,
    },
    navbar: {
      title: 'Ygégé',
      logo: {
        alt: 'Ygégé Logo',
        src: 'img/ygege-logo.png',
      },
      items: [
        {
          type: 'docSidebar',
          sidebarId: 'tutorialSidebar',
          position: 'left',
          label: 'Documentation',
        },
        {
          type: 'localeDropdown',
          position: 'right',
        },
        {
          href: 'https://github.com/UwUDev/ygege',
          label: 'GitHub',
          position: 'right',
        },
      ],
    },
    footer: {
      style: 'dark',
      links: [
        {
          title: 'Docs',
          items: [
            {
              label: 'Documentation',
              to: '/',
            },
          ],
        },
        {
          title: 'Community',
          items: [
            {
              label: 'GitHub',
              href: 'https://github.com/UwUDev/ygege',
            },
            {
              label: 'YGG Torrent',
              href: 'https://www.yggtorrent.fi',
            },
          ],
        },
        {
          title: 'More',
          items: [
            {
              label: 'Docker Hub',
              href: 'https://hub.docker.com/r/uwucode/ygege',
            },
            {
              label: 'GitHub Issues',
              href: 'https://github.com/UwUDev/ygege/issues',
            },
          ],
        },
      ],
      copyright: `Copyright © ${new Date().getFullYear()} UwUDev. Construit avec Docusaurus.`,
    },
    prism: {
      theme: prismThemes.github,
      darkTheme: prismThemes.vsDark,
    },
  } satisfies Preset.ThemeConfig,
};

export default config;

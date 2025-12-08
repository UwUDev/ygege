import type {SidebarsConfig} from '@docusaurus/plugin-content-docs';

// This runs in Node.js - Don't use client-side code here (browser APIs, JSX...)

/**
 * Creating a sidebar enables you to:
 - create an ordered group of docs
 - render a sidebar for each doc of that group
 - provide next/previous navigation

 The sidebars can be generated from the filesystem, or explicitly defined here.

 Create as many sidebars as you want.
 */
const sidebars: SidebarsConfig = {
  tutorialSidebar: [
    'intro',
    'getting-started',
    {
      type: 'category',
      label: 'Installation',
      items: ['installation/docker-guide', 'installation/binary-guide', 'installation/source-guide'],
    },
    'configuration',
    {
      type: 'category',
      label: 'Integrations',
      items: ['integrations/prowlarr', 'integrations/jackett'],
    },
    'tmdb-imdb',
    {
      type: 'category',
      label: 'Développeur',
      items: [
        'developer/contributing',
        'developer/ci-implementation',
        'developer/preview-workflow',
        'developer/release-workflow',
      ],
    },
    'api',
    'faq',
  ],
};

export default sidebars;

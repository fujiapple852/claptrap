import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import starlightVersions from 'starlight-versions'

// https://astro.build/config
export default defineConfig({
    site: 'https://claptrap.cli.rs',
    integrations: [
        starlight({
            plugins: [
              starlightVersions({
                versions: [{ slug: '0.1.0' }, { slug: '0.2.0' }],
              }),
            ],
            title: 'Claptrap',
            customCss: [
              // Relative path to your custom CSS file
              './src/styles/custom.css',
            ],
             editLink: {
               baseUrl: 'https://github.com/fujiapple852/claptrap/edit/master/docs/',
             },
            head: [
              {
                tag: 'link',
                attrs: {
                  rel: 'apple-touch-icon',
                  href: '/apple-touch-icon.png',
                },
              },
              {
                tag: 'link',
                attrs: {
                  rel: 'icon',
                  type: 'image/png',
                  href: '/favicon-96x96.png',
                },
              },
              {
                tag: 'script',
                attrs: {
                  defer: true,
                  src: 'https://cloud.umami.is/script.js',
                  'data-website-id': 'f894e1bd-3555-44b2-abe6-833da265d669',
                  'data-astro-rerun': true
                }
            }
            ],
            social: [
                { icon: 'github', label: 'github', href: 'https://github.com/fujiapple852/claptrap' },
                { icon: 'x.com', label: 'x.com', href: 'https://x.com/FujiApple852v2' },
            ],
            sidebar: [
                {
                    label: 'Start Here',
                    autogenerate: { directory: 'start' }
                },
                {
                    label: 'Guides',
                    autogenerate: { directory: 'guides' }
                },
                {
                    label: 'Reference',
                    items: [
                        'reference/overview',
                        'reference/cli',
                        'reference/api',
                        {
                            label: 'API Reference', items: [
                                'reference/api/command',
                                'reference/api/arg',
                                'reference/api/arg_group',
                                'reference/api/value_parser',
                                { label: 'Command', autogenerate: { directory: 'reference/api/command' } },
                                { label: 'Arg', autogenerate: { directory: 'reference/api/arg' } },
                                { label: 'ArgGroup', autogenerate: { directory: 'reference/api/arg_group' } },
                                { label: 'ValueParser', autogenerate: { directory: 'reference/api/value_parser' } },
                            ]
                        },

                    ]
                }
            ],
        }),
    ],
});

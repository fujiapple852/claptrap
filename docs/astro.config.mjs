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
                versions: [{ slug: '0.0.0' }],
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
                    autogenerate: { directory: 'reference' },
                },
                {
                    label: 'Development',
                    autogenerate: { directory: 'development' },
                },
            ],
        }),
    ],
});

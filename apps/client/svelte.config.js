import { sveltePreprocess } from 'svelte-preprocess';
import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  // Consult https://kit.svelte.dev/docs/integrations#preprocessors
  // for more information about preprocessors
  preprocess: vitePreprocess({ preprocess: sveltePreprocess({ postcss: true }) }),

  kit: {
    adapter: adapter(),
    alias: {
      $lib: 'src/lib',
      '$shadcn-svelte-ui-primitives': '../../packages/shadcn-svelte-ui-primitives/src/lib'
    }
  }
};

export default config;

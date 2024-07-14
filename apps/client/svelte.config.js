import { sveltePreprocess } from 'svelte-preprocess';
// eslint-disable-next-line import/no-extraneous-dependencies
import adapter from '@sveltejs/adapter-static';
// eslint-disable-next-line node/no-unpublished-import
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  // Consult https://kit.svelte.dev/docs/integrations#preprocessors
  // for more information about preprocessors
  preprocess: vitePreprocess({ preprocess: sveltePreprocess({ postcss: true }) }),

  kit: {
    adapter: adapter(),
    alias: {
      $lib: 'src/lib'
    }
  }
};

export default config;

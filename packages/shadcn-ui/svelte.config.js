import { sveltePreprocess } from 'svelte-preprocess';
import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

// FIXME: For some reason svelkit errors out because it cannot detect source maps
// when running this library in dev mode. It's not abig issue since most of the
// time this will be run via the tauri app, but I'd like to figure out what is
// going wrong here

/** @type {import('@sveltejs/kit').Config} */
const config = {
  // Consult https://kit.svelte.dev/docs/integrations#preprocessors
  // for more information about preprocessors
  preprocess: vitePreprocess({ preprocess: sveltePreprocess({ postcss: true }) }),

  kit: {
    // adapter-auto only supports some environments, see https://kit.svelte.dev/docs/adapter-auto for a list.
    // If your environment is not supported, or you settled on a specific environment, switch out the adapter.
    // See https://kit.svelte.dev/docs/adapters for more information about adapters.
    adapter: adapter(),
    alias: {
      $lib: 'src/lib',
      $util: 'src/lib/util',
      $components: 'src/lib/components'
    }
  }
};

export default config;

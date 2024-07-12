import adapter from '@sveltejs/adapter-auto';
// eslint-disable-next-line node/no-unpublished-import
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://kit.svelte.dev/docs/integrations#preprocessors
	// for more information about preprocessors
	preprocess: vitePreprocess(),

	kit: {
		adapter: adapter(),
		alias: {
			'@browser-package': '../../packages/browser-package/src/index.ts',
			$lib: 'src/lib',
		},
	},
};

export default config;
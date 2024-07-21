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
      $util: 'src/lib/util',
      $components: 'src/lib/components',
      '@shadcn-ui/badge': '../../packages/shadcn-ui/src/lib/components/ui/badge/index.ts',
      '@shadcn-ui/button': '../../packages/shadcn-ui/src/lib/components/ui/button/index.ts',
      '@shadcn-ui/card': '../../packages/shadcn-ui/src/lib/components/ui/card/index.ts',
      '@shadcn-ui/collapsible':
        '../../packages/shadcn-ui/src/lib/components/ui/collapsible/index.ts',
      '@shadcn-ui/command': '../../packages/shadcn-ui/src/lib/components/ui/command/index.ts',
      '@shadcn-ui/dialog': '../../packages/shadcn-ui/src/lib/components/ui/dialog/index.ts',
      '@shadcn-ui/input': '../../packages/shadcn-ui/src/lib/components/ui/input/index.ts',
      '@shadcn-ui/label': '../../packages/shadcn-ui/src/lib/components/ui/label/index.ts',
      '@shadcn-ui/popover': '../../packages/shadcn-ui/src/lib/components/ui/popover/index.ts',
      '@shadcn-ui/resizable': '../../packages/shadcn-ui/src/lib/components/ui/resizable/index.ts',
      '@shadcn-ui/scroll-area':
        '../../packages/shadcn-ui/src/lib/components/ui/scroll-area/index.ts',
      '@shadcn-ui/separator': '../../packages/shadcn-ui/src/lib/components/ui/separator/index.ts',
      '@shadcn-ui/form': '../../packages/shadcn-ui/src/lib/components/ui/form/index.ts'
    }
  }
};

export default config;

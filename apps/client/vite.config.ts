// eslint-disable-next-line node/no-unpublished-import
import { defineConfig } from 'vitest/config';
import { sveltekit } from '@sveltejs/kit/vite';

// eslint-disable-next-line import/no-default-export
export default defineConfig({
  plugins: [sveltekit()],
  test: {
    include: ['src/**/*.{test,spec}.{js,ts}']
  }
});

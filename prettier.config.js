// https://www.npmjs.com/package/prettier-config-moon
const config = require('prettier-config-moon');

module.exports = {
  ...config,
  plugins: ['prettier-plugin-svelte', 'prettier-plugin-tailwindcss'],
  // pluginSearchDirs: ['.'],
  useTabs: false,
  singleQuote: true,
  trailingComma: 'none',
  printWidth: 100,
  arrowParens: 'avoid',
  semi: true,
  overrides: [...config.overrides, { files: '*.svelte', options: { parser: 'svelte' } }]
};

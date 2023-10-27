/** @type { import('@storybook/web-components').Preview } */

import '../storybook.css';

import init from '../dist/storybook';
window.bevy_init_done = false;
await init()
  .then(() => {
    console.log("wasm init done");
    window.bevy_init_done = true;
  });

const preview = {
  parameters: {
    actions: { argTypesRegex: "^on[A-Z].*" },
    controls: {
      matchers: {
        color: /(background|color)$/i,
        date: /Date$/i,
      },
    },
  },
};

export default preview;

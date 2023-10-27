import { game_page_component } from "../../../dist/storybook";

function buildElement() {
  const parent = document.createElement('div');
  game_page_component(parent);
  return parent;
}

export default {
  title: 'Pages/GamePage',
  render: (args) => buildElement(args),
};

export const Default = {
  args: {
    
  },
};

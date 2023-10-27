import { board_piece_component } from "../../../dist/storybook";

const canvas_id = "bevy-canvas";

async function loadBevy(message) {
  let canvas = document.querySelector(`#${canvas_id}`);
  if (!canvas || !window.bevy_init_done) {
    message.innerText += ".";
    setTimeout(() => {
      loadBevy(message);
    }, 1000)
    return;
  }

  message.style = "display: none";
  board_piece_component(`#${canvas_id}`);
}

function buildElement() {
  const parent = document.createElement('div');
  parent.id = "bevy-canvas-container";

  const message = document.createElement('p');
  message.innerText = "loading...";
  parent.appendChild(message);

  const canvas = document.createElement('canvas');
  canvas.id = canvas_id;
  parent.appendChild(canvas);

  loadBevy(message);

  return parent;
}

export default {
  title: 'Board/Pieces',
  render: (args) => buildElement(args),
};

export const Default = {
  args: {

  },
};

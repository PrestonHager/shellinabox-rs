import init, { start } from './pkg/shellinabox_web.js';

const canvas = document.createElement('canvas');
canvas.width = 800;
canvas.height = 400;
document.body.appendChild(canvas);

const ws = new WebSocket(`ws://${location.host}/ws`);
ws.binaryType = 'arraybuffer';

init().then(() => start(canvas, ws));

document.addEventListener('keydown', (e) => {
  if (e.key.length === 1 || e.key === 'Enter' || e.key === 'Backspace') {
    ws.send(e.key === 'Enter' ? '\n' : e.key);
    e.preventDefault();
  }
});

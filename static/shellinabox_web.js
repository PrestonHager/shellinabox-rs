const ws = new WebSocket(`ws://${location.host}/ws`);
const canvas = document.createElement('canvas');
document.body.appendChild(canvas);
// Sugarloaf expects a canvas to render the terminal
// `Sugarloaf` should be provided by the compiled WASM package
const term = new Sugarloaf(canvas);

ws.binaryType = 'arraybuffer';
ws.onmessage = (event) => {
  const data = event.data;
  if (data instanceof ArrayBuffer) {
    const text = new TextDecoder().decode(new Uint8Array(data));
    term.write(text);
  } else if (typeof data === 'string') {
    term.write(data);
  }
};

document.addEventListener('keydown', (e) => {
  // Ignore modifier keys
  if (e.key.length === 1 || e.key === 'Enter' || e.key === 'Backspace') {
    ws.send(e.key === 'Enter' ? '\n' : e.key);
    e.preventDefault();
  }
});

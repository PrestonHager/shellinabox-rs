const ws = new WebSocket(`ws://${location.host}/ws`);
const term = document.createElement('pre');
term.style.whiteSpace = 'pre-wrap';
document.body.appendChild(term);

ws.binaryType = 'arraybuffer';
ws.onmessage = (event) => {
  const data = event.data;
  if (data instanceof ArrayBuffer) {
    const text = new TextDecoder().decode(new Uint8Array(data));
    term.textContent += text;
  } else if (typeof data === 'string') {
    term.textContent += data;
  }
};

document.addEventListener('keydown', (e) => {
  // Ignore modifier keys
  if (e.key.length === 1 || e.key === 'Enter' || e.key === 'Backspace') {
    ws.send(e.key === 'Enter' ? '\n' : e.key);
    e.preventDefault();
  }
});

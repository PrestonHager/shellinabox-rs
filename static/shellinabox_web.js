const ws = new WebSocket(`ws://${location.host}/ws`);
const canvas = document.createElement('canvas');
canvas.width = 800;
canvas.height = 400;
document.body.appendChild(canvas);

// Minimal canvas-based terminal renderer acting as a Sugarloaf stand-in
class Sugarloaf {
  constructor(canvas) {
    this.canvas = canvas;
    this.ctx = canvas.getContext('2d');
    this.ctx.font = '16px monospace';
    this.lineHeight = 18;
    this.x = 0;
    this.y = this.lineHeight;
  }

  write(text) {
    for (const ch of text) {
      if (ch === '\n') {
        this.x = 0;
        this.y += this.lineHeight;
        continue;
      }
      this.ctx.fillText(ch, this.x, this.y);
      this.x += this.ctx.measureText(ch).width;
    }
  }
}

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

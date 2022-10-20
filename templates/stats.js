let ctx = document.getElementById("cpu-stats").getContext("2d");
let ctx2 = document.getElementById("memory-stats").getContext("2d");

function createCanv(r) {
  r.font = "2rem SF Mono";
  r.lineWidth = 17;
  r.fontWeight = 700;
  r.fillStyle = "#2f2768";
  r.strokeStyle = "#E8C01A";
  r.textAlign = "center";
}

createCanv(ctx);
createCanv(ctx2);

let start = 0;
let cw = ctx.canvas.width;
let ch = ctx.canvas.height;

function setProgress(diff) {
  ctx.clearRect(0, 0, cw, ch);
  ctx.fillText(diff.toFixed(0) + "%", cw * 0.52, ch * 0.5 + 5, cw + 12);
  ctx.beginPath();
  ctx.arc(100, 100, 75, start, (Math.PI * diff) / 50);
  ctx.stroke();
}

let start2 = 0;
let cw1 = ctx2.canvas.width;
let ch1 = ctx2.canvas.height;

// todo: single function
function setMemProgress(diff) {
  ctx2.clearRect(0, 0, cw1, ch1);
  ctx2.fillText(diff.toFixed(0) + "%", cw1 * 0.52, ch1 * 0.5 + 5, cw1 + 12);
  ctx2.beginPath();
  ctx2.arc(100, 100, 75, start2, (Math.PI * diff) / 50);
  ctx2.stroke();
}

const units = ["bytes", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];

function slowBytes(x) {
  let l = 0;
  let n = parseInt(x, 10) || 0;

  while (n >= 1024 && ++l) {
    n = n / 1024;
  }

  return n.toFixed(n < 10 && l > 0 ? 1 : 0) + " " + units[l];
}

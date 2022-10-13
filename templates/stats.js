let ctx = document.getElementById("cpu-stats").getContext("2d");
let al = 0;
let start = 0;
let cw = ctx.canvas.width;
let ch = ctx.canvas.height;

ctx.font = "2rem system-ui";
ctx.lineWidth = 17;
ctx.fillStyle = "#4285f4";
ctx.strokeStyle = "#4285f4";
ctx.textAlign = "center";

function setProgress(diff) {
  ctx.clearRect(0, 0, cw, ch);
  ctx.fillText(diff.toFixed(0) + "%", cw * 0.52, ch * 0.5 + 5, cw + 12);
  ctx.beginPath();
  ctx.arc(100, 100, 75, start, (Math.PI * diff) / 50);
  ctx.stroke();
}

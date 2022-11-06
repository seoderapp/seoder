export let socket: WebSocket = null;

export const initSockets = () => {
  socket = new WebSocket("ws://127.0.0.1:8080");
};

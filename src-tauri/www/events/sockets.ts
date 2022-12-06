export let socket: WebSocket = null;
export let socketRuntime: WebSocket = null;

// init socket singleton for global app usage
export const initSockets = () => {
  socket = new WebSocket("ws://127.0.0.1:8080");
  socketRuntime = new WebSocket("ws://127.0.0.1:8089");
};

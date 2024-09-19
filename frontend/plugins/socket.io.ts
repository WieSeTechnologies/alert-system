// Tutorial: https://www.youtube.com/watch?v=QQ_jYgiSIxE

import io from "socket.io-client";

export default defineNuxtPlugin(() => {
  const config = useRuntimeConfig().public;

  const socket = io(config.socketUrl, {
    autoConnect: false,
  });

  return {
    provide: {
      io: socket,
    },
  };
});

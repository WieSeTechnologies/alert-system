// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: "2024-04-03",
  devtools: { enabled: true },
  ssr: false,
  runtimeConfig: {
    public: {
      socketUrl: process.env.SOCKET_URL || "http://localhost:9000",
    },
  },
  plugins: [
    // Socket should only connect on client.
    { src: "./plugins/socket.io", mode: "client" },
  ],
  postcss: {
    plugins: {
      tailwindcss: {},
      autoprefixer: {},
    },
  },
  css: ["~/assets/css/main.css"],
});

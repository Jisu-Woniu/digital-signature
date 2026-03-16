import { URL, fileURLToPath } from "node:url";

import { defineConfig } from "vite";
import icons from "unplugin-icons/vite";
import vue from "@vitejs/plugin-vue";

export default defineConfig({
  plugins: [
    vue(),
    icons({
      compiler: "vue3",
    }),
  ],

  clearScreen: false,

  server: {
    host: "127.0.0.1",
    port: 1420,
    strictPort: true,
  },

  envPrefix: ["VITE_", "TAURI_ENV_*"],
  resolve: {
    alias: {
      "@": fileURLToPath(new URL("./src", import.meta.url)),
    },
  },
  build: {
    // don't minify for debug builds
    minify: !process.env.TAURI_ENV_DEBUG ? "oxc" : false,
    // produce sourcemaps for debug builds
    sourcemap: !!process.env.TAURI_ENV_DEBUG,
  },
  define: {
    // Remove Vue Options API support for memory consumption.
    __VUE_OPTIONS_API__: false,
  },
});

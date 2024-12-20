import { URL, fileURLToPath } from "node:url";

import autoPrefixer from "autoprefixer";
import { defineConfig } from "vite";
import icons from "unplugin-icons/vite";
import postcssPresetEnv from "postcss-preset-env";
import vue from "@vitejs/plugin-vue";

export default defineConfig(async () => ({
  plugins: [vue(), icons({ compiler: "vue3" })],

  clearScreen: false,

  server: {
    host: "127.0.0.1",
    port: 1420,
    strictPort: true,
  },

  envPrefix: ["VITE_", "TAURI_ENV_*"],
  css: {
    postcss: {
      plugins: [postcssPresetEnv, autoPrefixer],
    },
  },
  resolve: {
    alias: {
      "@": fileURLToPath(new URL("./src", import.meta.url)),
    },
  },
  build: {
    // Tauri uses Edge on Windows and WebKit on macOS and Linux
    target:
      process.env.TAURI_ENV_PLATFORM === "windows" ? "edge105" : "safari13",
    // don't minify for debug builds
    minify: !process.env.TAURI_ENV_DEBUG ? "esbuild" : false,
    // produce sourcemaps for debug builds
    sourcemap: !!process.env.TAURI_ENV_DEBUG,
  },
  define: {
    // Remove Vue Options API support for memory consumption.
    __VUE_OPTIONS_API__: false,
  },
}));

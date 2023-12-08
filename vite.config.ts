import vue from "@vitejs/plugin-vue";
import autoPrefixer from "autoprefixer";
import { URL, fileURLToPath } from "node:url";
import postcssPresetEnv from "postcss-preset-env";
import icons from "unplugin-icons/vite";
import { defineConfig } from "vite";

export default defineConfig(async () => ({
  plugins: [vue(), icons({ compiler: "vue3" })],

  clearScreen: false,

  server: {
    host: "127.0.0.1",
    port: 1420,
    strictPort: true,
  },

  envPrefix: [
    "VITE_",
    "TAURI_ARCH",
    "TAURI_FAMILY",
    "TAURI_PLATFORM",
    "TAURI_PLATFORM_TYPE",
    "TAURI_PLATFORM_VERSION",
    "TAURI_DEBUG",
  ],
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
    target: process.env.TAURI_PLATFORM == "windows" ? "edge105" : "safari13",
    // produce sourcemaps for debug builds
    sourcemap: !!process.env.TAURI_DEBUG,
  },
  define: {
    // Remove Vue Options API support for memory consumption.
    __VUE_OPTIONS_API__: false,
  },
}));

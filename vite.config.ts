import { defineConfig } from "vite-plus";
import icons from "unplugin-icons/vite";
import vue from "@vitejs/plugin-vue";

export default defineConfig({
  staged: {
    "*": "vp check --fix",
  },
  lint: {
    plugins: ["eslint", "oxc", "typescript", "unicorn", "vue"],
    categories: {
      correctness: "error",
      suspicious: "warn",
    },
    options: {
      typeAware: true,
      // Type checking should use vue-tsc to support Vue SFCs, but oxlint doesn't support it currently.
      // Disable type checking now.
      // typeCheck: true,
    },
    env: {
      builtin: true,
    },
  },
  fmt: {
    ignorePatterns: ["pnpm-lock.yaml"],
  },
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
      "@": "./src",
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

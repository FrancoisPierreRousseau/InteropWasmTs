import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

export default defineConfig({
  plugins: [wasm(), topLevelAwait()],
  build: {
    watch: {
      include: ["src/**/*.ts"],
    },
    rollupOptions: {
      input: {
        main: "./src/index.html",
        app: "./src/index.ts",
      },
    },
    outDir: "./dist",
  },
  optimizeDeps: {
    exclude: ["@syntect/wasm"],
  },
});

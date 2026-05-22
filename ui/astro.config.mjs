import { defineConfig } from "astro/config";

// Tauri expects a static frontend bundled at ../src-tauri/.../ui/dist
// (configured in tauri.conf.json).
export default defineConfig({
  output: "static",
  build: {
    format: "directory",
  },
  server: {
    port: 4321,
    host: "127.0.0.1",
  },
  vite: {
    clearScreen: false,
    server: {
      strictPort: true,
    },
  },
});

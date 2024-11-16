
import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";


// @ts-expect-error process is a nodejs global
const mobile = !!/android|ios/.exec(process.env.TAURI_ENV_PLATFORM);

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [vue()],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
      port: 5173,
      strictPort: true,
      host: mobile ? "0.0.0.0" : false,
      // hmr: mobile
      //   ? {
      //       protocol: "ws",
      //       host: await internalIpV4(),
      //       port: 1421,
      //     }
      //   : undefined,
      watch: {
          // 3. tell vite to ignore watching `src-tauri`
          ignored: ["**/src-tauri/**"],
      },
  },
    // to access the Tauri environment variables set by the CLI with information about the current target
    envPrefix: ['VITE_', 'TAURI_ENV_*'],
    build: {
        // Tauri uses Chromium on Windows and WebKit on macOS and Linux
        target:
            process.env.TAURI_ENV_PLATFORM == 'windows'
                ? 'chrome105'
                : 'safari13',
        // don't minify for debug builds
        minify: !process.env.TAURI_ENV_DEBUG ? 'esbuild' : false,
        // produce sourcemaps for debug builds
        sourcemap: !!process.env.TAURI_ENV_DEBUG,
    },
}));
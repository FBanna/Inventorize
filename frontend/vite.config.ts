import { fileURLToPath, resolve, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueDevTools from 'vite-plugin-vue-devtools'

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    vueDevTools(),
  ],
  build: {
    outDir: "../target_dist",
    emptyOutDir: true,
    rolldownOptions: {
      input: {
        main: resolve(__dirname, "index.html"),
        login: resolve(__dirname, "login_src/index.html") // go to "/login/" last slash important!!!!
      }
    }
  },
  // resolve: {
  //   alias: {
  //     '@': fileURLToPath(new URL('./src', import.meta.url)),
  //   },
  // },
})

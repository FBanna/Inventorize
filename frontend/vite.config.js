import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueDevTools from 'vite-plugin-vue-devtools'
import { dirname, resolve } from 'node:path'



const __dirname = dirname(fileURLToPath(import.meta.url))
// https://vite.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    vueDevTools(),
  ],
  build: {
    chunkSizeWarningLimit: 1600,
    outDir: "../target/dist",
    emptyOutDir: true,
    rollupOptions: {
      input: {
        main: resolve(__dirname, "index.html"),
        login: resolve(__dirname, "login/index.html") // go to "/login/" last slash important!!!!
      }
    }
  },
  // resolve: {
  //   alias: {
  //     '@': path.resolve(__dirname, "./src")
  //   }
    
  //   // {
  //   //   '@': fileURLToPath(new URL('src', import.meta.url)),
  //   //   //'@login': fileURLToPath(new URL('./login-src', import.meta.url))
  //   // }
  // },
})

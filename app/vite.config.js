import { resolve } from 'path'
import { defineConfig } from 'vite'
import solid from 'vite-plugin-solid'

export default defineConfig({
  plugins: [solid()],

  build: {
    rollupOptions: {
      input: {
        main: resolve(__dirname, 'index.html'),
        p1: resolve(__dirname, 'file_page/index.html'),
      },
    }
  },

})

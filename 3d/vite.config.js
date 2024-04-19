import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import fs from 'fs'
import Checker from 'vite-plugin-checker'
import cheerio from 'cheerio'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
      react(),
      Checker()
      // myPlugin()
  ],
  publicDir: 'assets-3d',
  build: {
    outDir: '../target/site/3d-dist',
    assetsDir: 'assets-3d',
    assetsInlineLimit: 0,
    manifest: false,
    rollupOptions: {
      output: {
        entryFileNames: '[name].js',
        chunkFileNames: '[name].js',
        assetFileNames: '[name].[ext]'
      }
    }
  }
})

function myPlugin()  {
  return {
    name: 'my-plugin',
    enforce: 'post',
    apply: 'build',
    buildStart: async () => {
      fs.unlinkSync('./build-report.json')
    },
    closeBundle: async(config) => {
      const html = await fs.readFileSync('../target/site/3d-dist/index.html')
      const $ = cheerio.load(html);
      const scriptSrc = $('script').attr('src');
      fs.writeFileSync('./build-report.json', JSON.stringify({
        js_file: scriptSrc
      }))
    },
  };
}
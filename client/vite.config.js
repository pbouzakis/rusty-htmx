import { babel } from '@rollup/plugin-babel'

export default {
  root: './src',
  publicDir: './images',
  build: {
    manifest: true,
     rollupOptions: {
      // overwrite default .html entry
      input: './src/layout.html',
    },
    //outDir: '../public',
    outDir: '../../templates/_CLIENT_',
    emptyOutDir: true,
  },
  plugins: [
    babel({
      exclude: /node_modules/,
    }),
  ],
}

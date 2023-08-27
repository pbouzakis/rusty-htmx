import { babel } from '@rollup/plugin-babel'

export default {
  root: './src',
  build: {
    manifest: true,
    //outDir: '../public',
    rollupOptions: {
      // overwrite default .html entry
      input: './src/layout.html',
    },
    outDir: '../../templates/_CLIENT_',
    emptyOutDir: true,
  },
  plugins: [
    babel({
      exclude: /node_modules/,
    }),
  ],
}

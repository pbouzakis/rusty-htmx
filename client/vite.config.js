import { babel } from '@rollup/plugin-babel'

export default {
  root: './src',
  publicDir: './images',
  build: {
    manifest: true,
    rollupOptions: {
      input: './src/index.html',
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

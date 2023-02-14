const dev = process.env.NODE_ENV === "development";

/** @type {import('vite').UserConfig} */
export default {
  build: {
    sourcemap: dev,
    rollupOptions: {
      output: {
        entryFileNames: "[name].js",
        chunkFileNames: "[name].js",
        assetFileNames: "[name].[ext]",
      },
    },
  },
  css: {
    devSourcemap: dev,
  },
};

import { sveltekit } from "@sveltejs/kit/vite";
import path from "path";
import { FileSystemIconLoader } from "unplugin-icons/loaders";
import Icons from "unplugin-icons/vite";
import { SvelteKitPWA } from "@vite-pwa/sveltekit";
import type { UserConfig } from "vite";

const config: UserConfig = {
  resolve: {
    alias: {
      $app: path.resolve("./src/routes/(app)"),
      $auth: path.resolve("./src/routes/(auth)"),
      $routes: path.resolve("./src/routes"),
    },
  },
  plugins: [
    sveltekit(),
    SvelteKitPWA({
      manifest: {
        name: "Playa",
        short_name: "Playa",
        background_color: "#fff",
        theme_color: "#448aff",
        start_url: "/",
        display: "standalone",
        prefer_related_applications: true,
        icons: [
          {
            src: "./static/maskable_icon.png",
            sizes: "196x196",
            type: "image/png",
            purpose: "any maskable",
          },
          {
            src: "/android-chrome-192x192.png",
            type: "image/png",
            sizes: "192x192",
          },
          {
            src: "./static/android-chrome-512x512.png",
            type: "image/png",
            sizes: "512x512",
          },
        ],
      },
    }),
    Icons({
      compiler: "svelte",
      customCollections: {
        custom: FileSystemIconLoader("./static/icons"),
      },
    }),
  ],
};

export default config;

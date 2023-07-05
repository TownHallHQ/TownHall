import { sveltekit } from '@sveltejs/kit/vite';
import path from 'path';
import { FileSystemIconLoader } from 'unplugin-icons/loaders';
import Icons from 'unplugin-icons/vite';

import type { UserConfig } from 'vite';

const config: UserConfig = {
  resolve: {
    alias: {
      $app: path.resolve('./src/routes/(app)'),
      $auth: path.resolve('./src/routes/(auth)'),
      $routes: path.resolve('./src/routes')
    }
  },
  plugins: [
    sveltekit(),
    Icons({
      compiler: 'svelte',
      customCollections: {
        custom: FileSystemIconLoader('./static/icons')
      }
    })
  ]
};

export default config;

import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import { execSync } from 'node:child_process';

function getGitMetadata() {
  try {
    const commitHash = execSync('git rev-parse --short HEAD').toString().trim();
    const buildNumber = execSync('git rev-list --count HEAD').toString().trim();
    return { commitHash, buildNumber };
  } catch {
    return {
      commitHash: process.env.VITE_COMMIT_HASH || 'dev',
      buildNumber: process.env.VITE_BUILD_NUMBER || '0',
    };
  }
}

const { commitHash, buildNumber } = getGitMetadata();
const buildTimestamp = process.env.VITE_BUILD_TIMESTAMP || new Date().toISOString();
const appVersion = process.env.npm_package_version || '0.0.1';

export default defineConfig({
  plugins: [svelte()],
  define: {
    __APP_VERSION__: JSON.stringify(appVersion),
    __BUILD_NUMBER__: JSON.stringify(buildNumber),
    __COMMIT_HASH__: JSON.stringify(commitHash),
    __BUILD_TIMESTAMP__: JSON.stringify(buildTimestamp),
  },
  server: {
    port: 5173,
    proxy: {
      '/api': {
        target: 'http://localhost:8080',
        changeOrigin: true,
      },
      '/health': {
        target: 'http://localhost:8080',
        changeOrigin: true,
      },
    },
  },
  build: {
    outDir: 'dist',
    emptyOutDir: true,
    rollupOptions: {
      output: {
        assetFileNames: (assetInfo) => {
          if (assetInfo.name && assetInfo.name.endsWith('.css')) {
            return 'assets/app.css';
          }
          return 'assets/[name]-[hash][extname]';
        },
      },
    },
  },
});

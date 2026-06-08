import { defineConfig } from 'vite';
import { sveltekit } from '@sveltejs/kit/vite';
import tailwindcss from '@tailwindcss/vite';

export default defineConfig({
  plugins: [tailwindcss(), sveltekit()],
  resolve: {
    extensions: ['.svelte.ts', '.svelte', '.ts', '.js', '.mjs'],
  },
  server: {
    port: 5173,
    strictPort: true,
  },
});

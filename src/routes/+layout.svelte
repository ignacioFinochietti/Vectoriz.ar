<script lang="ts">
  let { children } = $props();
</script>

<header class="h-14 border-b border-zinc-800/60 bg-zinc-950/80 backdrop-blur-sm sticky top-0 z-50 px-6 flex items-center justify-between">
  <div class="flex items-center gap-3">
    <div class="w-8 h-8 rounded-lg bg-gradient-to-br from-violet-500 to-indigo-600 flex items-center justify-center shadow-lg shadow-violet-500/20">
      <svg class="w-5 h-5 text-white" fill="currentColor" viewBox="0 0 24 24">
        <path d="M11.5 22C11.17 22 10.85 21.83 10.67 21.52L6.17 13.52C5.92 13.07 6.04 12.5 6.44 12.21C6.84 11.92 7.41 12.04 7.7 12.44L11.5 19.2L16.3 10.8C16.59 10.4 17.16 10.28 17.56 10.57C17.96 10.86 18.08 11.43 17.79 11.83L12.33 21.52C12.15 21.83 11.83 22 11.5 22ZM12.5 13.5H7C6.45 13.5 6 13.05 6 12.5C6 11.95 6.45 11.5 7 11.5H11.5V3C11.5 2.45 11.95 2 12.5 2C13.05 2 13.5 2.45 13.5 3V11.5H18C18.55 11.5 19 11.95 19 12.5C19 13.05 18.55 13.5 18 13.5H12.5Z"/>
      </svg>
    </div>
    <span class="font-bold text-lg tracking-tight">Vectoriz<span class="text-violet-400 font-medium">.ar</span></span>
  </div>
  <div class="flex items-center gap-3">
    <span class="px-2 py-0.5 rounded border border-zinc-700/60 text-[10px] uppercase font-bold tracking-widest text-zinc-400 bg-zinc-900/50">Desktop App</span>
    <div class="w-2.5 h-2.5 rounded-full bg-emerald-500 animate-pulse" title="Motor local activo"></div>
  </div>
</header>

<main class="flex-1 max-w-7xl w-full mx-auto px-6 py-8 flex flex-col justify-center">
  {#if children}
    {@render children()}
  {/if}
</main>

<footer class="py-4 border-t border-zinc-800/40 bg-zinc-950 px-6 flex flex-col md:flex-row justify-between items-center text-[11px] text-zinc-500 gap-2">
  <p>&copy; 2026 Vectoriz.ar Desktop Studio. Todos los derechos reservados.</p>
  <div class="flex items-center gap-4">
    <span>Algoritmo de Trazado: VTracer O(n) Core</span>
    <span class="text-zinc-700">|</span>
    <span>Motor de Binarizaci&oacute;n Morfol&oacute;gica Local</span>
  </div>
</footer>

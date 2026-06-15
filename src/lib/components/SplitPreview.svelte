<script lang="ts">
  import { untrack } from 'svelte';
  import { vectorStore } from '$lib/stores/vectorStore';
  import { imageStore } from '$lib/stores/imageStore';

  let viewMode = $state<'side-by-side' | 'overlay'>('side-by-side');
  let overlayOpacity = $state(0.6);
  let showCheckerboard = $state(true);

  let svgCode = $derived(vectorStore.svgResult);
  let svgBlobUrl = $state<string | null>(null);

  $effect(() => {
    const code = svgCode;
    if (code) {
      svgBlobUrl = URL.createObjectURL(new Blob([code], { type: 'image/svg+xml' }));
    } else {
      svgBlobUrl = null;
    }
    return () => {
      const url = untrack(() => svgBlobUrl);
      if (url) URL.revokeObjectURL(url);
    };
  });



  let processingTime = $derived(vectorStore.processingTimeMs);
</script>

<!-- ═══════════════════ TOOLBAR ═══════════════════ -->
<div class="flex items-center justify-between mb-2 gap-2">
  <div class="flex items-center gap-0.5 bg-zinc-900 border border-zinc-700/50 rounded-lg p-0.5">
    <button
      onclick={() => viewMode = 'side-by-side'}
      class="px-3 py-1.5 rounded text-[10px] font-medium transition-colors flex items-center gap-1.5 {viewMode === 'side-by-side' ? 'bg-zinc-800 text-zinc-300' : 'text-zinc-500 hover:text-zinc-300'}"
      title="Vista lado a lado"
    >
      <svg class="w-3 h-3" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M9 4v16M3 8h4m-4 4h4m-4 4h4M21 8h-4m4 4h-4m4 4h-4"/></svg>
      Comparar
    </button>
    <button
      onclick={() => viewMode = 'overlay'}
      class="px-3 py-1.5 rounded text-[10px] font-medium transition-colors flex items-center gap-1.5 {viewMode === 'overlay' ? 'bg-zinc-800 text-zinc-300' : 'text-zinc-500 hover:text-zinc-300'}"
      title="Superponer capas"
    >
      <svg class="w-3 h-3" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M8 3H5a2 2 0 00-2 2v3m18 0V5a2 2 0 00-2-2h-3m0 18h3a2 2 0 002-2v-3M3 16v3a2 2 0 002 2h3"/></svg>
      Superponer
    </button>
  </div>

  <div class="flex items-center gap-3">
    {#if processingTime > 0}
      <span class="text-[10px] text-zinc-600 font-mono tabular-nums">{processingTime} ms</span>
    {/if}
    <button
      onclick={() => showCheckerboard = !showCheckerboard}
      class="text-[10px] px-2 py-1 rounded transition-colors flex items-center gap-1 border {showCheckerboard ? 'bg-sky-500/10 text-sky-400 border-sky-500/20' : 'bg-zinc-900 text-zinc-500 border-zinc-700/50'}"
      title="Alternar fondo cuadriculado"
    >
      <svg class="w-3 h-3" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M4 4h16v16H4z M4 12h16 M12 4v16"/></svg>
      Grid
    </button>
  </div>
</div>

<!-- ═══════════════════ SIDE-BY-SIDE MODE ═══════════════════ -->
{#if viewMode === 'side-by-side'}
  <div class="grid grid-cols-2 gap-3 h-full">
    <!-- Left: Raster -->
    <div class="flex flex-col min-h-0">
      <div class="flex items-center justify-between mb-1.5">
        <span class="text-[10px] font-medium uppercase tracking-wider text-zinc-500">Original</span>
        <span class="text-[9px] text-zinc-600 px-1.5 py-0.5 rounded bg-zinc-900 border border-zinc-800/50">Raster</span>
      </div>
      <div class="flex-1 rounded-lg border border-zinc-800/60 bg-zinc-900/40 flex items-center justify-center overflow-hidden min-h-[200px]">
        {#if imageStore.fileInfo?.url}
          <img src={imageStore.fileInfo.url} alt="Original raster" class="max-w-full max-h-full object-contain" />
        {:else}
          <span class="text-zinc-600 text-[11px]">Sin imagen</span>
        {/if}
      </div>
    </div>

    <!-- Right: SVG -->
    <div class="flex flex-col min-h-0">
      <div class="flex items-center justify-between mb-1.5">
        <span class="text-[10px] font-medium uppercase tracking-wider text-zinc-500">Vectorizado</span>
        <span class="text-[9px] px-1.5 py-0.5 rounded bg-sky-500/10 text-sky-400 border border-sky-500/20">SVG</span>
      </div>
      <div
        class="flex-1 rounded-lg border flex items-center justify-center overflow-hidden min-h-[200px] {showCheckerboard ? 'checkerboard border-sky-500/20' : 'border-zinc-800/60 bg-zinc-900/40'}"
      >
        {#if svgBlobUrl}
          <img src={svgBlobUrl} alt="SVG Preview" class="max-w-full max-h-full object-contain" />
        {:else}
          <div class="flex flex-col items-center gap-2 text-zinc-600">
            <svg class="w-6 h-6" fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M13 10V3L4 14h7v7l9-11h-7z"/></svg>
            <span class="text-[11px]">Vectorizar para ver resultado</span>
          </div>
        {/if}
      </div>
    </div>
  </div>

<!-- ═══════════════════ OVERLAY MODE ═══════════════════ -->
{:else}
  <div class="flex flex-col h-full gap-2">
    <!-- Canvas container -->
    <div
      class="flex-1 relative rounded-lg border overflow-hidden min-h-[200px] {showCheckerboard ? 'checkerboard border-sky-500/20' : 'border-zinc-800/60 bg-zinc-900/40'}"
    >
      <!-- Raster layer -->
      {#if imageStore.fileInfo?.url}
        <img src={imageStore.fileInfo.url} alt="Original" class="absolute inset-0 w-full h-full object-contain" />
      {/if}

      <!-- SVG overlay layer -->
      {#if svgBlobUrl}
        <div class="absolute inset-0 transition-opacity duration-100" style="opacity: {overlayOpacity}">
          <img src={svgBlobUrl} alt="SVG Overlay" class="w-full h-full object-contain" />
        </div>
      {/if}

      <!-- Empty state when no image -->
      {#if !imageStore.fileInfo?.url}
        <div class="absolute inset-0 flex items-center justify-center">
          <span class="text-zinc-600 text-[11px]">Sin imagen</span>
        </div>
      {/if}

      <!-- Opacity slider (bottom bar) -->
      {#if svgBlobUrl}
        <div class="absolute bottom-3 left-3 right-3 flex items-center gap-3 bg-zinc-950/90 backdrop-blur-sm border border-zinc-700/50 rounded-lg px-3 py-2">
          <span class="text-[10px] font-medium text-zinc-400 uppercase tracking-wider shrink-0">Opacidad</span>
          <input
            type="range"
            min="0"
            max="1"
            step="0.01"
            value={overlayOpacity}
            oninput={(e) => overlayOpacity = parseFloat((e.target as HTMLInputElement).value)}
            class="flex-1 h-1 rounded-full appearance-none cursor-pointer bg-zinc-700 accent-sky-500
              [&::-webkit-slider-thumb]:appearance-none
              [&::-webkit-slider-thumb]:w-3
              [&::-webkit-slider-thumb]:h-3
              [&::-webkit-slider-thumb]:rounded-full
              [&::-webkit-slider-thumb]:bg-sky-500
              [&::-webkit-slider-thumb]:shadow-md
              [&::-webkit-slider-thumb]:shadow-sky-500/20"
          />
          <span class="text-[10px] font-mono text-sky-400 tabular-nums w-8 text-right">{Math.round(overlayOpacity * 100)}%</span>
        </div>
      {/if}
    </div>

    <!-- Layer legend -->
    <div class="flex items-center gap-4 px-1">
      <div class="flex items-center gap-1.5">
        <div class="w-2.5 h-2.5 rounded-sm border border-zinc-600 bg-zinc-800"></div>
        <span class="text-[10px] text-zinc-500">Raster</span>
      </div>
      <div class="flex items-center gap-1.5">
        <div class="w-2.5 h-2.5 rounded-sm border border-sky-500/50 bg-sky-500/30"></div>
        <span class="text-[10px] text-zinc-500">SVG ({Math.round(overlayOpacity * 100)}%)</span>
      </div>
    </div>
  </div>
{/if}

<script lang="ts">
  import { batchStore } from '$lib/stores/batchStore';
  import { vectorStore } from '$lib/stores/vectorStore';
  import { imageStore } from '$lib/stores/imageStore';

  let queue = $derived(batchStore.queue);
  let results = $derived(batchStore.results);
  let processing = $derived(batchStore.isProcessing);
  let hasQueue = $derived(batchStore.hasQueue);
  let hasResults = $derived(batchStore.hasResults);
  let viewSvg = $state<string | null>(null);

  function addCurrent() {
    const img = imageStore.currentImage;
    if (!img) return;
    const id = crypto.randomUUID();
    batchStore.add({
      id,
      data_base64: img.data_base64,
      name: img.name,
      url: imageStore.originalUrl || imageStore.fileInfo?.url || '',
      params: {
        mode: vectorStore.params.mode,
        threshold: vectorStore.params.threshold,
        blur_radius: vectorStore.params.blur_radius,
        despeckle: vectorStore.params.despeckle,
        sparsity: vectorStore.params.sparsity,
        node_optimization: vectorStore.params.node_optimization,
      },
    });
  }

  function getResult(id: string) {
    return results.find(r => r.id === id);
  }

  function downloadSvg(svg: string, name: string) {
    const blob = new Blob([svg], { type: 'image/svg+xml' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = name.replace(/\.[^/.]+$/, '') + '_vectorizado.svg';
    a.click();
    URL.revokeObjectURL(url);
  }
</script>

<div class="space-y-4">
  <div class="flex items-center justify-between">
    <div class="flex items-center gap-2">
      {#if hasQueue}
        <span class="text-[10px] font-semibold text-zinc-500 uppercase tracking-wider">Cola</span>
        <span class="px-1.5 py-0.5 rounded text-[9px] bg-sky-500/15 text-sky-400">{queue.length}</span>
      {/if}
    </div>
    <div class="flex items-center gap-2">
      {#if !!imageStore.currentImage}
        <button
          onclick={addCurrent}
          disabled={processing}
          class="text-[10px] px-2 py-1 rounded border border-sky-500/30 text-sky-400 hover:bg-sky-500/10 transition-colors disabled:opacity-30 flex items-center gap-1"
        >
          <svg class="w-3 h-3" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" d="M12 4v16m8-8H4"/>
          </svg>
          Agregar actual
        </button>
      {/if}
      {#if hasQueue}
        <button
          onclick={() => batchStore.processAll()}
          disabled={processing}
          class="text-[10px] px-3 py-1.5 rounded bg-gradient-to-r from-sky-600 to-teal-600 text-white font-semibold hover:from-sky-500 hover:to-teal-500 transition-all disabled:opacity-40 flex items-center gap-1.5"
        >
          {#if processing}
            <svg class="w-3 h-3 animate-spin" fill="none" viewBox="0 0 24 24">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"/>
              <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"/>
            </svg>
            Procesando...
          {:else}
            <svg class="w-3 h-3" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" d="M13 10V3L4 14h7v7l9-11h-7z"/>
            </svg>
            Vectorizar Todo
          {/if}
        </button>
      {/if}
      {#if hasQueue || hasResults}
        <button
          onclick={() => batchStore.clear()}
          disabled={processing}
          class="text-[10px] px-2 py-1 rounded text-zinc-500 hover:text-red-400 hover:bg-red-500/10 transition-colors disabled:opacity-30"
        >
          Limpiar
        </button>
      {/if}
    </div>
  </div>

  {#if hasQueue}
    <div class="space-y-1.5">
      {#each queue as entry}
        <div class="flex items-center gap-2 p-2 rounded-lg bg-zinc-900/60 border border-zinc-800/40">
          <img src={entry.url} alt={entry.name} class="w-8 h-8 rounded object-cover border border-zinc-700/50 bg-zinc-800 shrink-0" />
          <div class="flex-1 min-w-0">
            <p class="text-[11px] text-zinc-300 truncate">{entry.name}</p>
            <p class="text-[9px] text-zinc-500 capitalize">{entry.params.mode} &middot; th={entry.params.threshold} &middot; bl={entry.params.blur_radius}</p>
          </div>
          <button
            onclick={() => batchStore.remove(entry.id)}
            disabled={processing}
            title="Quitar de la cola"
            class="text-zinc-600 hover:text-red-400 p-1 disabled:opacity-30 transition-colors"
          >
            <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"/>
            </svg>
          </button>
        </div>
      {/each}
    </div>
  {/if}

  {#if hasResults}
    <div class="space-y-1.5">
      <p class="text-[10px] uppercase font-semibold text-zinc-500 tracking-wider">Resultados</p>
      {#each results as res}
        {@const entry = queue.find(e => e.id === res.id)}
        <div class="flex items-center gap-2 p-2 rounded-lg bg-zinc-900/60 border border-zinc-800/40">
          {#if entry}
            <img src={entry.url} alt={res.name} class="w-8 h-8 rounded object-cover border border-zinc-700/50 bg-zinc-800 shrink-0" />
          {/if}
          <div class="flex-1 min-w-0">
            <p class="text-[11px] text-zinc-300 truncate">{res.name}</p>
            {#if res.error}
              <p class="text-[9px] text-red-400">{res.error}</p>
            {:else}
              <p class="text-[9px] text-emerald-400">Vectorizado</p>
            {/if}
          </div>
          {#if res.svg}
            <div class="flex items-center gap-1">
              <button
                onclick={() => { viewSvg = res.svg; }}
                class="text-[9px] px-1.5 py-0.5 rounded border border-zinc-700/50 text-zinc-400 hover:text-sky-400 hover:border-sky-500/30 transition-colors"
              >
                Ver
              </button>
              <button
                onclick={() => downloadSvg(res.svg!, res.name)}
                class="text-[9px] px-1.5 py-0.5 rounded border border-zinc-700/50 text-zinc-400 hover:text-emerald-400 hover:border-emerald-500/30 transition-colors"
              >
                Desc.
              </button>
            </div>
          {/if}
        </div>
      {/each}
    </div>
  {/if}

  {#if viewSvg}
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="fixed inset-0 z-50 bg-zinc-950/90 backdrop-blur-sm flex items-center justify-center p-8" role="dialog" tabindex="-1" onclick={() => { viewSvg = null; }} onkeydown={(e) => { if (e.key === 'Escape') viewSvg = null; }}>
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div class="bg-zinc-900 border border-zinc-700/50 rounded-xl p-4 max-w-2xl w-full max-h-[80vh] flex flex-col gap-3" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()}>
        <div class="flex items-center justify-between">
          <span class="text-xs font-semibold text-zinc-300">C&oacute;digo SVG</span>
          <button onclick={() => { viewSvg = null; }} title="Cerrar" class="text-zinc-500 hover:text-zinc-300">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"/>
            </svg>
          </button>
        </div>
        <div class="flex-1 overflow-auto checkerboard rounded-lg border border-zinc-700/30 p-4">
          <div class="text-zinc-100" style="white-space: pre-wrap; font-family: monospace; font-size: 11px;">
            {@html viewSvg}
          </div>
        </div>
        <pre class="text-[10px] text-zinc-400 bg-zinc-950 border border-zinc-800 rounded p-3 max-h-48 overflow-auto"><code>{viewSvg}</code></pre>
      </div>
    </div>
  {/if}
</div>

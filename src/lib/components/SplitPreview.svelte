<script lang="ts">
  import { vectorStore } from '$lib/stores/vectorStore';

  let svgCode = $derived(vectorStore.svgResult);
  let svgBlobUrl = $state<string | null>(null);
  let showCheckerboard = $state(true);

  $effect(() => {
    if (svgCode && svgBlobUrl) URL.revokeObjectURL(svgBlobUrl);
    if (svgCode) {
      svgBlobUrl = URL.createObjectURL(new Blob([svgCode], { type: 'image/svg+xml' }));
    } else {
      svgBlobUrl = null;
    }
    return () => { if (svgBlobUrl) URL.revokeObjectURL(svgBlobUrl); };
  });

  let imgSrc = $state<string | null>(null);
  let stamp = $state(0);

  import { imageStore } from '$lib/stores/imageStore';

  $effect(() => {
    const info = imageStore.fileInfo;
    if (info) {
      imgSrc = info.url;
      stamp++;
    }
  });

  function cbClass(): string {
    let cls = 'flex-1 rounded-lg border overflow-hidden flex items-center justify-center min-h-[200px]';
    if (showCheckerboard) {
      cls += ' checkerboard border-violet-500/30';
    } else {
      cls += ' border-zinc-800/60 bg-zinc-900/40';
    }
    return cls;
  }

  function badgeClass(): string {
    let cls = 'text-[9px] uppercase font-bold tracking-wider px-1.5 py-0.5 rounded transition-all';
    if (showCheckerboard) {
      cls += ' bg-violet-500/20 text-violet-400';
    } else {
      cls += ' bg-zinc-800/60 text-zinc-500';
    }
    return cls;
  }
</script>

<div class="grid grid-cols-2 gap-2 h-full">
  <div class="flex flex-col">
    <div class="flex items-center justify-between mb-1.5">
      <h4 class="text-[11px] font-medium text-zinc-500 tracking-wide">Imagen Original</h4>
      <span class="text-[9px] uppercase font-bold tracking-wider text-zinc-600 bg-zinc-800/60 px-1.5 py-0.5 rounded">Mapa de bits</span>
    </div>
    <div class="flex-1 rounded-lg border border-zinc-800/60 overflow-hidden bg-zinc-900/40 flex items-center justify-center min-h-[200px]">
      {#if imgSrc}
        <img key={stamp} src={imgSrc} alt="original" class="max-w-full max-h-full object-contain" />
      {:else}
        <span class="text-zinc-600 text-xs">Sin imagen</span>
      {/if}
    </div>
  </div>

  <div class="flex flex-col">
    <div class="flex items-center justify-between mb-1.5">
      <h4 class="text-[11px] font-medium text-zinc-500 tracking-wide">SVG Vectorizado</h4>
      <button
        onclick={() => showCheckerboard = !showCheckerboard}
        class={badgeClass()}
      >
        Checkerboard
      </button>
    </div>
    <div
      class={cbClass()}
    >
      {#if svgBlobUrl}
        <img src={svgBlobUrl} alt="SVG Preview" class="max-w-full max-h-full object-contain" />
      {:else}
        <div class="flex flex-col items-center gap-2 text-zinc-600">
          <svg class="w-8 h-8" fill="none" stroke="currentColor" stroke-width="1" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" d="M8 16l4 4 4-4M12 12V4"/>
          </svg>
          <span class="text-xs">Vectoriza para ver resultado</span>
        </div>
      {/if}
    </div>
  </div>
</div>

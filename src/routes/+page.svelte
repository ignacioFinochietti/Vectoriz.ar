<script lang="ts">
  import { imageStore } from '$lib/stores/imageStore';
  import { vectorStore } from '$lib/stores/vectorStore';
  import DropZone from '$lib/components/DropZone.svelte';
  import SplitPreview from '$lib/components/SplitPreview.svelte';
  import ControlPanel from '$lib/components/ControlPanel.svelte';
  import ActionBar from '$lib/components/ActionBar.svelte';

  let hasImage = $derived(imageStore.current !== null);
  let img = $derived(imageStore.current);
  let vectorizing = $derived(vectorStore.vectorizing);
</script>

<div class="flex-1 flex flex-col">
  {#if !hasImage}
    <DropZone />
  {:else}
    <div class="flex items-center justify-between mb-4 p-3 rounded-lg bg-zinc-900/60 border border-zinc-800/40">
      <div class="flex items-center gap-3">
        {#if img?.url}
          <img src={img.url} alt={img.name} class="w-10 h-10 rounded-lg object-cover border border-zinc-700/50 bg-zinc-800" />
        {/if}
        <div>
          <p class="text-sm font-medium text-zinc-200">{img?.name ?? 'Imagen'}</p>
          <p class="text-[11px] text-zinc-500">Lista para vectorizar</p>
        </div>
      </div>
      <button
        onclick={() => { imageStore.clear(); vectorStore.clear(); }}
        disabled={vectorizing}
        class="text-xs text-zinc-500 hover:text-red-400 transition-colors disabled:opacity-30 disabled:cursor-not-allowed px-2.5 py-1.5 rounded-lg hover:bg-red-500/10"
      >
        Quitar Imagen
      </button>
    </div>

    <div class="grid grid-cols-[1fr_280px] gap-5 flex-1">
      <div class="flex flex-col gap-4">
        <SplitPreview />
        <ActionBar />
      </div>
      <div class="bg-zinc-900/60 border border-zinc-800/40 rounded-xl p-4">
        <ControlPanel />
      </div>
    </div>
  {/if}
</div>

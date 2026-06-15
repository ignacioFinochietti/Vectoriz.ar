<script lang="ts">
  import { imageStore } from '$lib/stores/imageStore';
  import { vectorStore } from '$lib/stores/vectorStore';
  import { batchStore } from '$lib/stores/batchStore';
  import DropZone from '$lib/components/DropZone.svelte';
  import SplitPreview from '$lib/components/SplitPreview.svelte';
  import ControlPanel from '$lib/components/ControlPanel.svelte';
  import ActionBar from '$lib/components/ActionBar.svelte';
  import BatchPanel from '$lib/components/BatchPanel.svelte';

  let sidebarPinned = $state(true);
  let drawerOpen = $state(false);

  let hasImage = $derived(imageStore.current !== null);
  let img = $derived(imageStore.current);
  let vectorizing = $derived(vectorStore.vectorizing || batchStore.isProcessing);
  let batchCount = $derived(batchStore.queue.length);
  let batchHasResults = $derived(batchStore.results.length > 0);

  $effect(() => {
    if (batchHasResults) drawerOpen = true;
  });
</script>

<div class="h-screen w-screen flex flex-col bg-zinc-950 text-zinc-100 overflow-hidden select-none">
  <!-- ═══════════════════════ HEADER h-12 ═══════════════════════ -->
  <header class="h-12 shrink-0 border-b border-zinc-800 bg-zinc-950 flex items-center justify-between px-4 gap-4">
    <div class="flex items-center gap-3 min-w-0">
      <div class="w-7 h-7 rounded-md bg-gradient-to-br from-sky-500 to-teal-500 flex items-center justify-center shrink-0">
        <svg class="w-4 h-4 text-white" fill="currentColor" viewBox="0 0 24 24">
          <path d="M11.5 22C11.17 22 10.85 21.83 10.67 21.52L6.17 13.52C5.92 13.07 6.04 12.5 6.44 12.21C6.84 11.92 7.41 12.04 7.7 12.44L11.5 19.2L16.3 10.8C16.59 10.4 17.16 10.28 17.56 10.57C17.96 10.86 18.08 11.43 17.79 11.83L12.33 21.52C12.15 21.83 11.83 22 11.5 22ZM12.5 13.5H7C6.45 13.5 6 13.05 6 12.5C6 11.95 6.45 11.5 7 11.5H11.5V3C11.5 2.45 11.95 2 12.5 2C13.05 2 13.5 2.45 13.5 3V11.5H18C18.55 11.5 19 11.95 19 12.5C19 13.05 18.55 13.5 18 13.5H12.5Z"/>
        </svg>
      </div>
      <span class="font-bold text-sm tracking-tight whitespace-nowrap">Vectoriz<span class="text-sky-400">.ar</span></span>
    </div>

    <div class="flex items-center gap-3 min-w-0">
      {#if hasImage}
        <div class="flex items-center gap-2 min-w-0">
          <div class="w-6 h-6 rounded overflow-hidden border border-zinc-700/50 bg-zinc-800 shrink-0">
            {#if imageStore.fileInfo?.url}
              <img src={imageStore.fileInfo.url} alt={img?.name ?? ''} class="w-full h-full object-cover" />
            {/if}
          </div>
          <span class="text-[11px] text-zinc-400 truncate max-w-[180px]">{img?.name ?? 'Imagen'}</span>
          <button
            onclick={() => { imageStore.clear(); vectorStore.clear(); }}
            disabled={vectorizing}
            class="text-[10px] text-zinc-500 hover:text-red-400 transition-colors disabled:opacity-30 ml-1 flex items-center gap-1"
            title="Quitar imagen"
          >
            <svg class="w-3 h-3" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"/></svg>
          </button>
        </div>
      {/if}

      <span class="hidden sm:inline px-1.5 py-0.5 rounded text-[9px] uppercase font-bold tracking-widest text-zinc-500 border border-zinc-700/40">Desktop</span>

      {#if batchCount > 0}
        <button
          onclick={() => drawerOpen = !drawerOpen}
          class="flex items-center gap-1.5 px-2 py-1 rounded text-[10px] font-medium transition-colors {drawerOpen ? 'bg-sky-500/10 text-sky-400' : 'text-zinc-500 hover:text-sky-400'}"
          title="Cola batch"
        >
          <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"/></svg>
          <span>{batchCount}</span>
        </button>
      {/if}
    </div>
  </header>

  <!-- ════════════════ PROGRESS BAR ════════════════ -->
  {#if vectorizing}
    <div class="h-0.5 bg-zinc-800 shrink-0">
      <div class="h-full bg-gradient-to-r from-sky-500 via-teal-400 to-sky-500 animate-pulse w-full"></div>
    </div>
    <div class="h-5 shrink-0 bg-zinc-900 border-b border-zinc-800 flex items-center px-4">
      <div class="flex items-center gap-2">
        <svg class="w-3 h-3 text-sky-400 animate-spin" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"/>
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"/>
        </svg>
        <span class="text-[10px] text-zinc-400 font-medium">
          {#if batchStore.isProcessing}
            Procesando batch... {batchStore.results.length}/{batchCount}
          {:else}
            Vectorizando en Rust...
          {/if}
        </span>
      </div>
    </div>
  {/if}

  <!-- ════════════════════ MAIN WORKSPACE ════════════════════ -->
  <div class="flex-1 flex overflow-hidden">
    <!-- ───── Sidebar Toggle Button ───── -->
    {#if !sidebarPinned}
      <button
        onclick={() => sidebarPinned = true}
        class="absolute left-0 top-16 z-30 w-8 h-8 flex items-center justify-center bg-zinc-900 border border-zinc-700/50 rounded-r-lg text-zinc-500 hover:text-sky-400 transition-colors"
        title="Fijar panel"
      >
        <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7"/></svg>
      </button>
    {/if}

    <!-- ───── Left Sidebar Inspector ───── -->
    <aside
      class="shrink-0 border-r border-zinc-800 bg-zinc-950 flex flex-col transition-all duration-200 ease-in-out overflow-hidden {sidebarPinned ? 'w-72' : 'w-0 border-r-0'}"
    >
      {#if sidebarPinned}
        <div class="flex items-center justify-between px-3 py-2.5 border-b border-zinc-800/60">
          <span class="text-[10px] font-bold uppercase tracking-widest text-zinc-500">Inspector</span>
          <button
            onclick={() => sidebarPinned = false}
            class="text-zinc-600 hover:text-zinc-400 transition-colors"
            title="Desfijar panel"
          >
            <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M15 19l-7-7 7-7"/></svg>
          </button>
        </div>
        <div class="flex-1 overflow-y-auto p-3">
          <ControlPanel />
        </div>
      {/if}
    </aside>

    <!-- ───── Central Canvas ───── -->
    <div class="flex-1 flex flex-col overflow-hidden bg-zinc-950">
      {#if !hasImage}
        <div class="flex-1 flex items-center justify-center">
          <DropZone />
        </div>
      {:else}
        <div class="flex-1 overflow-hidden p-3">
          <SplitPreview />
        </div>
        <div class="shrink-0 border-t border-zinc-800/40 px-3 py-2.5 bg-zinc-950">
          <ActionBar />
        </div>
      {/if}
    </div>
  </div>

  <!-- ═══════════════════ BOTTOM DRAWER ═══════════════════ -->
  {#if drawerOpen && (batchCount > 0 || batchHasResults)}
    <div class="shrink-0 border-t border-zinc-800 bg-zinc-950 max-h-[40vh] overflow-y-auto">
      <div class="flex items-center justify-between px-4 py-2 border-b border-zinc-800/60">
        <div class="flex items-center gap-2">
          <span class="text-[10px] font-bold uppercase tracking-widest text-zinc-500">Cola Batch</span>
          {#if batchCount > 0}
            <span class="px-1.5 py-0.5 rounded text-[9px] bg-sky-500/15 text-sky-400">{batchCount}</span>
          {/if}
        </div>
        <button
          onclick={() => drawerOpen = false}
          class="text-zinc-600 hover:text-zinc-400 transition-colors"
          title="Cerrar bandeja"
        >
          <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"/></svg>
        </button>
      </div>
      <div class="p-3">
        <BatchPanel />
      </div>
    </div>
  {/if}
</div>

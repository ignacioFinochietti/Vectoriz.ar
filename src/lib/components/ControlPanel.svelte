<script lang="ts">
  import SliderControl from './SliderControl.svelte';
  import PresetSelector from './PresetSelector.svelte';
  import { vectorStore } from '$lib/stores/vectorStore';

  let params = $derived(vectorStore.params);
  let activePreset = $derived(vectorStore.activePreset);
  let vectorizing = $state(false);
  let error = $state<string | null>(null);
  let success = $state(false);

  function updateParam(key: string, value: number) {
    vectorStore.updateParam(key as any, value);
  }

  async function startVectorize() {
    if (vectorizing) return;
    vectorizing = true;
    error = null;
    success = false;
    try {
      await vectorStore.vectorize();
      success = true;
    } catch (e) {
      error = String(e);
    } finally {
      vectorizing = false;
    }
  }
</script>

<div class="space-y-5">
  <div>
    <h3 class="text-xs font-semibold text-zinc-300 mb-3 tracking-wide">Preestablecidos</h3>
    <PresetSelector />
  </div>

  <div class="border-t border-zinc-800/60 pt-5">
    <h4 class="text-[11px] font-semibold text-zinc-500 uppercase tracking-widest mb-3">Filtros de Entrada</h4>
    <div class="space-y-3">
      <SliderControl
        label="Desenfoque Gaussiano"
        value={params.blur}
        min={0}
        max={10}
        step={1}
        onchange={(v) => updateParam('blur', v)}
      />
      <SliderControl
        label="Umbral de Binarizaci&oacute;n"
        value={params.threshold}
        min={0}
        max={255}
        step={1}
        onchange={(v) => updateParam('threshold', v)}
      />
      <SliderControl
        label="Despeckle / Limpieza"
        value={params.despeckle}
        min={0}
        max={15}
        step={1}
        onchange={(v) => updateParam('despeckle', v)}
      />
    </div>
  </div>

  <div class="border-t border-zinc-800/60 pt-5">
    <h4 class="text-[11px] font-semibold text-zinc-500 uppercase tracking-widest mb-3">Ajuste de Vectores</h4>
    <div class="space-y-3">
      <SliderControl
        label="Sparsity (simplificar)"
        value={params.sparsity}
        min={1}
        max={10}
        step={1}
        onchange={(v) => updateParam('sparsity', v)}
      />
      <SliderControl
        label="Optimizaci&oacute;n Nodos"
        value={params.node_optimization}
        min={1}
        max={10}
        step={1}
        onchange={(v) => updateParam('node_optimization', v)}
      />
    </div>
  </div>

  {#if error}
    <div class="p-3 rounded-lg bg-red-950/80 border border-red-800/50 text-red-300 text-xs flex items-start gap-2">
      <svg class="w-4 h-4 mt-0.5 shrink-0" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v3m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
      </svg>
      <span>{error}</span>
    </div>
  {/if}
  {#if success}
    <div class="p-3 rounded-lg bg-emerald-950/80 border border-emerald-800/50 text-emerald-300 text-xs flex items-start gap-2">
      <svg class="w-4 h-4 mt-0.5 shrink-0" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
      </svg>
      <span>Vectorizaci&oacute;n completada.</span>
    </div>
  {/if}

  <button
    onclick={startVectorize}
    disabled={vectorizing}
    class="w-full py-2.5 rounded-lg font-semibold text-sm transition-all duration-150 flex items-center justify-center gap-2
      bg-gradient-to-r from-violet-600 to-indigo-600
      hover:from-violet-500 hover:to-indigo-500
      disabled:opacity-40 disabled:cursor-not-allowed
      shadow-lg shadow-violet-600/20 hover:shadow-violet-500/30"
  >
    {#if vectorizing}
      <svg class="w-4 h-4 animate-spin" fill="none" viewBox="0 0 24 24">
        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"/>
        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"/>
      </svg>
      Vectorizando...
    {:else}
      <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" d="M13 10V3L4 14h7v7l9-11h-7z"/>
      </svg>
      Vectorizar en Rust
    {/if}
  </button>
</div>

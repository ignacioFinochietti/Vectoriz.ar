<script lang="ts">
  import SliderControl from './SliderControl.svelte';
  import PresetSelector from './PresetSelector.svelte';
  import { vectorStore } from '$lib/stores/vectorStore';
  import { batchStore } from '$lib/stores/batchStore';
  import { imageStore } from '$lib/stores/imageStore';

  let params = $derived(vectorStore.params);
  let activePreset = $derived(vectorStore.activePreset);
  let vectorizing = $derived(vectorStore.vectorizing || batchStore.isProcessing);
  let error = $state<string | null>(null);
  let success = $state(false);

  function updateParam(key: string, value: number) {
    vectorStore.updateParam(key as any, value);
  }

  async function startVectorize() {
    if (vectorizing) return;
    error = null;
    success = false;
    try {
      await vectorStore.vectorize();
      success = true;
    } catch (e) {
      error = String(e);
    }
  }

  function addToQueue() {
    const img = imageStore.currentImage;
    if (!img) return;
    batchStore.add({
      id: crypto.randomUUID(),
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
    success = true;
    setTimeout(() => { success = false; }, 2000);
  }

  const presetColors = ['', '#000000', '#ef4444', '#3b82f6', '#10b981'];
  let isCustomColor = $derived(vectorStore.outputColor !== '' && !presetColors.includes(vectorStore.outputColor));
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
        value={params.blur_radius}
        min={0}
        max={10}
        step={1}
        onchange={(v) => updateParam('blur_radius', v)}
      />
      <SliderControl
        label={params.mode === 'contour' ? 'Agrupación de Color / Detalle' : 'Umbral de Binarización'}
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

  <div class="border-t border-zinc-800/60 pt-5">
    <h4 class="text-[11px] font-semibold text-zinc-500 uppercase tracking-widest mb-3">Color de Salida</h4>
    <div class="flex items-center gap-2">
      <!-- Original -->
      <button
        onclick={() => vectorStore.outputColor = ''}
        class="w-6 h-6 rounded-full border flex items-center justify-center transition-all duration-150 relative overflow-hidden
          {vectorStore.outputColor === '' ? 'border-sky-500 ring-2 ring-sky-500/20' : 'border-zinc-700 hover:border-zinc-500 bg-zinc-900'}"
        title="Color original"
      >
        <span class="text-[9px] font-bold text-zinc-400">Orig</span>
      </button>

      <!-- Black -->
      <button
        onclick={() => vectorStore.outputColor = '#000000'}
        class="w-6 h-6 rounded-full border bg-black transition-all duration-150
          {vectorStore.outputColor === '#000000' ? 'border-sky-500 ring-2 ring-sky-500/20 scale-110' : 'border-zinc-700 hover:border-zinc-500'}"
        title="Negro"
      ></button>

      <!-- Red -->
      <button
        onclick={() => vectorStore.outputColor = '#ef4444'}
        class="w-6 h-6 rounded-full border bg-red-500 transition-all duration-150
          {vectorStore.outputColor === '#ef4444' ? 'border-sky-500 ring-2 ring-sky-500/20 scale-110' : 'border-zinc-700 hover:border-zinc-500'}"
        title="Rojo"
      ></button>

      <!-- Blue -->
      <button
        onclick={() => vectorStore.outputColor = '#3b82f6'}
        class="w-6 h-6 rounded-full border bg-blue-500 transition-all duration-150
          {vectorStore.outputColor === '#3b82f6' ? 'border-sky-500 ring-2 ring-sky-500/20 scale-110' : 'border-zinc-700 hover:border-zinc-500'}"
        title="Azul"
      ></button>

      <!-- Green -->
      <button
        onclick={() => vectorStore.outputColor = '#10b981'}
        class="w-6 h-6 rounded-full border bg-emerald-500 transition-all duration-150
          {vectorStore.outputColor === '#10b981' ? 'border-sky-500 ring-2 ring-sky-500/20 scale-110' : 'border-zinc-700 hover:border-zinc-500'}"
        title="Verde"
      ></button>

      <!-- Custom -->
      <div class="relative w-6 h-6">
        <input
          type="color"
          value={isCustomColor ? vectorStore.outputColor : '#6366f1'}
          oninput={(e) => vectorStore.outputColor = (e.target as HTMLInputElement).value}
          class="absolute inset-0 opacity-0 w-6 h-6 cursor-pointer z-10"
          title="Color personalizado"
        />
        <button
          class="absolute inset-0 w-6 h-6 rounded-full border flex items-center justify-center transition-all duration-150
            {isCustomColor ? 'border-sky-500 ring-2 ring-sky-500/20 scale-110' : 'border-zinc-700 hover:border-zinc-500 bg-zinc-900'}"
          style={isCustomColor ? `background-color: ${vectorStore.outputColor}` : ''}
          title="Personalizado"
        >
          {#if !isCustomColor}
            <svg class="w-3.5 h-3.5 text-zinc-400" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" d="M12 4v16m8-8H4"/>
            </svg>
          {/if}
        </button>
      </div>
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
      bg-gradient-to-r from-sky-600 to-teal-600
      hover:from-sky-500 hover:to-teal-500
      disabled:opacity-40 disabled:cursor-not-allowed
      shadow-lg shadow-sky-600/20 hover:shadow-sky-500/30"
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

  <button
    onclick={addToQueue}
    disabled={vectorizing}
    class="w-full py-2 rounded-lg text-xs font-medium transition-all duration-150 flex items-center justify-center gap-1.5
      border border-sky-500/30 text-sky-400
      hover:bg-sky-500/10 hover:border-sky-500/50
      disabled:opacity-30 disabled:cursor-not-allowed"
  >
    <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
      <path stroke-linecap="round" stroke-linejoin="round" d="M12 4v16m8-8H4"/>
    </svg>
    Agregar a Cola Batch
  </button>
</div>

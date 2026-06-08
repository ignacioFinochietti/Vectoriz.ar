<script lang="ts">
  import { vectorStore } from '$lib/stores/vectorStore';
  import type { Preset } from '$lib/types/presets';

  let presets: Preset[] = [
    {
      name: 'Firma',
      icon: 'edit',
      badge: 'L&iacute;nea',
      blur: 3, threshold: 160, despeckle: 5, sparsity: 7, node_optimization: 8, mode: 'centerline'
    },
    {
      name: 'Logo',
      icon: 'vector',
      badge: 'Curvas',
      blur: 2, threshold: 128, despeckle: 3, sparsity: 5, node_optimization: 6, mode: 'contour'
    },
    {
      name: 'Pixelart',
      icon: 'grid',
      badge: 'Rectas',
      blur: 0, threshold: 200, despeckle: 1, sparsity: 1, node_optimization: 10, mode: 'contour'
    },
    {
      name: 'Fotograf&iacute;a',
      icon: 'image',
      badge: 'Detalle',
      blur: 4, threshold: 100, despeckle: 8, sparsity: 3, node_optimization: 3, mode: 'contour'
    }
  ];

  let active = $state('Logo');

  function selectPreset(p: Preset) {
    active = p.name;
    vectorStore.setPreset(p);
  }

  function presetIcon(icon: string) {
    switch (icon) {
      case 'edit': return 'M15.232 5.232l3.536 3.536M9 11l-5 5v4h4l5-5';
      case 'vector': return 'M4 20c3-8 8-16 16-4';
      case 'grid': return 'M4 4h16v16H4z M4 12h16 M12 4v16';
      case 'image': return 'M4 16l4-4 4 4 4-4 4 4 M4 20h16V4H4z';
      default: return 'M4 16l4-4 4 4 4-4 4 4';
    }
  }

  function btnClass(activeName: string): string {
    const a = active === activeName;
    let cls = 'relative px-3 py-2.5 rounded-lg border text-left transition-all duration-150';
    if (a) {
      cls += ' border-violet-500/50 bg-violet-500/10 ring-1 ring-violet-500/20 shadow-sm shadow-violet-500/5';
    } else {
      cls += ' border-zinc-700/40 bg-zinc-900/60';
    }
    return cls;
  }
</script>

<div class="grid grid-cols-2 gap-2">
  {#each presets as p}
    <button
      onclick={() => selectPreset(p)}
      class={btnClass(p.name)}
    >
      <div class="flex items-center justify-between">
        <svg class="w-4 h-4 text-zinc-400" fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" d={presetIcon(p.icon)} />
        </svg>
        <span class="text-[9px] uppercase font-bold tracking-widest text-violet-400/70 bg-violet-500/10 px-1.5 py-0.5 rounded">{p.badge}</span>
      </div>
      <span class="block text-xs font-semibold text-zinc-200 mt-1" data-preset-name={p.name}>{p.name}</span>
    </button>
  {/each}
</div>

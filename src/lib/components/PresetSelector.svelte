<script lang="ts">
  import { vectorStore } from '$lib/stores/vectorStore';
  import { PRESETS, type Preset } from '$lib/types/presets';

  let presets: Preset[] = PRESETS;
  let active = $derived(vectorStore.activePreset);

  function selectPreset(p: Preset) {
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
      cls += ' border-sky-500/50 bg-sky-500/10 ring-1 ring-sky-500/20 shadow-sm shadow-sky-500/5';
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
        <span class="text-[9px] uppercase font-bold tracking-widest text-sky-400/70 bg-sky-500/10 px-1.5 py-0.5 rounded">{p.badge}</span>
      </div>
      <span class="block text-xs font-semibold text-zinc-200 mt-1" data-preset-name={p.name}>{p.name}</span>
    </button>
  {/each}
</div>

<script lang="ts">
  import { vectorStore } from '$lib/stores/vectorStore';

  let hasSvg = $derived(vectorStore.svgResult !== null);
  let vectorizing = $derived(vectorStore.vectorizing);
  let currentSvg = $derived(vectorStore.svgResult);

  async function downloadSvg() {
    if (!currentSvg) return;
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      await invoke('save_svg', { data: currentSvg });
    } catch {
      const blob = new Blob([currentSvg], { type: 'image/svg+xml' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = 'vectorizado.svg';
      a.click();
      URL.revokeObjectURL(url);
    }
  }

  async function copySvg() {
    if (!currentSvg) return;
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      await invoke('copy_svg_to_clipboard', { data: currentSvg });
    } catch {
      await navigator.clipboard.writeText(currentSvg);
    }
  }

  const dlBtnClass = 'flex-1 py-2 rounded-lg text-xs font-semibold transition-all duration-150 flex items-center justify-center gap-1.5 bg-gradient-to-r from-emerald-600 to-emerald-700 hover:from-emerald-500 hover:to-emerald-600 disabled:opacity-30 disabled:cursor-not-allowed';
  const cpBtnClass = 'flex-1 py-2 rounded-lg text-xs font-semibold transition-all duration-150 flex items-center justify-center gap-1.5 bg-zinc-800 border border-zinc-700/50 hover:bg-zinc-700 hover:border-zinc-600 disabled:opacity-30 disabled:cursor-not-allowed';
</script>

<div class="flex gap-2">
  <button
    onclick={downloadSvg}
    disabled={!hasSvg || vectorizing}
    class={dlBtnClass}
  >
    <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
      <path stroke-linecap="round" stroke-linejoin="round" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4-4m0 0l-4 4m4-4v12"/>
    </svg>
    Descargar SVG
  </button>
  <button
    onclick={copySvg}
    disabled={!hasSvg || vectorizing}
    class={cpBtnClass}
  >
    <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
      <path stroke-linecap="round" stroke-linejoin="round" d="M8 5H6a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2v-1M8 5a2 2 0 002 2h2a2 2 0 002-2M8 5a2 2 0 012-2h2a2 2 0 012 2m0 0h2a2 2 0 012 2v3m2 4H10m0 0l3-3m-3 3l3 3"/>
    </svg>
    Copiar c&oacute;digo SVG
  </button>
</div>

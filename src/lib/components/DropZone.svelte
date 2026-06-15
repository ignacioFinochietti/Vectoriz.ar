<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { imageStore } from '$lib/stores/imageStore';

  let dragging = $state(false);
  let hasImage = $derived(imageStore.current !== null);

  let fileInput = $state<HTMLInputElement>();

  function getDropClass() {
    let cls = 'relative w-full max-w-lg border-2 border-dashed rounded-2xl transition-all duration-200 cursor-pointer';
    if (dragging) {
      cls += ' border-sky-500/60 bg-sky-500/5';
    } else {
      cls += ' border-zinc-700/50 bg-zinc-900/40';
    }
    return cls;
  }

  async function onDrop(e: DragEvent) {
    dragging = false;
    const file = e.dataTransfer?.files?.[0];
    if (file && file.type.startsWith('image/')) {
      const url = URL.createObjectURL(file);
      imageStore.set({ file, url, name: file.name });
      await imageStore.loadImage(file);
    }
  }

  async function onFileChange(e: Event) {
    const input = e.target as HTMLInputElement;
    const file = input.files?.[0];
    if (file) {
      const url = URL.createObjectURL(file);
      imageStore.set({ file, url, name: file.name });
      await imageStore.loadImage(file);
    }
  }

  function openFilePicker() {
    fileInput?.click();
  }

  async function loadDemo(kind: string) {
    const name = kind === 'logo' ? 'demo_logo.png' : 'demo_photo.png';
    try {
      const response = await fetch(`/${name}`);
      const blob = await response.blob();
      const file = new File([blob], name, { type: 'image/png' });
      const url = URL.createObjectURL(file);
      imageStore.set({ file, url, name: file.name });
      await imageStore.loadImage(file);
    } catch (e) {
      console.error('Failed to load demo image:', e);
      const svg = kind === 'logo'
        ? `<svg viewBox="0 0 200 200" xmlns="http://www.w3.org/2000/svg"><circle cx="100" cy="100" r="80" fill="none" stroke="#0ea5e9" stroke-width="4"/><path d="M60 120 L100 60 L140 120" fill="none" stroke="#0ea5e9" stroke-width="4"/></svg>`
        : `<svg viewBox="0 0 200 200" xmlns="http://www.w3.org/2000/svg"><rect x="20" y="20" width="160" height="160" fill="none" stroke="#a1a1aa" stroke-width="2"/><circle cx="60" cy="70" r="20" fill="#a1a1aa"/><path d="M20 160 Q60 120 100 140 T180 100" fill="none" stroke="#a1a1aa" stroke-width="2"/></svg>`;
      const blob = new Blob([svg], { type: 'image/svg+xml' });
      const file = new File([blob], kind === 'logo' ? 'demo_logo.svg' : 'demo_photo.svg', { type: 'image/svg+xml' });
      imageStore.set({ file, url: URL.createObjectURL(blob), name: file.name });
      await imageStore.loadImage(file);
    }
  }
</script>

{#if !hasImage}
  <div class="flex flex-col items-center text-center py-20">
    <div class="mb-8">
      <h1 class="text-5xl md:text-6xl font-bold tracking-tight leading-tight">
        <span class="bg-gradient-to-r from-sky-400 via-sky-300 to-teal-400 bg-clip-text text-transparent">Vectorizado local</span><br/>
        <span class="text-zinc-300">ultra-r&aacute;pido</span>
      </h1>
      <p class="mt-4 text-zinc-400 text-sm max-w-md mx-auto leading-relaxed">
        Sin servidores. Sin l&iacute;mites. Solo tu navegador y el motor en Rust de alto rendimiento.
      </p>
    </div>

    <div
      class={getDropClass()}
      ondragover={(e) => { e.preventDefault(); dragging = true; }}
      ondragleave={() => { dragging = false; }}
      ondrop={(e) => { e.preventDefault(); onDrop(e); }}
      onclick={openFilePicker}
      role="button"
      tabindex="0"
      onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') openFilePicker(); }}
    >
      <input
        type="file"
        accept="image/*"
        class="hidden"
        bind:this={fileInput}
        onchange={onFileChange}
      />

      <div class="flex flex-col items-center py-12 px-8">
        <div class="w-20 h-20 rounded-full bg-gradient-to-br from-sky-600/20 to-teal-600/10 border border-sky-500/20 flex items-center justify-center mb-6 transition-transform hover:scale-110">
          <svg class="w-9 h-9 text-sky-400" fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" d="M3 16.5v2.25A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75V16.5m-13.5-9L12 3m0 0l4.5 4.5M12 3v13.5"/>
          </svg>
        </div>
        <span class="text-zinc-300 font-medium text-base mb-1">Arrastra tu imagen aqu&iacute;</span>
        <span class="text-zinc-500 text-xs">o haz clic para seleccionar (PNG, JPG, WEBP)</span>
      </div>
    </div>

    <div class="mt-8 flex items-center gap-3">
      <span class="text-zinc-500 text-xs">Prueba con:</span>
      <button onclick={() => loadDemo('logo')} class="px-3 py-1.5 rounded-lg border border-zinc-700/50 hover:border-sky-500/40 hover:bg-sky-500/10 text-xs font-medium text-zinc-400 hover:text-sky-300 transition-all">
        Logo de prueba
      </button>
      <button onclick={() => loadDemo('foto')} class="px-3 py-1.5 rounded-lg border border-zinc-700/50 hover:border-sky-500/40 hover:bg-sky-500/10 text-xs font-medium text-zinc-400 hover:text-sky-300 transition-all">
        Foto de prueba
      </button>
    </div>
  </div>
{/if}

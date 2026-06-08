import type { VectorizeParams, VectorizeMode } from '$lib/types/vectorization';
import type { Preset } from '$lib/types/presets';
import { imageStore } from './imageStore.svelte';

let svg = $state<string>('');
let isProcessing = $state(false);
let processingTimeMs = $state(0);
let params = $state<VectorizeParams>({
  mode: 'contour',
  threshold: 128,
  blur_radius: 2,
  despeckle: 3,
  sparsity: 5,
  node_optimization: 6,
});
let activePreset = $state<string>('Logo');

export const vectorStore = {
  get svg() { return svg; },
  get svgResult() { return svg || null; },
  get isProcessing() { return isProcessing; },
  get vectorizing() { return isProcessing; },
  get processingTimeMs() { return processingTimeMs; },
  get params() { return params; },
  get activePreset() { return activePreset; },

  setPreset(p: Preset) {
    activePreset = p.name;
    params = {
      mode: p.mode as VectorizeMode,
      threshold: p.threshold,
      blur_radius: p.blur,
      despeckle: p.despeckle,
      sparsity: p.sparsity,
      node_optimization: p.node_optimization,
    };
  },

  updateParam(key: keyof VectorizeParams, value: number) {
    params = { ...params, [key]: value };
  },

  async vectorize() {
    const img = imageStore.currentImage;
    if (!img) throw new Error('No image loaded');
    isProcessing = true;
    svg = '';
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      const result = await invoke<string>('vectorize_image', {
        data: img.data_base64,
        width: img.width,
        height: img.height,
        mode: params.mode,
        threshold: params.threshold,
        blurRadius: params.blur_radius,
        despeckle: params.despeckle,
        sparsity: params.sparsity,
        nodeOptimization: params.node_optimization,
      });
      svg = result;
    } finally {
      isProcessing = false;
    }
  },

  clear() {
    svg = '';
    processingTimeMs = 0;
  },
};

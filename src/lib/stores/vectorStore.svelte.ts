import { invoke } from '@tauri-apps/api/core';
import type { VectorizeParams, VectorizeMode, VectorizeResult } from '$lib/types/vectorization';
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
let outputColor = $state<string>('');

function recolorSvg(svgText: string, color: string): string {
  return svgText.replace(/(<[a-zA-Z0-9]+[^>]+)(fill|stroke)="([^"]+)"/g, (match, prefix, attr, val) => {
    if (val === 'none') {
      return match;
    }
    if (attr === 'fill') {
      const lowerVal = val.toLowerCase().trim();
      if (lowerVal === '#ffffff' || lowerVal === 'white' || lowerVal === 'rgb(255,255,255)' || lowerVal === 'rgb(255, 255, 255)') {
        return match;
      }
      return `${prefix}fill="${color}"`;
    } else if (attr === 'stroke') {
      return `${prefix}stroke="${color}"`;
    }
    return match;
  });
}

export const vectorStore = {
  get svg() { return svg; },
  get svgResult() {
    if (!svg) return null;
    if (outputColor) {
      return recolorSvg(svg, outputColor);
    }
    return svg;
  },
  get isProcessing() { return isProcessing; },
  get vectorizing() { return isProcessing; },
  get processingTimeMs() { return processingTimeMs; },
  get params() { return params; },
  get activePreset() { return activePreset; },
  get outputColor() { return outputColor; },
  set outputColor(val: string) { outputColor = val; },

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
      const result = await invoke<VectorizeResult>('vectorize_image', {
        dataBase64: img.data_base64,
        params: {
          mode: params.mode,
          threshold: params.threshold,
          blur_radius: params.blur_radius,
          despeckle: params.despeckle,
          sparsity: params.sparsity,
          node_optimization: params.node_optimization,
        },
      });
      svg = result.svg;
      processingTimeMs = result.processing_time_ms;
    } catch (e) {
      console.error('[vectorStore] vectorize failed:', e);
      throw e;
    } finally {
      isProcessing = false;
    }
  },

  clear() {
    svg = '';
    processingTimeMs = 0;
    outputColor = '';
  },
};

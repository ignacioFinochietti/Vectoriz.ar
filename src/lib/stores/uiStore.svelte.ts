import type { VectorizeMode, VectorizeParams } from '$lib/types/vectorization';
import { PRESETS, type Preset } from '$lib/types/presets';

export const DEFAULT_PARAMS: VectorizeParams = {
  mode: 'contour',
  threshold: 128,
  blur_radius: 0.5,
  despeckle: 4,
  sparsity: 60,
  node_optimization: 0.5,
};

let mode = $state<VectorizeMode>('contour');
let params = $state<VectorizeParams>({ ...DEFAULT_PARAMS });
let activePreset = $state<Preset | null>(null);

function paramsMatchPreset(p: VectorizeParams, preset: Preset): boolean {
  return (
    p.mode === preset.params.mode &&
    p.threshold === preset.params.threshold &&
    p.blur_radius === preset.params.blur_radius &&
    p.despeckle === preset.params.despeckle &&
    p.sparsity === preset.params.sparsity &&
    p.node_optimization === preset.params.node_optimization
  );
}

export const uiStore = {
  get mode() { return mode; },
  set mode(val: VectorizeMode) {
    mode = val;
    params.mode = val;
  },
  get params() { return params; },
  get activePreset() { return activePreset; },
  get isModified(): boolean {
    if (!activePreset) return false;
    return !paramsMatchPreset(params, activePreset);
  },
  applyPreset(preset: Preset) {
    activePreset = preset;
    mode = preset.params.mode;
    params = { ...preset.params };
  },
  resetToPreset() {
    if (activePreset) {
      this.applyPreset(activePreset);
    }
  },
  updateParam<K extends keyof VectorizeParams>(key: K, value: VectorizeParams[K]) {
    params = { ...params, [key]: value };
  },
  resetParams() {
    params = { ...DEFAULT_PARAMS };
    mode = DEFAULT_PARAMS.mode;
    activePreset = null;
  },
};

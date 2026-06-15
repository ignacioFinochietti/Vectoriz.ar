import { invoke } from '@tauri-apps/api/core';
import type { VectorizeParams } from '$lib/types/vectorization';

export interface BatchEntry {
  id: string;
  data_base64: string;
  name: string;
  url: string;
  params: VectorizeParams;
}

export interface BatchResultItem {
  id: string;
  name: string;
  svg: string | null;
  error: string | null;
}

let queue = $state<BatchEntry[]>([]);
let results = $state<BatchResultItem[]>([]);
let isProcessing = $state(false);

export const batchStore = {
  get queue() { return queue; },
  get results() { return results; },
  get isProcessing() { return isProcessing; },
  get hasQueue() { return queue.length > 0; },
  get hasResults() { return results.length > 0; },

  add(entry: BatchEntry) {
    if (!queue.find(e => e.id === entry.id)) {
      queue = [...queue, entry];
    }
  },

  remove(id: string) {
    queue = queue.filter(e => e.id !== id);
  },

  clear() {
    queue = [];
    results = [];
  },

  clearResults() {
    results = [];
  },

  async processAll() {
    if (queue.length === 0 || isProcessing) return;
    isProcessing = true;
    results = [];
    const snapshot = [...queue];
    try {
      const batchResults = await invoke<Array<{
        name: string;
        result: { svg: string; width: number; height: number; processing_time_ms: number };
        error: string | null;
      }>>('vectorize_batch', {
        images: snapshot.map(e => ({
          dataBase64: e.data_base64,
          name: e.name,
          params: {
            mode: e.params.mode,
            threshold: e.params.threshold,
            blur_radius: e.params.blur_radius,
            despeckle: e.params.despeckle,
            sparsity: e.params.sparsity,
            node_optimization: e.params.node_optimization,
          },
        })),
      });
      results = batchResults.map((r, i) => ({
        id: snapshot[i].id,
        name: r.name,
        svg: r.error ? null : r.result.svg,
        error: r.error,
      }));
    } catch (e) {
      results = snapshot.map(s => ({
        id: s.id,
        name: s.name,
        svg: null,
        error: String(e),
      }));
    } finally {
      isProcessing = false;
    }
  },
};

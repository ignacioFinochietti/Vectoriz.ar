export type VectorizeMode = 'centerline' | 'contour';

export interface VectorizeParams {
  mode: VectorizeMode;
  threshold: number;
  blur_radius: number;
  despeckle: number;
  sparsity: number;
  node_optimization: number;
}

export interface VectorizeResult {
  svg: string;
  width: number;
  height: number;
  processing_time_ms: number;
}

import type { VectorizeMode } from './vectorization';

export interface Preset {
  name: string;
  icon: string;
  badge: string;
  blur: number;
  threshold: number;
  despeckle: number;
  sparsity: number;
  node_optimization: number;
  mode: VectorizeMode;
}

export const PRESETS: Preset[] = [
  {
    name: 'Firma',
    icon: 'edit',
    badge: 'L\u00ednea',
    blur: 3,
    threshold: 160,
    despeckle: 5,
    sparsity: 7,
    node_optimization: 8,
    mode: 'centerline',
  },
  {
    name: 'Logo',
    icon: 'vector',
    badge: 'Curvas',
    blur: 2,
    threshold: 128,
    despeckle: 3,
    sparsity: 5,
    node_optimization: 6,
    mode: 'contour',
  },
  {
    name: 'Pixelart',
    icon: 'grid',
    badge: 'Rectas',
    blur: 0,
    threshold: 200,
    despeckle: 1,
    sparsity: 1,
    node_optimization: 10,
    mode: 'contour',
  },
  {
    name: 'Fotograf\u00eda',
    icon: 'image',
    badge: 'Detalle',
    blur: 4,
    threshold: 100,
    despeckle: 8,
    sparsity: 3,
    node_optimization: 3,
    mode: 'contour',
  },
];

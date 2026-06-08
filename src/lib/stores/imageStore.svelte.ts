import type { ImageData } from '$lib/types/image';

interface FileInfo {
  file: File;
  url: string;
  name: string;
}

let fileInfo = $state<FileInfo | null>(null);
let currentImage = $state<ImageData | null>(null);
let originalUrl = $state<string>('');

export const imageStore = {
  get fileInfo() { return fileInfo; },
  get current() { return currentImage; },
  get currentImage() { return currentImage; },
  set currentImage(val: ImageData | null) { currentImage = val; },
  get originalUrl() { return originalUrl; },
  set originalUrl(val: string) { originalUrl = val; },
  set(data: FileInfo) { fileInfo = data; },
  clear() {
    fileInfo = null;
    currentImage = null;
    originalUrl = '';
  },
};

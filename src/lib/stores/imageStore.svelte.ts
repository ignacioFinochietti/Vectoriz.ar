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

  async loadImage(file: File) {
    const dataUrl = await new Promise<string>((resolve, reject) => {
      const reader = new FileReader();
      reader.onload = () => resolve(reader.result as string);
      reader.onerror = reject;
      reader.readAsDataURL(file);
    });
    const img = await new Promise<HTMLImageElement>((resolve, reject) => {
      const i = new Image();
      i.onload = () => resolve(i);
      i.onerror = reject;
      i.src = dataUrl;
    });
    currentImage = {
      data_base64: dataUrl.split(',')[1] ?? dataUrl,
      width: img.naturalWidth,
      height: img.naturalHeight,
      name: file.name,
    };
    originalUrl = dataUrl;
  },

  clear() {
    if (fileInfo?.url) URL.revokeObjectURL(fileInfo.url);
    fileInfo = null;
    currentImage = null;
    originalUrl = '';
  },
};

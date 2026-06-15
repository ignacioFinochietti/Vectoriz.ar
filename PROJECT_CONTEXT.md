# Vectoriz.ar — Contexto Completo del Proyecto

## Visión General

**Vectoriz.ar** es una app desktop de vectorización de imágenes (raster → SVG). Usa **Tauri v2** (Rust backend + Svelte 5 frontend) con **VTracer** para trazado de contornos y un algoritmo propio de **centerline** (Zhang-Suen thinning) para firmas/trazos.

---

## Stack Tecnológico

### Frontend
- **Svelte 5** con runes (`$state`, `$derived`, `$effect`)
- **SvelteKit** con adapter-static
- **Tailwind CSS v4** via `@tailwindcss/vite`
- **Vite 6** como bundler

### Backend (Rust)
- **Tauri v2** (framework desktop)
- **VTracer 0.6** — vectorización de contornos
- **image 0.25** / **imageproc 0.25** — manipulación de imágenes, blur gaussiano
- **base64 0.22** — decodificación de imágenes recibidas del frontend
- **rayon 1** — procesamiento paralelo para batch vectorization
- **serde / serde_json** — serialización JS ↔ Rust
- Plugins Tauri: `dialog`, `clipboard-manager`, `fs`

---

## Estructura de Archivos (Actualizada)

```
veriz.ar/
├── package.json
├── svelte.config.js
├── tsconfig.json
├── vite.config.ts                # Vite + tailwind + sveltekit, watch.ignored: target/
├── app-icon.png
│
├── src/
│   ├── app.html                  # h-screen, overflow-hidden, sky accent
│   ├── app.css                   # Tailwind + checkerboard pattern
│   ├── routes/
│   │   ├── +layout.svelte        # Minimal shell: solo render children
│   │   └── +page.svelte          # Workspace completo (header + sidebar + canvas + drawer)
│   └── lib/
│       ├── stores/
│       │   ├── imageStore.svelte.ts
│       │   ├── vectorStore.svelte.ts
│       │   └── batchStore.svelte.ts
│       ├── types/
│       │   ├── image.ts
│       │   ├── presets.ts
│       │   └── vectorization.ts
│       └── components/
│           ├── ActionBar.svelte       # Descargar/Copiar SVG
│           ├── BatchPanel.svelte      # Cola batch + resultados + visor SVG
│           ├── ControlPanel.svelte    # Sliders + presets + botones Vectorizar/Agregar
│           ├── DropZone.svelte        # Drag & drop + file picker
│           ├── PresetSelector.svelte  # Grid 2x2 presets
│           ├── SliderControl.svelte   # Slider reutilizable
│           └── SplitPreview.svelte    # Canvas con modos Side-by-side y Superponer
│
└── src-tauri/
    ├── Cargo.toml
    ├── tauri.conf.json
    ├── build.rs
    ├── capabilities/default.json
    ├── src/
    │   ├── main.rs
    │   ├── lib.rs
    │   ├── commands/
    │   │   ├── mod.rs
    │   │   ├── vectorize.rs       # vectorize_image + vectorize_batch (rayon)
    │   │   └── file_ops.rs        # save_svg + copy_svg_to_clipboard
    │   ├── engine/
    │   │   ├── mod.rs
    │   │   ├── types.rs           # Todos los tipos serializables
    │   │   ├── contour.rs         # VTracer contour
    │   │   └── centerline.rs      # Thinning + skeleton + RDP simplify
    │   └── image/
    │       ├── mod.rs
    │       ├── load.rs            # Carga + blur gaussiano
    │       └── binarize.rs        # Otsu (no usado actualmente)
    └── icons/
```



---

## Diseño de Layout (Workspace Principal)

### `+layout.svelte`
Shell mínimo. Solo renderiza `children`. Todo el layout está en `+page.svelte`.

### `+page.svelte` — Workspace 100vh/100vw sin scroll global

```
┌─────────────────────────────────────────────────────────────┐
│ HEADER h-12: brand | image thumbnail | batch badge | util  │
├──────────┬──────────────────────────────────────────────────┤
│ SIDEBAR  │              CENTRAL CANVAS                      │
│ w-72     │                                                  │
│ pinnable │   empty → DropZone                               │
│ ◀ unpin  │   active → SplitPreview + ActionBar             │
│          │                                                  │
│ Control  │                                                  │
│ Panel    │                                                  │
│ inside   │                                                  │
│ scroll   │                                                  │
├──────────┴──────────────────────────────────────────────────┤
│ BOTTOM DRAWER (collapsible, max 40vh)                       │
│ BatchPanel inside                                           │
└─────────────────────────────────────────────────────────────┘
```

**Estados del sidebar:**
- `$state(sidebarPinned = true)`: Sidebar visible (w-72), con toggle para colapsar
- `$state(sidebarPinned = false)`: Sidebar oculto (w-0), botón flotante para re-abrir

**Estados del drawer:**
- `$state(drawerOpen = false)`: Oculto
- Se abre automáticamente cuando `batchStore.hasResults` cambia a true
- Se cierra con botón X
- Contiene `<BatchPanel />`

### `SplitPreview.svelte` — Visor Canvas con dos modos

**Toolbar superior:**
- Toggle `[Comparar | Superponer]` con indicador de modo activo
- Grid toggle (checkerboard on/off)
- Processing time display

**Modo Side-by-Side (default):**
- Grid 2 columnas 50/50
- Izquierda: raster original (desde `imageStore.fileInfo.url`)
- Derecha: SVG vectorizado (desde `vectorStore.svgResult` → blob URL)

**Modo Overlay:**
- Contenedor único `relative`
- Capa base: raster (positioned normally)
- Capa superior: SVG con `absolute inset-0` y `opacity` dinámica
- Slider de opacidad (0-100%) en barra inferior con backdrop-blur
- Leyenda de capas (Raster/SVG)

**Color accent:** `sky` (sky-400, sky-500) reemplazó `violet` en la nueva UI

---

## Flujo de Datos

### Vectorización Individual
1. Usuario arrastra/selecciona imagen en **DropZone**
2. `imageStore.set({ file, url, name })` guarda FileInfo (para preview)
3. `imageStore.loadImage(file)` lee el archivo como base64, calcula dimensiones, guarda `currentImage`
4. Usuario ajusta parámetros en **ControlPanel** (presets o sliders manuales)
5. Presiona "Vectorizar en Rust" → `vectorStore.vectorize()`
6. Invoca `vectorize_image` via Tauri invoke:
   ```
   invoke('vectorize_image', {
     dataBase64: "...",
     params: { mode, threshold, blurRadius, despeckle, sparsity, nodeOptimization }
   })
   ```
7. Rust decodifica base64 → image crate → blur gaussiano → VTracer (contour) o thinning (centerline)
8. Retorna `VectorizeResult { svg, width, height, processing_time_ms }`
9. SVG se renderiza en **SplitPreview**, se puede descargar/copiar via **ActionBar**

### Vectorización Batch
1. Usuario ajusta parámetros para la imagen actual
2. Presiona "Agregar a Cola Batch" → `batchStore.add({ id, data_base64, name, url, params })`
3. Repite para múltiples imágenes con distintos parámetros
4. Presiona "Vectorizar Todo" → `batchStore.processAll()`
5. Invoca `vectorize_batch` con array de `{ dataBase64, name, params }`
6. Rust procesa en paralelo con **rayon** (`par_iter`)
7. Retorna array de `{ name, result: VectorizeResult, error }`
8. Resultados se muestran en **BatchPanel** con opciones Ver código / Descargar

### Notas técnicas de la UI

- **Efectos `$effect`**: en `SplitPreview.svelte`, el efecto que genera blob URLs para el SVG nunca lee el estado `svgBlobUrl` que escribe; usa `untrack()` en el cleanup para evitar `effect_update_depth_exceeded`.
- **CSS global**: importado explícitamente en `+layout.svelte` (`import '../app.css'`) para que Tailwind v4 cargue correctamente.
- **Vite watcher**: `vite.config.ts` ignora `src-tauri/target/` para evitar EBUSY crashes.

---

## Tipos y Estructuras Clave

### Frontend (TypeScript)

```ts
// types/image.ts
interface ImageData {
  data_base64: string;
  width: number;
  height: number;
  name: string;
}

// types/vectorization.ts
type VectorizeMode = 'centerline' | 'contour';

interface VectorizeParams {
  mode: VectorizeMode;
  threshold: number;      // 0-255
  blur_radius: number;    // 0-10
  despeckle: number;      // 0-15
  sparsity: number;       // 1-10
  node_optimization: number; // 1-10
}

interface VectorizeResult {
  svg: string;
  width: number;
  height: number;
  processing_time_ms: number;
}

// types/presets.ts
interface Preset {
  name: string;
  icon: string;      // 'edit' | 'vector' | 'grid' | 'image'
  badge: string;
  blur: number;
  threshold: number;
  despeckle: number;
  sparsity: number;
  node_optimization: number;
  mode: VectorizeMode;
}

// stores/batchStore.svelte.ts
interface BatchEntry {
  id: string;
  data_base64: string;
  name: string;
  url: string;
  params: VectorizeParams;
}

interface BatchResultItem {
  id: string;
  name: string;
  svg: string | null;
  error: string | null;
}
```

### Rust

```rust
// engine/types.rs
enum VectorizeMode { Contour, Centerline }

struct VectorizeParams {
    mode: VectorizeMode,
    threshold: u8,           // via serde(rename = "contour"/"centerline")
    blur_radius: f64,
    despeckle: u32,
    sparsity: f64,
    node_optimization: f64,
}

struct VectorizeResult {
    svg: String,
    width: u32,
    height: u32,
    processing_time_ms: u64,
}

struct BatchImageInput {
    data_base64: String,
    name: String,
    params: VectorizeParams,
}

struct BatchResultItem {
    name: String,
    result: VectorizeResult,
    error: Option<String>,
}
```

### Mapeo JS → Rust (Tauri invoke)

Tauri v2 convierte camelCase → snake_case automáticamente **solo para argumentos de comando de primer nivel** (ej: `dataBase64` → `data_base64`). Para **objetos anidados** (como `params`) la conversión NO se aplica. El frontend envía los campos de `params` en snake_case exacto para evitar `missing field 'blur_radius'`.

| JS (request)        | Rust (parámetro)     |
|---------------------|---------------------|
| `dataBase64`        | `data_base64`       |
| `params.blur_radius` | `blur_radius`      |
| `params.node_optimization` | `node_optimization` |

| Rust (response)        | JS (resultado)         |
|------------------------|------------------------|
| `svg`                  | `result.svg`           |
| `processing_time_ms`   | `result.processing_time_ms` |

---

## Presets

| Nombre     | Modo       | Blur | Thresh | Despeckle | Sparsity | NodeOpt |
|------------|------------|------|--------|-----------|----------|---------|
| Firma      | centerline | 3    | 160    | 5         | 7        | 8       |
| Logo       | contour    | 2    | 128    | 3         | 5        | 6       |
| Pixelart   | contour    | 0    | 200    | 1         | 1        | 10      |
| Fotografía | contour    | 4    | 100    | 8         | 3        | 3       |

---

## Stores (Estado Global)

### imageStore
- `fileInfo: { file, url, name }` — info del File original + URL blob para preview
- `currentImage: ImageData` — base64 + dimensiones (lo que se envía a Rust)
- `originalUrl: string` — data URL completa (para thumbnails)
- `loadImage(file)` — lee File como base64, extrae dimensiones
- `clear()` — revokea URLs, resetea todo

### vectorStore
- `params: VectorizeParams` — parámetros actuales
- `svg: string` — resultado SVG de la última vectorización individual
- `isProcessing: boolean` — true durante vectorización
- `processingTimeMs: number` — tiempo de la última vectorización
- `activePreset: string` — nombre del preset activo
- `setPreset(p)` — aplica un preset completo
- `updateParam(key, value)` — modifica un parámetro individual
- `vectorize()` — invoca comando Rust individual
- `clear()` — limpia resultado

### batchStore
- `queue: BatchEntry[]` — cola de imágenes pendientes
- `results: BatchResultItem[]` — resultados del último batch
- `isProcessing: boolean`
- `add(entry)` — agrega a la cola
- `remove(id)` — quita de la cola
- `processAll()` — invoca `vectorize_batch` en Rust
- `clear()` — limpia cola + resultados

---

## Comandos Tauri (Rust → Frontend)

| Comando                    | Parámetros                          | Retorno                  |
|----------------------------|-------------------------------------|--------------------------|
| `vectorize_image`          | `data_base64`, `params`             | `VectorizeResult`        |
| `vectorize_batch`          | `images: BatchImageInput[]`         | `BatchResultItem[]`      |
| `save_svg`                 | `path`, `svg`                       | `()`                     |
| `copy_svg_to_clipboard`    | `svg` (app_handle implícito)        | `()`                     |

---

## Algoritmos de Vectorización

### Contour (VTracer)
- Usa la librería `vtracer` 0.6
- Parámetros mapeados: `filter_speckle` ← `despeckle`, `corner_threshold` ← `sparsity`, `path_precision` ← `node_optimization * 10`
- Config: `Hierarchical::Stacked`, `color_precision=6`, `layer_difference=249`
- Retorna SVG con paths y curvas

### Centerline (Custom)
- Pipeline: grayscale → binarize → remove_small_components → morphological_thin (Zhang-Suen) → trace_skeleton → RDP simplify → SVG
- `binarize`: threshold simple (pixel > threshold → blanco, sino negro)
- `remove_small_components`: BFS flood-fill, elimina componentes < despeckle píxeles
- `morphological_thin`: Zhang-Suen thinning con buffer swapping (optimizado, sin clone por iteración)
- `trace_skeleton`: BFS sobre píxeles del esqueleto
- `rdp_simplify`: Ramer-Douglas-Peucker con epsilon = (1 - node_optimization) * 5 + 0.5
- Genera paths SVG con `stroke="#000" stroke-width="1.5"`

---

## Bugs Corregidos en Esta Sesión

1. **`vectorize_image` fallaba en runtime**: Frontend enviaba campos planos (`data`, `width`, `height`, `mode`, ...) pero Rust esperaba `data_base64` + `params: VectorizeParams` anidado
2. **`imageStore.currentImage` nunca se poblaba**: DropZone solo seteaba `fileInfo`, pero `vectorize()` leía `currentImage` → siempre null
3. **`save_svg` y `copy_svg_to_clipboard` con nombres de parámetros incorrectos**: Frontend enviaba `data` pero Rust espera `path`/`svg` y `svg` respectivamente
4. **`uiStore.svelte.ts` era dead code con bugs**: Eliminado
5. **`ControlPanel` tenía estado `vectorizing` local aislado**: Ahora usa `$derived(vectorStore.vectorizing)`
6. **Sliders usaban `params.blur` en vez de `params.blur_radius`**: Corregido en todos los sliders
7. **Thumbnail en header usaba `img.url` (no existía)**: Cambiado a `imageStore.fileInfo.url`
8. **Vite watcher crasheaba con `src-tauri/target/`**: Agregado `watch.ignored` en `vite.config.ts`

---

## Mejoras Implementadas

1. **Batch vectorization**: Comando `vectorize_batch` con rayon (paralelo en todos los cores)
2. **BatchPanel**: UI completa con cola, botón "Vectorizar Todo", resultados, visor SVG modal
3. **Botón "Agregar a Cola Batch"** en ControlPanel
4. **`morphological_thin` optimizado**: Buffer swapping en vez de clone por iteración
5. **CSP configurado** en `tauri.conf.json` (antes era `null`)
6. **`dialog.save()` nativo** para guardar archivos (antes usaba descarga por blob)
7. **Static imports** de `@tauri-apps/api/core` (antes `await import(...)` en cada llamada)
8. **SVG code viewer** con syntax highlighting y renderizado inline en modal
9. **`processing_time_ms`** almacenado y disponible en el store

---

## Estado Actual (Build OK)

- `cargo check` → OK (2 warnings de dead code en `binarize.rs`, inofensivos)
- `vite build` → OK (sin errores)
- `npm run tauri dev` → OK (app se abre correctamente en `localhost:5173`)

---

## Workflows de Usuario

### Flujo Individual
1. Abrir app → DropZone visible
2. Arrastrar imagen o hacer clic → se muestra header con thumbnail + SplitPreview
3. Seleccionar preset (Firma/Logo/Pixelart/Foto) o ajustar sliders manualmente
4. Presionar "Vectorizar en Rust" → spinner → SVG aparece en SplitPreview
5. Descargar SVG (diálogo nativo) o Copiar código SVG al portapapeles
6. "Quitar Imagen" para resetear

### Flujo Batch
1. Cargar imagen, ajustar parámetros, presionar "Agregar a Cola Batch"
2. Repetir paso 1 para múltiples imágenes con distintos parámetros
3. La cola se muestra abajo con thumbnails y resumen de params
4. Presionar "Vectorizar Todo" → procesamiento paralelo en Rust
5. Resultados aparecen con opciones Ver (modal con código SVG + preview) y Descargar
6. "Limpiar" para resetear cola

---

## Posibles Mejoras Futuras

- **UI/UX**: Modo oscuro/claro toggle, animaciones de transición, drag & drop múltiple
- **Edición de SVG**: Visor interactivo con zoom/pan, edición de nodos
- **Formatos de salida**: PNG, PDF, EPS además de SVG
- **Pre-procesamiento**: Recorte, rotación, ajuste de contraste antes de vectorizar
- **Batch avanzado**: Progreso por imagen, cancelación, reintentos
- **Internacionalización**: i18n para español/inglés
- **Comparación A/B**: Ver resultados de distintos parámetros lado a lado
- **Exportación en lote**: Descargar todos los SVG del batch como ZIP
- **Historial**: Guardar vectorizaciones anteriores localmente
- **Perfiles personalizados**: Guardar/cargar configuraciones de parámetros

---

## Cómo Ejecutar

```bash
# Instalar dependencias
cd veriz.ar
npm install

# Desarrollo (frontend + Rust)
npm run tauri dev

# Solo frontend (para probar UI sin Rust)
npm run dev

# Build producción
npm run build           # frontend
npm run tauri build     # app completa
```

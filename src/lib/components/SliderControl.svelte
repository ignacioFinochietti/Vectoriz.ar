<script lang="ts">
  let { label, value, min, max, step = 1, onchange }: {
    label: string;
    value: number;
    min: number;
    max: number;
    step?: number;
    onchange?: (v: number) => void;
  } = $props();

  let localValue = $state(0);

  $effect(() => {
    localValue = value;
  });

  function handleInput(e: Event) {
    const v = parseFloat((e.target as HTMLInputElement).value);
    localValue = v;
    onchange?.(v);
  }

  let inputId = $state(crypto?.randomUUID?.() ?? Math.random().toString(36).slice(2));
</script>

<div class="space-y-1.5">
  <div class="flex justify-between items-center">
    <label for={inputId} class="text-[11px] font-medium text-zinc-400 tracking-wide">{label}</label>
    <span class="text-[11px] font-mono text-zinc-500 tabular-nums">{localValue.toFixed(step < 1 ? 1 : 0)}</span>
  </div>
  <input
    id={inputId}
    type="range"
    min={min}
    max={max}
    step={step}
    value={localValue}
    oninput={handleInput}
    class="w-full h-1.5 rounded-full appearance-none cursor-pointer
      bg-zinc-800
      accent-sky-500
      [&::-webkit-slider-thumb]:appearance-none
      [&::-webkit-slider-thumb]:w-3.5
      [&::-webkit-slider-thumb]:h-3.5
      [&::-webkit-slider-thumb]:rounded-full
      [&::-webkit-slider-thumb]:bg-sky-500
      [&::-webkit-slider-thumb]:shadow-md
      [&::-webkit-slider-thumb]:shadow-sky-500/30
      [&::-webkit-slider-thumb]:transition-transform
      [&::-webkit-slider-thumb]:duration-150
      [&::-webkit-slider-thumb]:hover:scale-125"
  />
</div>

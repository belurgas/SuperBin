<script lang="ts">
  import { Switch } from '@skeletonlabs/skeleton-svelte';

    let autoEmpty = true;
    let confirmation = false;
    let maxSize = 10;
</script>

<div class="bg-slate-800/50 backdrop-blur-sm rounded-2xl p-6 border border-emerald-800/20 fade-in">
  <h3 class="text-xl font-semibold mb-6">Recycle Bin Settings</h3>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <div>
        <p class="font-medium">Auto-empty recycle bin</p>
        <p class="text-sm text-slate-400">
          Automatically empty when full
        </p>
      </div>
      <div class="">
        <Switch name="example" checked={autoEmpty} onCheckedChange={(e: any) => (autoEmpty = e.checked)} />
      </div>
      <!-- <button 
        type="button"
        class="w-12 h-6 bg-emerald-600 rounded-full relative cursor-pointer"
        onclick={() => autoEmpty = !autoEmpty}
        aria-label={autoEmpty ? 'Disable auto-empty' : 'Enable auto-empty'}
        aria-pressed={autoEmpty}
      >
        <div 
          class="w-5 h-5 bg-white rounded-full absolute transition-transform top-0.5"
          class:translate-x-7={autoEmpty}
          aria-hidden="true"
        ></div>
      </button> -->
    </div>
    
    <div class="flex items-center justify-between">
      <div>
        <p class="font-medium">Confirmation dialog</p>
        <p class="text-sm text-slate-400">
          Show confirmation before deletion
        </p>
      </div>
      <div>
        <Switch name="example" checked={confirmation} onCheckedChange={(e: any) => (confirmation = e.checked)} />
      </div>
      <!-- <button 
        type="button"
        class="w-12 h-6 bg-slate-600 rounded-full relative cursor-pointer"
        onclick={() => confirmation = !confirmation}
        aria-label={confirmation ? 'Disable confirmation dialog' : 'Enable confirmation dialog'}
        aria-pressed={confirmation}
      >
        <div 
          class="w-5 h-5 bg-white rounded-full absolute transition-transform top-0.5"
          class:translate-x-7={confirmation}
          aria-hidden="true"
        ></div>
      </button> -->
    </div>
    
    <div>
      <p class="font-medium mb-2">Maximum size (GB)</p>
      <input
        type="range"
        min="1"
        max="100"
        bind:value={maxSize}
        class="w-full h-2 bg-slate-700 rounded-lg appearance-none cursor-pointer accent-emerald-500"
      />
      <div class="flex justify-between text-sm text-slate-400 mt-1">
        <span>1 GB</span>
        <span class="text-emerald-400 font-medium">{maxSize} GB selected</span>
        <span>100 GB</span>
      </div>
    </div>
  </div>
</div>
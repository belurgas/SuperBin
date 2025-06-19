<script lang="ts">
    import { onDestroy } from "svelte";
    import { isScanning } from "../../stores/uiStore";

    let scanProgress = 0;
    let scanInterval: number;

    const startScan = () => {
        isScanning.set(true);
        scanProgress = 0;

        scanInterval = setInterval(() => {
            scanProgress += 5;
            if (scanProgress >= 100) {
                clearInterval(scanInterval);
                setTimeout(() => isScanning.set(false), 500);
            }
        }, 100);
    };

    onDestroy(() => {
        if (scanInterval) clearInterval(scanInterval);
    });
</script>

<header class="border-b border-emerald-800/30 bg-slate-900/50 backdrop-blur-sm">
  <div class="max-w-6xl mx-auto px-3 py-1">
    <div class="flex items-center justify-between">
      <div class="flex items-center space-x-3">
        <!-- <div class="w-6 h-6 bg-gradient-to-r from-emerald-500 to-green-600 rounded-xl flex items-center justify-center">
          <span class="text-xl font-bold">SB</span>
        </div> -->
        <div>
          <h1 class="text-xl font-bold bg-gradient-to-r from-emerald-400 to-green-400 bg-clip-text text-transparent">
            SuperBin
          </h1>
          <p class="text-xs text-slate-400">Advanced System Utility</p>
        </div>
      </div>
      <div class="flex items-center space-x-4">
        <button 
          on:click={startScan}
          class="px-4 py-2 bg-emerald-600 hover:bg-emerald-700 rounded-lg transition-colors text-sm font-medium flex items-center"
          disabled={$isScanning}
        >
          {#if $isScanning}
            <svg class="animate-spin -ml-1 mr-2 h-4 w-4 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
              <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            Scanning... {scanProgress}%
          {:else}
            Quick Scan
          {/if}
        </button>
      </div>
    </div>
  </div>
</header>

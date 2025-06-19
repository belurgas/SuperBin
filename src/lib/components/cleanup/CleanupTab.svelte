<script lang="ts">
    import FileItem from "./FileItem.svelte";
    import type { RecentFile } from "../../types";

    const recentFiles: RecentFile[] = [
        {
            name: 'temp_cache_2024.tmp',
            size: '2.4 GB',
            type: 'Temporary',
            path: 'C:\\Windows\\Temp\\',
        },
        {
            name: 'old_logs.log',
            size: '856 MB',
            type: 'Log Files',
            path: 'C:\\Program Files\\App\\',
        },
        {
            name: 'duplicate_photos', 
            size: '1.2 GB', 
            type: 'Duplicates', 
            path: 'D:\\Pictures\\' 
        },
    ];
</script>

<div class="space-y-6 fade-in">
  <div class="bg-slate-800/50 backdrop-blur-sm rounded-2xl p-6 border border-emerald-800/20">
    <h3 class="text-xl font-semibold mb-6">Smart File Detection</h3>
    <div class="space-y-4">
      {#each recentFiles as file}
        <FileItem {file} />
      {/each}
    </div>
    <button
      class="w-full mt-6 py-3 bg-gradient-to-r from-emerald-600 to-green-600 rounded-xl font-semibold hover:from-emerald-700 hover:to-green-700 transition-all shadow-lg hover:shadow-emerald-900/30"
    >
      Clean Selected Files
    </button>
  </div>
</div>
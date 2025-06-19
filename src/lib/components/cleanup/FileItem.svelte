<script lang="ts">
    import type { FileItemProps, RecentFile } from "../../types";
    export let file: RecentFile;
</script>

<div class="flex items-center justify-between p-4 bg-slate-700/30 rounded-xl transition-all hover:bg-slate-700/50">
  <div class="flex items-center space-x-4">
    <div class="w-10 h-10 bg-gradient-to-r from-red-500 to-orange-500 rounded-lg flex items-center justify-center">
      <span class="text-sm">🗑️</span>
    </div>
    <div>
      <p class="font-medium">{file.name}</p>
      <p class="text-sm text-slate-400">{file.path}</p>
    </div>
  </div>
  <div class="text-right">
    <p class="font-semibold text-emerald-400">{file.size}</p>
    <p class="text-xs text-slate-400">{file.type}</p>
  </div>
</div>
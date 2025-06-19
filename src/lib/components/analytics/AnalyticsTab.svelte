<script lang="ts">
    import type { FileTypeItem, FolderItem } from "../../types";

    const fileTypes: FileTypeItem[] = [
        { type: 'Documents', size: '12.4 GB', color: 'bg-blue-500' },
        { type: 'Media', size: '45.2 GB', color: 'bg-purple-500' },
        { type: 'Applications', size: '23.1 GB', color: 'bg-green-500' },
        { type: 'System', size: '8.7 GB', color: 'bg-red-500' },
    ];

    const folders: FolderItem[] = [
        { folder: 'Program Files', size: '34.2 GB' },
        { folder: 'Users', size: '28.7 GB' },
        { folder: 'Windows', size: '15.3 GB' },
        { folder: 'Downloads', size: '9.1 GB' },
    ];

    let { class: string } = $props();
</script>

<div class="flex flex-col gap-4 mx-auto">
  <div class="flex items-center justify-between bg-slate-800/50 backdrop-blur-sm rounded-2xl p-6 border border-emerald-800/20 fade-in">
    <h3 class="text-xl font-bold">Choose the disk</h3>
    <div class="flex items-center gap-4">
      <select id="select-row" class="select px-4 py-2 rounded-lg border border-gray-300 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent">
        <option>Вариант 1</option>
        <option>Вариант 2</option>
        <option>Вариант 3</option>
      </select>
    </div>
  </div>

  <div class="bg-slate-800/50 backdrop-blur-sm rounded-2xl p-6 border border-emerald-800/20 fade-in">
    <h3 class="text-xl font-semibold mb-6">Detailed Disk Analytics</h3>
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
      <div class="space-y-4">
        <h4 class="font-semibold text-emerald-400">
          File Types Distribution
        </h4>
        <div class="space-y-3">
          {#each fileTypes as item}
            <div class="flex items-center justify-between p-3 bg-slate-700/30 rounded-lg">
              <div class="flex items-center space-x-3">
                <div class={`w-4 h-4 rounded ${item.color}`}></div>
                <span>{item.type}</span>
              </div>
              <span class="text-slate-400">{item.size}</span>
            </div>
          {/each}
        </div>
      </div>
      
      <div class="space-y-4">
        <h4 class="font-semibold text-emerald-400">
          Largest Folders
        </h4>
        <div class="space-y-3">
          {#each folders as item}
            <div class="flex items-center justify-between p-3 bg-slate-700/30 rounded-lg">
              <span>{item.folder}</span>
              <span class="text-emerald-400 font-medium">{item.size}</span>
            </div>
          {/each}
        </div>
      </div>
    </div>
  </div>
</div>
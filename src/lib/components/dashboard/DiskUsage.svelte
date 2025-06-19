<script lang="ts">
    import { diskStats } from "../../stores/diskStore";
    import ProgressBar from "../core/ProgressBar.svelte";
    import type { ProgressBarProps } from "../../types";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import type { TestDiskStat } from "../../types";

    // Типы
    type DiskInfo = {
      letter: string;
      name: string;
      total: number;
      free: number;
      used: number;
    };

    type RawDiskData = [string, string, number, number];

    let disks = $state<DiskInfo[]>([]);
    let isLoading = $state<boolean>(true);
    let error = $state<Error | null>(null);

    const fetchDosks = async (): Promise<void> => {
      isLoading = true;
      error = null;

      try {
        const result = await invoke<RawDiskData[]>("get_disks_info");

        disks = result.map(([letter, name, total, free]): DiskInfo => ({
          letter,
          name,
          total,
          free,
          used: total - free,
        }));
      } catch (err) {
        error = err instanceof Error ? err : new Error(String(err));
      } finally {
        isLoading = false;
      }
    }

    onMount(async () => {
      fetchDosks();
      
    })
</script>

<div class="bg-slate-800/50 backdrop-blur-sm rounded-2xl p-6 border border-emerald-800/20">
  <h3 class="text-xl font-semibold mb-6">Disk Usage Overview</h3>
  <div class="space-y-4">
    {#if isLoading}
      <p>Loading disk info...</p>
    {:else}
      {#each disks as disk}
        <div class="space-y-2">
          <h3 class="text-gray-500 font-bold">{disk.name}</h3>
          <div class="flex justify-between items-center">
            <span class="font-medium">{disk.letter}</span>
            <span class="text-slate-400">{Math.round(disk.used / 1024 / 1024 / 1024 * 100) / 100} GB / {Math.round(disk.total / 1024 / 1024 / 1024 * 100) / 100} GB</span>
          </div>
          <ProgressBar props={{
            value: disk.used,
            max: disk.total,
            colorClass: "from-teal-500 to-green-500"
          }} />
        </div>
      {/each}
    {/if}
  </div>
</div>
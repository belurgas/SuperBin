<script lang="ts">
    import type { StatsCardProps } from "../../types";

    let { stats, ram = 0 }: {stats: StatsCardProps, ram: number} = $props();
</script>

<div class="bg-slate-800/50 backdrop-blur-sm rounded-2xl p-6 border border-emerald-800/20">
  <div class="flex items-center justify-between mb-4">
    <h3 class="text-lg font-semibold">{stats.title}</h3>
    {#if stats.pulse}
      <div class="w-3 h-3 bg-green-500 rounded-full animate-pulse"></div>
    {:else if stats.icon}
      <span class="text-2xl">{stats.icon}</span>
    {/if}
  </div>
  <div class={`text-3xl font-bold mb-2 ${stats.color}`}>{stats.value}</div>
  <p class="text-slate-400 text-sm">{stats.description}</p>
</div>
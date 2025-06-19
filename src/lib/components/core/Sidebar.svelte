<script lang="ts">
    import { TABS } from "../../stores/uiStore";
    import { activeTab } from "../../stores/uiStore";
</script>

<div class="w-64 space-y-2">
  {#each TABS as tab}
    <button
      on:click={() => $activeTab = tab.id}
      class="w-full flex items-center space-x-3 px-4 py-3 rounded-xl transition-all group"
      class:bg-gradient-to-r={$activeTab === tab.id}
      class:from-emerald-600={$activeTab === tab.id}
      class:to-green-600={$activeTab === tab.id}
      class:shadow-lg={$activeTab === tab.id}
      class:bg-slate-800={$activeTab !== tab.id}
      class:hover:bg-slate-700={$activeTab !== tab.id}
    >
      <span class="text-lg group-hover:scale-110 transition-transform">
        {tab.icon}
      </span>
      <span class="font-medium">{tab.name}</span>
    </button>
  {/each}
</div>
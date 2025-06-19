<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import type { TestDiskStat } from "$lib/types";

  import Header from "../lib/components/core/Header.svelte";
  import Sidebar from "../lib/components/core/Sidebar.svelte";
  import StatsGrid from "../lib/components/dashboard/StatsGrid.svelte";
  import DiskUsage from "../lib/components/dashboard/DiskUsage.svelte";
  import CleanupTab from "../lib/components/cleanup/CleanupTab.svelte";
  import RecycleSettings from "../lib/components/cleanup/RecycleSettings.svelte";
  import AnalyticsTab from "../lib/components/analytics/AnalyticsTab.svelte";

  import { activeTab } from "../lib/stores/uiStore";
  import { onDestroy, onMount } from "svelte";
  import { getCurrentWindow } from "@tauri-apps/api/window";

  // Animation
  let animate = false;

  $effect(() => {
    if (activeTab) {
      animate = true;

      // Устанавливаем таймер для отключения анимации
      const timer = setTimeout(() => {
        animate = false;
      }, 300);

      // Очистка таймера при уничтожении компонента
      onDestroy(() => clearTimeout(timer));
    }
  });

  onMount(() => {
    const appWindow = getCurrentWindow();

    document
      .getElementById('titlebar-minimize')
      ?.addEventListener('click', () => appWindow.minimize());
    document
      .getElementById('titlebar-close')
      ?.addEventListener('click', () => appWindow.close());
  })
</script>

<main>
  <div class="min-h-screen pt-6 bg-gradient-to-br from-slate-900 via-slate-800 to-emerald-900 text-white">
    <div data-tauri-drag-region class="titlebar border-emerald-800/30 bg-slate-900/50 backdrop-blur-sm pr-2">
        <div class="titlebar-button bg-yellow-500 hover:animate-pulse hover:bg-amber-600" id="titlebar-minimize">
            
        </div>
        <div class="titlebar-button bg-red-600 hover:animate-pulse hover:bg-red-700" id="titlebar-close">
        </div>
    </div>
    <Header />
  
  <div class="max-w-7xl mx-auto px-4 sm:px-6 py-6">
    <div class="flex flex-col md:flex-row gap-6">
      <Sidebar />
      
      <div class="flex-1 space-y-6">
        {#if $activeTab === 'dashboard'}
          <StatsGrid />
          <DiskUsage />
          
        {:else if $activeTab === 'cleanup'}
          <CleanupTab />
          
        {:else if $activeTab === 'recycle'}
          <RecycleSettings />
          
        {:else if $activeTab === 'analytics'}
          <AnalyticsTab class={animate ? 'fade-in' : ''} />
        {/if}
      </div>
    </div>
  </div>
</div>
</main>

<style>
  :root {
    background: transparent;
    background-blend-mode: overlay;
    font-synthesis: none;
    overflow: hidden;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  .fade-in {
    animation: fadeIn 1s ease-in forwards;
    opacity: 0;
  }
  
  @keyframes fadeIn {
    to { opacity: 1; transform: translateY(0); }
  }
  
  /* Кастомный ползунок */
  input[type=range]::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 18px;
    height: 18px;
    border-radius: 50%;
    background: #10b981;
    cursor: pointer;
    border: 2px solid #0f172a;
    box-shadow: 0 0 5px rgba(16, 185, 129, 0.5);
  }

  .titlebar {
        height: 24px;
        
        user-select: none;
        display: flex;
        justify-content: flex-end;
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        z-index: 100;
    }
    .titlebar-button {
        display: inline-flex;
        justify-content: center;
        align-items: center;
        width: 14px;
        height: 14px;
        border-radius: 50%;
        padding: 5px;
        margin-top: 5px;
        margin-right: 5px;

        user-select: none;
        -webkit-user-select: none;
    }
</style>

<!-- <style>

:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style> -->

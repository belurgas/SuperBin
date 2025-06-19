<script lang="ts">
    import StatsCard from "../core/StatsCard.svelte";
    import type { StatsCardProps } from "../../types";
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from '@tauri-apps/api/event';

    let used_memory = $state<number>(0);

    listen('memory-update', (event) => {

        let pl: any = event.payload;
        used_memory = Math.round(pl / 1024 / 1024 / 1024 * 100) / 100; // Получаем занятость памяти
        console.log(`Used Memory: ${used_memory} GB`);
        // Обновите интерфейс пользователя по мере необходимости
    });


    const stats = $derived([
        { 
            title: 'System Health', 
            value: '98%', 
            description: 'Optimal Performance',
            pulse: true,
            color: 'text-emerald-400'
        },
        { 
            title: 'RAM Usage', 
            value: `${used_memory} / 16 GB`, 
            description: 'This month',
            icon: '💾',
            color: 'text-green-400'
        },
        { 
            title: 'Files Processed', 
            value: '1,247', 
            description: 'Last scan',
            icon: '📁',
            color: 'text-teal-400'
        }
    ]);

    onMount(async () => {
        let temp = await invoke("get_temperatures");
        console.log(temp);
    });
</script>

<div class="grid grid-cols-1 md:grid-cols-3 gap-6">
  {#each stats as stat}
    <StatsCard stats={stat} ram={used_memory}/>
  {/each}
</div>
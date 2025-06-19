import { writable } from "svelte/store";
import { browser } from "$app/environment";
import type { DiskStat } from "../types";

let local = localStorage.getItem('diskStats') || "";

const initialDiskStats: DiskStat[] = [
    { name: 'System (C:)', used: 85, total: 500, color: 'from-emerald-500 to-green-600' },
    { name: 'Data (D:)', used: 120, total: 1000, color: 'from-green-500 to-emerald-600' },
    { name: 'Backup (E:)', used: 45, total: 250, color: 'from-teal-500 to-green-500' },
]

export const diskStats = writable<DiskStat[]>(
    browser ? JSON.parse(localStorage.getItem('diskStats') || JSON.stringify(initialDiskStats)) : initialDiskStats
);

diskStats.subscribe(stats => {
    if (browser) {
        localStorage.setItem('diskStats', JSON.stringify(stats))
    }
});

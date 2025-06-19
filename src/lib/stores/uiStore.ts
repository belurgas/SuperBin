import { writable } from "svelte/store";
import type { Tab } from "../types";

export const activeTab = writable('dashboard');
export const isScanning = writable(false);

// loc
export const TABS: Tab[] = [
    { id: 'dashboard', name: 'Dashboard', icon: '📊' },
    { id: 'cleanup', name: 'Smart Cleanup', icon: '🧹' },
    { id: 'recycle', name: 'Recycle Settings', icon: '🗑️' },
    { id: 'analytics', name: 'Disk Analytics', icon: '📈' },
]
import { writable } from 'svelte/store';

export const currentTheme = writable<'dark' | 'light' | 'system'>('dark');

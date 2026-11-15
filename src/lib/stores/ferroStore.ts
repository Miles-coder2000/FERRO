import { writable } from 'svelte/store';

export const ferroStore = writable({
  activeSector: 'HOME',
  searchQuery: '',
});
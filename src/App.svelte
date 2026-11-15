<script lang="ts">
  import { onMount } from 'svelte';
  import { ferroStore } from '$lib/stores/ferroStore.svelte';
  import TopBar from '$lib/components/TopBar.svelte';
  import Sidebar from '$lib/components/Sidebar.svelte';
  import ArmoryGrid from '$lib/components/ArmoryGrid.svelte';
  import FolderMatrix from '$lib/components/FolderMatrix.svelte';
  import AppDock from '$lib/components/AppDock.svelte';
  import ContextsPanel from '$lib/components/ContextsPanel.svelte';

  let searchInputEl: HTMLInputElement | null = null;

  onMount(async () => {
    try {
      await ferroStore.init();
    } catch (err) {
      console.error('Failed to initialize store:', err);
    }
  });

  function handleGlobalKeydown(e: KeyboardEvent) {
    // Ctrl+K / Cmd+K: focus search
    if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
      e.preventDefault();
      searchInputEl?.focus();
    }
    // Escape: clear search first, then hide on second press
    if (e.key === 'Escape') {
      if (ferroStore.searchQuery.trim().length > 0) {
        e.preventDefault();
        ferroStore.resetSearch();
      }
    }
  }
</script>

<svelte:window onkeydown={handleGlobalKeydown} />

<div class="h-full w-full bg-[#0B0D10] text-[#F1F5F9] overflow-hidden select-none antialiased">
  <!-- Top Bar - draggable region -->
  <TopBar bind:searchInputEl={searchInputEl} />
  <div class="data-tauri-drag-region" style="-webkit-app-region: drag;"></div>

  <!-- Main Layout: Sidebar + Content -->
  <div class="flex flex-1 h-full overflow-hidden">
    <Sidebar />

    <!-- Main Content Area -->
    <main class="flex-1 h-full overflow-y-auto bg-[#0B0D10] p-4">
      {#key ferroStore.activeSector}
        {#if ferroStore.activeSector === 'HOME'}
          <div class="grid grid-cols-1 lg:grid-cols-3 gap-4">
            <div class="lg:col-span-2">
              <ArmoryGrid compact={false} />
            </div>
            <div>
              <FolderMatrix compact={false} />
            </div>
          </div>
        {:else if ferroStore.activeSector === 'FOLDERS'}
          <FolderMatrix compact={false} />
        {:else if ferroStore.activeSector === 'ARMORY'}
          <ArmoryGrid compact={false} />
        {:else if ferroStore.activeSector === 'STACK'}
          <AppDock compact={false} />
        {:else if ferroStore.activeSector === 'CONTEXTS'}
          <ContextsPanel />
        {/if}
      {/key}
    </main>
  </div>

  <!-- Bottom Engine Status Bar -->
  <div class="h-10 border-t border-[#262C36] bg-[#0B0D10]/80 backdrop-blur flex items-center justify-between px-4 text-[10px] font-mono text-[#8A94A6]">
    <span class="flex items-center gap-1.5">
      <span class="w-1.5 h-1.5 rounded-full bg-[#E86A24] animate-pulse"></span>
      ENGINE: RUST &bull; TAURI 2.0
    </span>
    <span>
      {ferroStore.telemetry.indexed_items_count} indexed &bull;
      {ferroStore.telemetry.cpu_usage_percent.toFixed(1)}% CPU &bull;
      {ferroStore.telemetry.ram_usage_mb.toFixed(0)} MB RAM
    </span>
  </div>
</div>

<style>
  :global([data-tauri-drag-region]) {
    -webkit-app-region: drag;
  }
  :global([data-tauri-drag-region] button),
  :global([data-tauri-drag-region] input),
  :global([data-tauri-drag-region] a) {
    -webkit-app-region: no-drag;
  }
</style>

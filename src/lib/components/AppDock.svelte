<script lang="ts">
  import { ferroStore } from '$lib/stores/ferroStore.svelte';
  import { resolveAssetUrl } from '$lib/utils';
  import type { AppEntry } from '$lib/types/ferro';
  import { Layers, Shield, FolderOpen, Play, AppWindow } from 'lucide-svelte';

  let { compact = false }: { compact?: boolean } = $props();

  let activeCategory = $state<string>('ALL');
  let contextMenu = $state<{ x: number; y: number; app: AppEntry | null }>({
    x: 0,
    y: 0,
    app: null,
  });

  const categories = ['ALL', 'Developer', 'Creative', 'Comms & Web', 'System & Utility'];

  let filteredApps = $derived.by(() => {
    let list = ferroStore.apps;
    if (activeCategory !== 'ALL') {
      list = list.filter((a) => a.category.toLowerCase() === activeCategory.toLowerCase());
    }
    return compact ? list.slice(0, 18) : list;
  });

  function handleContextMenu(e: MouseEvent, app: AppEntry) {
    e.preventDefault();
    contextMenu = {
      x: Math.min(e.clientX, window.innerWidth - 180),
      y: Math.min(e.clientY, window.innerHeight - 150),
      app,
    };
  }

  function closeContextMenu() {
    contextMenu = { x: 0, y: 0, app: null };
  }
</script>

<svelte:window onclick={closeContextMenu} />

<section class="flex flex-col gap-3">
  <div class="flex items-center justify-between">
    <div class="flex items-center gap-2.5">
      <Layers class="w-4 h-4 text-[#FF9E44]" />
      <h2 class="font-display text-xs font-bold tracking-[0.14em] uppercase text-[#F1F5F9]">
        APPLICATION MATRIX
      </h2>
      <span class="font-mono text-[10px] px-1.5 py-0.5 rounded bg-[#12161C] border border-[#262C36] text-[#8A94A6]">
        {ferroStore.apps.length} APPS
      </span>
    </div>

    {#if !compact}
      <div class="flex items-center gap-1.5">
        {#each categories as cat}
          <button
            onclick={() => (activeCategory = cat)}
            class="px-2 py-1 rounded text-[10px] font-mono transition-colors cursor-pointer border {activeCategory === cat
              ? 'bg-[#E86A24]/15 border-[#E86A24] text-[#FF9E44]'
              : 'bg-[#12161C] border-[#262C36] text-[#8A94A6] hover:text-[#F1F5F9]'}"
          >
            {cat}
          </button>
        {/each}
      </div>
    {:else if ferroStore.apps.length > 18}
      <button
        onclick={() => (ferroStore.activeSector = 'STACK')}
        class="text-[11px] font-mono text-[#FF9E44] hover:underline cursor-pointer"
      >
        EXPAND ({ferroStore.apps.length}) →
      </button>
    {/if}
  </div>

  {#if filteredApps.length === 0}
    <div class="ferro-panel rounded-xl p-6 text-center text-xs font-mono text-[#8A94A6]">
      No applications in this category. Rescan to index Start Menu applications.
    </div>
  {:else}
    <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-3">
      {#each filteredApps as app (app.id)}
        {@const iconUrl = resolveAssetUrl(app.icon_path)}
        <button
          onclick={() => ferroStore.launchTarget(app.path)}
          oncontextmenu={(e) => handleContextMenu(e, app)}
          title="{app.name} (Right-click for options)"
          class="group ferro-panel-interactive p-3 rounded-xl flex flex-col items-center gap-2 text-center cursor-pointer border border-[#262C36] hover:border-[#FF9E44]/70 transition-all"
        >
          <div
            class="relative w-12 h-12 flex items-center justify-center p-0.5 rounded-xl bg-gradient-to-br from-[#262C36] via-[#12161C] to-[#0B0D10] border border-[#262C36] group-hover:border-[#FF9E44] group-hover:shadow-[0_0_12px_rgba(255,158,68,0.25)] transition-all"
          >
            {#if iconUrl}
              <img src={iconUrl} alt={app.name} class="w-8 h-8 object-contain" />
            {:else}
              <AppWindow class="w-6 h-6 text-[#8A94A6] group-hover:text-[#FF9E44]" />
            {/if}
          </div>

          <div class="flex flex-col items-center w-full min-w-0">
            <span class="font-display text-xs font-semibold text-[#F1F5F9] truncate w-full group-hover:text-[#FF9E44] transition-colors">
              {app.name}
            </span>
            <span class="font-mono text-[9px] text-[#8A94A6] truncate w-full">
              {app.category}
            </span>
          </div>
        </button>
      {/each}
    </div>
  {/if}
</section>

{#if contextMenu.app}
  <div
    style="top: {contextMenu.y}px; left: {contextMenu.x}px;"
    class="fixed z-50 w-44 ferro-panel rounded-lg p-1.5 flex flex-col gap-1 border border-[#FF9E44]/40 shadow-2xl"
  >
    <div class="px-2 py-1 text-[10px] font-mono text-[#8A94A6] border-b border-[#262C36] truncate">
      {contextMenu.app.name}
    </div>
    <button
      onclick={() => {
        if (contextMenu.app) ferroStore.launchTarget(contextMenu.app.path);
        closeContextMenu();
      }}
      class="flex items-center gap-2 px-2 py-1.5 rounded text-xs font-mono text-[#F1F5F9] hover:bg-[#12161C] hover:text-[#FF9E44] text-left cursor-pointer"
    >
      <Play class="w-3.5 h-3.5 text-[#E86A24]" />
      Launch App
    </button>
    <button
      onclick={() => {
        if (contextMenu.app) ferroStore.launchTarget(contextMenu.app.path, [], true);
        closeContextMenu();
      }}
      class="flex items-center gap-2 px-2 py-1.5 rounded text-xs font-mono text-[#F1F5F9] hover:bg-[#12161C] hover:text-[#FF9E44] text-left cursor-pointer"
    >
      <Shield class="w-3.5 h-3.5 text-[#FF9E44]" />
      Run as Admin
    </button>
    <button
      onclick={() => {
        if (contextMenu.app) ferroStore.revealLocation(contextMenu.app.path);
        closeContextMenu();
      }}
      class="flex items-center gap-2 px-2 py-1.5 rounded text-xs font-mono text-[#F1F5F9] hover:bg-[#12161C] hover:text-[#38BDF8] text-left cursor-pointer"
    >
      <FolderOpen class="w-3.5 h-3.5 text-[#38BDF8]" />
      Open Location
    </button>
  </div>
{/if}

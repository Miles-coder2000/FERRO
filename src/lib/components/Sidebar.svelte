<script lang="ts">
  import { ferroStore } from '$lib/stores/ferroStore.svelte';
  import type { SectorTab } from '$lib/types/ferro';
  import { Flame, FolderKanban, Gamepad2, Layers, Cpu } from 'lucide-svelte';

  const sectors: { id: SectorTab; label: string; sub: string; icon: typeof Flame }[] = [
    { id: 'HOME', label: 'HOME', sub: 'The Forge', icon: Flame },
    { id: 'FOLDERS', label: 'FOLDERS', sub: 'Workspaces', icon: FolderKanban },
    { id: 'ARMORY', label: 'ARMORY', sub: 'Games', icon: Gamepad2 },
    { id: 'STACK', label: 'STACK', sub: 'Apps', icon: Layers },
    { id: 'CONTEXTS', label: 'CONTEXTS', sub: 'Modes & Config', icon: Cpu },
  ];
</script>

<aside
  class="w-[220px] shrink-0 border-r border-[#262C36] bg-[#0B0D10]/80 backdrop-blur-xl flex flex-col justify-between p-3.5 select-none"
>
  <div class="flex flex-col gap-1.5">
    <div class="px-2.5 pb-2 pt-1 flex items-center justify-between">
      <span class="font-display text-[11px] font-semibold tracking-[0.16em] text-[#8A94A6] uppercase">
        SECTORS
      </span>
      <span class="h-1.5 w-1.5 rounded-full bg-[#E86A24] shadow-[0_0_8px_#E86A24]"></span>
    </div>

    {#each sectors as sector}
      {@const isActive = ferroStore.activeSector === sector.id}
      {@const Icon = sector.icon}
      <button
        onclick={() => {
          ferroStore.activeSector = sector.id;
          ferroStore.resetSearch();
        }}
        class="group relative flex items-center gap-3 px-3 py-2.5 rounded-lg text-left transition-all duration-150 cursor-pointer border {isActive
          ? 'bg-[#12161C] border-[#FF9E44]/70 shadow-[inset_0_0_12px_rgba(232,106,36,0.18),0_0_14px_rgba(232,106,36,0.12)]'
          : 'bg-transparent border-transparent hover:bg-[#12161C]/70 hover:border-[#262C36]'}"
      >
        {#if isActive}
          <span
            class="absolute left-0 top-2 bottom-2 w-1 rounded-r bg-gradient-to-b from-[#FF9E44] to-[#E86A24]"
          ></span>
        {/if}

        <div
          class="p-1.5 rounded-md border transition-colors {isActive
            ? 'bg-[#E86A24]/15 border-[#E86A24]/50 text-[#FF9E44]'
            : 'bg-[#12161C] border-[#262C36] text-[#8A94A6] group-hover:text-[#F1F5F9]'}"
        >
          <Icon class="w-4 h-4" />
        </div>

        <div class="flex flex-col min-w-0">
          <span
            class="font-display text-xs font-bold tracking-[0.1em] uppercase {isActive
              ? 'text-[#F1F5F9]'
              : 'text-[#8A94A6] group-hover:text-[#F1F5F9]'}"
          >
            {sector.label}
          </span>
          <span class="text-[10px] text-[#8A94A6]/80 truncate">{sector.sub}</span>
        </div>
      </button>
    {/each}
  </div>

  <!-- Sidebar Footer Stats -->
  <div class="ferro-panel rounded-lg p-3 flex flex-col gap-2">
    <div class="flex items-center justify-between text-[10px] font-mono text-[#8A94A6]">
      <span>INDEXED</span>
      <span class="text-[#38BDF8] font-semibold">
        {ferroStore.telemetry.indexed_items_count || ferroStore.apps.length + ferroStore.games.length}
      </span>
    </div>
    <div class="flex items-center justify-between text-[10px] font-mono text-[#8A94A6]">
      <span>HOTKEY</span>
      <span class="px-1.5 py-0.5 rounded bg-[#0B0D10] border border-[#262C36] text-[#F1F5F9]">
        ALT+SPACE
      </span>
    </div>
  </div>
</aside>

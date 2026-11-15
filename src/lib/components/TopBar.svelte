<script lang="ts">
  import { ferroStore } from '$lib/stores/ferroStore.svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { isTauriRuntime } from '$lib/utils';
  import logoImg from '$assets/logo.png';
  import {
    Search, Terminal, FolderSearch, Gamepad2,
    Minus, Pin, X, RefreshCw, Cpu, MemoryStick, Zap,
  } from 'lucide-svelte';

  let { searchInputEl = $bindable<HTMLInputElement | null>(null) } = $props();

  function handleHeaderMouseDown(e: MouseEvent) {
    if (!isTauriRuntime()) return;
    if ((e.target as HTMLElement).closest('button, input')) return;
    getCurrentWindow().startDragging();
  }

  let modeMeta = $derived.by(() => {
    switch (ferroStore.searchMode) {
      case 'cli': return { label: 'RUST CLI', cls: 'text-[#FF9E44] border-[#FF9E44]/50 bg-[#FF9E44]/10' };
      case 'folder': return { label: 'DIR NAV', cls: 'text-[#38BDF8] border-[#38BDF8]/50 bg-[#38BDF8]/10' };
      case 'game': return { label: 'ARMORY', cls: 'text-[#E86A24] border-[#E86A24]/50 bg-[#E86A24]/10' };
      default: return { label: 'OMNI', cls: 'text-[#8A94A6] border-[#262C36] bg-[#0B0D10]/70' };
    }
  });
</script>

<header
  onmousedown={handleHeaderMouseDown}
  class="flex flex-col border-b border-[#262C36] bg-[#0B0D10]/90 backdrop-blur-xl select-none"
>
  <div class="flex items-center justify-between px-4 py-2 border-b border-[#262C36]/70">
    <div class="flex items-center gap-3">
      <img
        src={logoImg}
        alt="FERRO Emblem"
        class="h-6 w-6 rounded-md object-cover border border-[#E86A24]/60 shadow-[0_0_10px_rgba(232,106,36,0.3)]"
      />
      <div class="flex items-baseline gap-2">
        <span class="font-display text-sm font-bold tracking-[0.14em] text-[#F1F5F9]">FERRO</span>
        <span class="font-mono text-[10px] tracking-widest text-[#8A94A6] uppercase">SYS.HUD // V0.1</span>
      </div>
    </div>

    <div class="flex items-center gap-4">
      <div class="hidden md:flex items-center gap-3 px-3 py-1 rounded bg-[#12161C]/90 border border-[#262C36] font-mono text-[11px] text-[#8A94A6]">
        <span class="flex items-center gap-1.5 text-[#38BDF8]">
          <Cpu class="w-3.5 h-3.5" />
          CPU: {ferroStore.telemetry.cpu_usage_percent.toFixed(2)}%
        </span>
        <span class="text-[#262C36]">|</span>
        <span class="flex items-center gap-1.5 text-[#F1F5F9]">
          <MemoryStick class="w-3.5 h-3.5 text-[#8A94A6]" />
          RAM: {ferroStore.telemetry.ram_usage_mb.toFixed(1)} MB
        </span>
        <span class="text-[#262C36]">|</span>
        <span class="flex items-center gap-1.5 text-[#FF9E44]">
          <Zap class="w-3.5 h-3.5" />
          Latency: {ferroStore.telemetry.last_launch_latency_ms.toFixed(2)} ms
        </span>
      </div>

      <div class="flex items-center gap-1">
        <button
          onclick={() => ferroStore.rescanAll()}
          title="Rescan Apps & Games"
          class="p-1.5 rounded text-[#8A94A6] hover:text-[#FF9E44] hover:bg-[#12161C] transition-colors cursor-pointer"
        >
          <RefreshCw class="w-3.5 h-3.5 {ferroStore.isScanning ? 'animate-spin text-[#E86A24]' : ''}" />
        </button>
        <button
          onclick={() => ferroStore.togglePinOnTop()}
          title="Pin Window to Top"
          class="p-1.5 rounded transition-colors cursor-pointer {ferroStore.isPinnedOnTop
            ? 'text-[#FF9E44] bg-[#E86A24]/15'
            : 'text-[#8A94A6] hover:text-[#F1F5F9] hover:bg-[#12161C]'}"
        >
          <Pin class="w-3.5 h-3.5" />
        </button>
        <button
          onclick={() => ferroStore.minimizeWindow()}
          title="Minimize"
          class="p-1.5 rounded text-[#8A94A6] hover:text-[#F1F5F9] hover:bg-[#12161C] transition-colors cursor-pointer"
        >
          <Minus class="w-3.5 h-3.5" />
        </button>
        <button
          onclick={() => ferroStore.closeWindow()}
          title="Hide (Alt+Space)"
          class="p-1.5 rounded text-[#8A94A6] hover:text-[#E86A24] hover:bg-[#E86A24]/15 transition-colors cursor-pointer"
        >
          <X class="w-3.5 h-3.5" />
        </button>
      </div>
    </div>
  </div>

  <div class="px-5 py-3 flex items-center justify-center">
    <div
      class="relative w-full max-w-3xl flex items-center gap-3 px-4 py-2.5 rounded-lg bg-[#12161C]/90 border transition-all duration-150 {ferroStore.isSearchOpen
        ? 'border-[#E86A24] shadow-[0_0_20px_rgba(232,106,36,0.2)]'
        : 'border-[#262C36] focus-within:border-[#E86A24]/80 focus-within:shadow-[0_0_16px_rgba(232,106,36,0.15)]'}"
    >
      {#if ferroStore.searchMode === 'cli'}
        <Terminal class="w-4 h-4 text-[#FF9E44] shrink-0" />
      {:else if ferroStore.searchMode === 'folder'}
        <FolderSearch class="w-4 h-4 text-[#38BDF8] shrink-0" />
      {:else if ferroStore.searchMode === 'game'}
        <Gamepad2 class="w-4 h-4 text-[#E86A24] shrink-0" />
      {:else}
        <Search class="w-4 h-4 text-[#8A94A6] shrink-0" />
      {/if}

      <input
        bind:this={searchInputEl}
        bind:value={ferroStore.searchQuery}
        oninput={() => { ferroStore.selectedIndex = 0; }}
        type="text"
        placeholder="Search anything or launch with Rust CLI...  (> shell, / folder, @ games)"
        class="w-full bg-transparent text-sm font-mono text-[#F1F5F9] placeholder-[#8A94A6]/65 focus:outline-none"
        spellcheck="false"
        autocomplete="off"
      />

      <span class="px-2 py-0.5 rounded border font-mono text-[10px] tracking-wider shrink-0 {modeMeta.cls}">
        {modeMeta.label}
      </span>

      {#if ferroStore.searchQuery}
        <button
          onclick={() => ferroStore.resetSearch()}
          class="text-[#8A94A6] hover:text-[#F1F5F9] text-xs font-mono cursor-pointer"
        >
          ESC
        </button>
      {/if}
    </div>
  </div>
</header>

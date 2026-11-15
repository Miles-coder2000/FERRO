<script lang="ts">
  import { ferroStore } from '$lib/stores/ferroStore.svelte';
  import { formatBytes } from '$lib/utils';
  import {
    FolderKanban, Folder, Monitor, FileText,
    Download, Code2, ExternalLink, Plus, Trash2,
  } from 'lucide-svelte';

  let { compact = false }: { compact?: boolean } = $props();
  let showAddForm = $state(false);
  let newPath = $state('');
  let newName = $state('');

  function getFolderIcon(iconName?: string) {
    switch (iconName) {
      case 'Monitor': return Monitor;
      case 'FileText': return FileText;
      case 'Download': return Download;
      case 'Code2': return Code2;
      default: return Folder;
    }
  }

  async function handlePinSubmit(e: SubmitEvent) {
    e.preventDefault();
    if (!newPath.trim()) return;
    await ferroStore.pinFolder(newPath.trim(), newName.trim() || undefined);
    newPath = '';
    newName = '';
    showAddForm = false;
  }

  let displayFolders = $derived(compact ? ferroStore.folders.slice(0, 6) : ferroStore.folders);
</script>

<section class="flex flex-col gap-3">
  <div class="flex items-center justify-between">
    <div class="flex items-center gap-2.5">
      <FolderKanban class="w-4 h-4 text-[#38BDF8]" />
      <h2 class="font-display text-xs font-bold tracking-[0.14em] uppercase text-[#F1F5F9]">
        ACTIVE SYSTEM WORKSPACES
      </h2>
      <span class="font-mono text-[10px] px-1.5 py-0.5 rounded bg-[#12161C] border border-[#262C36] text-[#8A94A6]">
        {ferroStore.folders.length} PINNED
      </span>
    </div>

    <div class="flex items-center gap-2">
      <button
        onclick={() => (showAddForm = !showAddForm)}
        class="flex items-center gap-1.5 px-2.5 py-1 rounded text-[11px] font-mono border border-[#262C36] bg-[#12161C] text-[#F1F5F9] hover:border-[#38BDF8] transition-colors cursor-pointer"
      >
        <Plus class="w-3.5 h-3.5 text-[#38BDF8]" />
        PIN DIRECTORY
      </button>
      {#if compact && ferroStore.folders.length > 6}
        <button
          onclick={() => (ferroStore.activeSector = 'FOLDERS')}
          class="text-[11px] font-mono text-[#38BDF8] hover:underline cursor-pointer"
        >
          VIEW ALL →
        </button>
      {/if}
    </div>
  </div>

  {#if showAddForm}
    <form onsubmit={handlePinSubmit} class="ferro-panel rounded-lg p-3 flex flex-wrap items-center gap-2.5">
      <input
        bind:value={newName}
        placeholder="Folder Label"
        class="px-3 py-1.5 rounded bg-[#0B0D10] border border-[#262C36] text-xs font-mono text-[#F1F5F9] focus:border-[#38BDF8] focus:outline-none flex-1 min-w-[140px]"
      />
      <input
        bind:value={newPath}
        placeholder="Folder Path (e.g. D:\Projects)"
        class="px-3 py-1.5 rounded bg-[#0B0D10] border border-[#262C36] text-xs font-mono text-[#F1F5F9] focus:border-[#38BDF8] focus:outline-none flex-[2] min-w-[200px]"
      />
      <button
        type="submit"
        class="px-3 py-1.5 rounded bg-[#38BDF8] hover:bg-[#38BDF8]/80 text-[#0B0D10] font-display text-xs font-bold uppercase cursor-pointer"
      >
        PIN
      </button>
    </form>
  {/if}

  {#if displayFolders.length === 0}
    <div class="ferro-panel rounded-xl p-6 text-center text-xs font-mono text-[#8A94A6]">
      No pinned folders yet. Pin workspace directories to enable real-time file watching.
    </div>
  {:else}
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
      {#each displayFolders as folder (folder.id)}
        {@const Icon = getFolderIcon(folder.icon_name)}
        <div class="group ferro-panel-interactive relative p-3.5 rounded-xl flex flex-col justify-between gap-3 border border-[#262C36] hover:border-[#38BDF8]/60 transition-all">
          <div class="flex items-start justify-between gap-2">
            <div class="flex items-center gap-2.5 min-w-0">
              <div class="p-2 rounded-lg bg-[#0B0D10] border border-[#262C36] text-[#38BDF8] shrink-0">
                <Icon class="w-4 h-4" />
              </div>
              <div class="flex flex-col min-w-0">
                <span class="font-display text-xs font-bold tracking-wider text-[#F1F5F9] truncate group-hover:text-[#38BDF8] transition-colors">
                  {folder.name}
                </span>
                <span class="font-mono text-[10px] text-[#8A94A6] truncate" title={folder.path}>
                  {folder.path}
                </span>
              </div>
            </div>

            <button
              onclick={() => ferroStore.unpinFolder(folder.id)}
              title="Unpin folder"
              class="opacity-0 group-hover:opacity-100 p-1 rounded text-[#8A94A6] hover:text-[#E86A24] transition-all cursor-pointer"
            >
              <Trash2 class="w-3.5 h-3.5" />
            </button>
          </div>

          <div class="flex items-center justify-between pt-2 border-t border-[#262C36]/50">
            <div class="flex items-center gap-2">
              <span class="w-1.5 h-1.5 rounded-full bg-[#38BDF8] shadow-[0_0_6px_#38BDF8]"></span>
              <span class="font-mono text-[11px] text-[#F1F5F9] font-semibold">{folder.file_count} files</span>
              {#if folder.total_size_bytes > 0}
                <span class="font-mono text-[10px] text-[#8A94A6]">({formatBytes(folder.total_size_bytes)})</span>
              {/if}
            </div>

            <button
              onclick={() => ferroStore.openFolder(folder.path)}
              class="flex items-center gap-1 text-[10px] font-mono text-[#8A94A6] hover:text-[#38BDF8] transition-colors cursor-pointer"
            >
              <span>OPEN</span>
              <ExternalLink class="w-3 h-3" />
            </button>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</section>

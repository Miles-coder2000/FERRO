<script lang="ts">
  import { ferroStore } from '$lib/stores/ferroStore.svelte';
  import type { ContextProfile } from '$lib/types/ferro';
  import { Cpu, Play, Plus, Trash2, Check, RefreshCw } from 'lucide-svelte';

  let showCreateModal = $state(false);
  let newName = $state('');
  let newDesc = $state('');
  let newShortcut = $state('');
  let selectedAppIds = $state<string[]>([]);
  let selectedFolderPaths = $state<string[]>([]);

  function toggleApp(id: string) {
    selectedAppIds = selectedAppIds.includes(id)
      ? selectedAppIds.filter((x) => x !== id)
      : [...selectedAppIds, id];
  }

  function toggleFolder(path: string) {
    selectedFolderPaths = selectedFolderPaths.includes(path)
      ? selectedFolderPaths.filter((x) => x !== path)
      : [...selectedFolderPaths, path];
  }

  async function handleCreateContext(e: SubmitEvent) {
    e.preventDefault();
    if (!newName.trim()) return;

    const profile: ContextProfile = {
      id: `ctx-${Date.now()}`,
      name: newName.trim(),
      description: newDesc.trim() || 'Custom FERRO environment',
      app_ids: selectedAppIds,
      folder_paths: selectedFolderPaths,
      shortcut_key: newShortcut.trim() || undefined,
    };

    await ferroStore.saveContext(profile);
    newName = '';
    newDesc = '';
    newShortcut = '';
    selectedAppIds = [];
    selectedFolderPaths = [];
    showCreateModal = false;
  }
</script>

<div class="flex flex-col gap-5">
  <div class="flex items-center justify-between">
    <div class="flex items-center gap-2.5">
      <Cpu class="w-4 h-4 text-[#FF9E44]" />
      <h2 class="font-display text-xs font-bold tracking-[0.14em] uppercase text-[#F1F5F9]">
        OPERATION CONTEXTS & MODES
      </h2>
    </div>

    <button
      onclick={() => (showCreateModal = !showCreateModal)}
      class="flex items-center gap-1.5 px-3 py-1.5 rounded text-xs font-mono border border-[#E86A24] bg-[#E86A24]/10 text-[#FF9E44] hover:bg-[#E86A24]/20 transition-colors cursor-pointer"
    >
      <Plus class="w-3.5 h-3.5" />
      NEW CONTEXT PROFILE
    </button>
  </div>
</div>

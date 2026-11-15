<script lang="ts">
  import { ferroStore } from '$lib/stores/ferroStore.svelte';
  import { resolveAssetUrl } from '$lib/utils';
  import { Gamepad2, Play, Plus } from 'lucide-svelte';

  let { compact = false }: { compact?: boolean } = $props();

  let showAddForm = $state(false);
  let newTitle = $state('');
  let newPath = $state('');

  let displayGames = $derived(compact ? ferroStore.games.slice(0, 6) : ferroStore.games);

  async function handleAddGame(e: SubmitEvent) {
    e.preventDefault();
    if (!newTitle.trim() || !newPath.trim()) return;
    await ferroStore.addCustomGame(newTitle.trim(), newPath.trim(), 'Local');
    newTitle = '';
    newPath = '';
    showAddForm = false;
  }
</script>

<section class="flex flex-col gap-3">
  <div class="flex items-center justify-between">
    <div class="flex items-center gap-2.5">
      <Gamepad2 class="w-4 h-4 text-[#E86A24]" />
      <h2 class="font-display text-xs font-bold tracking-[0.14em] uppercase text-[#F1F5F9]">
        QUICK ARMORY
      </h2>
      <span class="font-mono text-[10px] px-1.5 py-0.5 rounded bg-[#12161C] border border-[#262C36] text-[#8A94A6]">
        {ferroStore.games.length} TITLES
      </span>
    </div>

    <div class="flex items-center gap-2">
      {#if !compact}
        <button
          onclick={() => (showAddForm = !showAddForm)}
          class="flex items-center gap-1.5 px-2.5 py-1 rounded text-[11px] font-mono border border-[#262C36] bg-[#12161C] text-[#F1F5F9] hover:border-[#E86A24] transition-colors cursor-pointer"
        >
          <Plus class="w-3.5 h-3.5 text-[#E86A24]" />
          ADD EXECUTABLE
        </button>
      {:else if ferroStore.games.length > 6}
        <button
          onclick={() => (ferroStore.activeSector = 'ARMORY')}
          class="text-[11px] font-mono text-[#FF9E44] hover:underline cursor-pointer"
        >
          VIEW ALL →
        </button>
      {/if}
    </div>
  </div>

  {#if showAddForm}
    <form
      onsubmit={handleAddGame}
      class="ferro-panel rounded-lg p-3 flex flex-wrap items-center gap-2.5"
    >
      <input
        bind:value={newTitle}
        placeholder="Game Title"
        class="px-3 py-1.5 rounded bg-[#0B0D10] border border-[#262C36] text-xs font-mono text-[#F1F5F9] focus:border-[#E86A24] focus:outline-none flex-1 min-w-[160px]"
      />
      <input
        bind:value={newPath}
        placeholder="Executable Path (e.g. D:\Games\Game.exe)"
        class="px-3 py-1.5 rounded bg-[#0B0D10] border border-[#262C36] text-xs font-mono text-[#F1F5F9] focus:border-[#E86A24] focus:outline-none flex-[2] min-w-[240px]"
      />
      <button
        type="submit"
        class="px-3 py-1.5 rounded bg-[#E86A24] hover:bg-[#FF9E44] text-[#0B0D10] font-display text-xs font-bold tracking-wider uppercase cursor-pointer transition-colors"
      >
        REGISTER
      </button>
    </form>
  {/if}

  {#if displayGames.length === 0}
    <div class="ferro-panel rounded-xl p-6 flex items-center justify-between text-xs font-mono text-[#8A94A6]">
      <span>No Steam/Epic games detected yet. Use "ADD EXECUTABLE" in Armory or rescan libraries.</span>
      <button
        onclick={() => ferroStore.rescanAll()}
        class="px-3 py-1.5 rounded border border-[#262C36] hover:border-[#E86A24] text-[#F1F5F9] cursor-pointer"
      >
        RESCAN
      </button>
    </div>
  {:else}
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3.5">
      {#each displayGames as game (game.id)}
        {@const coverUrl = resolveAssetUrl(game.cover_image)}
        <button
          onclick={() => ferroStore.launchTarget(game.executable_path, game.launch_args)}
          class="group ferro-panel-interactive relative aspect-video rounded-xl overflow-hidden text-left cursor-pointer"
        >
          {#if coverUrl}
            <img
              src={coverUrl}
              alt={game.title}
              class="absolute inset-0 w-full h-full object-cover opacity-80 group-hover:opacity-100 group-hover:scale-105 transition-all duration-300"
            />
          {:else}
            <div class="absolute inset-0 bg-gradient-to-br from-[#161B24] via-[#0B0D10] to-[#1A120E] flex items-center justify-center">
              <Gamepad2 class="w-10 h-10 text-[#262C36] group-hover:text-[#E86A24]/60 transition-colors" />
            </div>
          {/if}

          <div class="absolute inset-0 bg-gradient-to-t from-[#0B0D10] via-[#0B0D10]/45 to-transparent"></div>

          <div class="absolute top-2.5 left-2.5 flex items-center gap-1.5">
            <span class="px-2 py-0.5 rounded bg-[#0B0D10]/85 border border-[#262C36] font-mono text-[10px] tracking-wider text-[#FF9E44] uppercase">
              {game.platform}
            </span>
          </div>

          <div class="absolute top-2.5 right-2.5 opacity-0 group-hover:opacity-100 transition-opacity">
            <div class="p-2 rounded-full bg-[#E86A24] text-[#0B0D10] shadow-[0_0_16px_rgba(232,106,36,0.6)]">
              <Play class="w-3.5 h-3.5 fill-current" />
            </div>
          </div>

          <div class="absolute bottom-0 inset-x-0 p-3 flex flex-col gap-0.5">
            <span class="font-display text-sm font-bold tracking-wide text-[#F1F5F9] truncate group-hover:text-[#FF9E44] transition-colors">
              {game.title}
            </span>
            <span class="font-mono text-[10px] text-[#8A94A6]">
              {game.launch_count > 0 ? `${game.launch_count} launches` : 'Ready'} • Tauri Active
            </span>
          </div>
        </button>
      {/each}
    </div>
  {/if}
</section>

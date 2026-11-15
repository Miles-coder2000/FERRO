<script lang="ts">
  import { Cpu, Plus } from 'lucide-svelte';

  let showCreateModal = $state(false);
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

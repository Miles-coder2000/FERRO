<script lang="ts">
  import { ferroStore } from '$lib/stores/ferroStore';
  import { onMount } from 'svelte';
  import { resolveAssetUrl } from '$lib/utils';
  import {
    Search, Terminal, FolderSearch, Minus, Pin, X, RefreshCw, Cpu, MemoryStick, Zap,
    Flame, FolderKanban, Gamepad2, Layers, Cpu as CpuIcon,
  } from 'lucide-svelte';

  // Page state
  let showSearch = $state(false);
  let searchQuery = $state('');
  let selectedIndex = $state(0);
  let activeSector = $state<'HOME' | 'FOLDERS' | 'ARMORY' | 'STACK' | 'CONTEXTS'>('HOME');
  let isMinimized = $state(false);
  let isPinnedOnTop = $state(false);

  // Sector definitions
  const sectors = [
    { id: 'HOME', label: 'HOME', icon: Flame },
    { id: 'FOLDERS', label: 'FOLDERS', icon: FolderKanban },
    { id: 'ARMORY', label: 'ARMORY', icon: Gamepad2 },
    { id: 'STACK', label: 'STACK', icon: Layers },
    { id: 'CONTEXTS', label: 'CONTEXTS', icon: Cpu },
  ];
</script>

<!doctype html>
<html lang="en" class="dark">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>FERRO - System Launcher</title>
    <link rel="preconnect" href="https://fonts.googleapis.com" />
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
    <link
      href="https://fonts.googleapis.com/css2?family=Chakra+Petch:wght@500;600;700&family=Inter:wght@400;500;600&family=JetBrains+Mono:wght@400;500;600&display=swap"
      rel="stylesheet"
    />
  </head>
  <body class="bg-[#0B0D10] text-[#F1F5F9] overflow-hidden select-none antialiased">
    <div id="app"></div>
    <script type="module" src="/src/main.ts"></script>
  </body>
</html>
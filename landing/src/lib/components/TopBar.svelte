<!-- TopBar.svelte - Sticky glassmorphic navbar -->
<div class="fixed top-0 left-0 right-0 z-50 border-b bg-[rgba(18,20,24,0.78)] backdrop-blur-md border-t border-white/8">
  <div class="max-w-[1400px] mx-auto px-6 py-3 flex items-center justify-between">
    <!-- Left: Logo + Wordmark -->
    <div class="flex items-center gap-2">
      <div class="w-8 h-8 rounded-full bg-[url('/assets/ferro-logo.svg')] bg-no-repeat bg-center"></div>
      <span class="text-xl font-bold tracking-wider uppercase text-[white]">FERRO</span>
    </div>

    <!-- Center: Navigation Links -->
    <nav class="hidden md:flex items-center gap-8">
      <a href="#sectors" class="text-gray-300 hover:text-[#E85D04] transition-colors">Sectors</a>
      <a href="#benchmarks" class="text-gray-300 hover:text-[#E85D04] transition-colors">Benchmarks</a>
      <a href="#architecture" class="text-gray-300 hover:text-[#E85D04] transition-colors">Architecture</a>
      <a href="#download" class="text-gray-300 hover:text-[#E85D04] transition-colors">Download</a>
    </nav>

    <!-- Right: Action Buttons -->
    <div class="flex items-center gap-3">
      <!-- GitHub ghost button -->
      <button 
        class="text-gray-400 hover:text-[#9CA3AF] transition-colors p-2 rounded-lg"
        aria-label="GitHub repository">
        <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6l4 2"></path><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 18v6l-4 2"></path><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m4.93 4.93 1.414 1.414A8 8 0 1114.172 9.828l1.414 1.414A6.003 6.003 0 009.828 14.172l1.414-1.414A4.5 4.5 0 014.93 4.93z"></path>
        </svg>
      </button>

      <!-- Download primary CTA -->
      <button 
        class="bg-[#E85D04] text-white font-medium px-4 py-2 rounded-lg hover:bg-[#F48C06] transition-colors shadow-lg shadow-[rgba(232,93,4,0.3)]"
        aria-label="Download FERRO">
        Download .exe
      </button>
    </div>
  </div>
</div>
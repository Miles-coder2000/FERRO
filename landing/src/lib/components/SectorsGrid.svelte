<!-- SectorsGrid.svelte - Four sector bento grid -->
<section class="py-20 md:py-32 bg-[rgba(18,20,24,0.8)] backdrop-blur rounded-lg">
  <div class="max-w-[1400px] mx-auto px-6">
    <h2 class="text-3xl font-bold text-[white] mb-12 text-center">Unified Sectors Bento Grid</h2>
    
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
      <!-- 01 // THE FORGE -->
      <div class="group rounded-lg bg-[rgba(18,22,28,0.8)] backdrop-blur border border-white/5 hover:border-[#E85D04] transition-colors">
        <div class="p-6">
          <div class="w-12 h-12 rounded-bg bg-[#E85D04]/20 flex items-center justify-center mb-4">
            <svg class="w-6 h-6 text-[#E85D04]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16H3"></path><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 3H3v13"></path><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 16l6 6v2m6-14l6 6v2m2.5-5.5a2.5 2.5 0 110 5l-2.5 2.5a2.5 2.5 0 01-2.5-5l2.5-2.5z"></path>
            </svg>
          </div>
          <h3 class="text-xl font-semibold text-[white] group-hover:text-[#E85D04] transition-colors">THE FORGE</h3>
          <p class="text-gray-400 mt-2">Unified command dashboard with quick access to recent sessions and live hardware telemetry.</p>
        </div>
      </div>

      <!-- 02 // ARMORY -->
      <div class="group rounded-lg bg-[rgba(18,22,28,0.8)] backdrop-blur border border-white/5 hover:border-[#E85D04] transition-colors">
        <div class="p-6">
          <div class="w-12 h-12 rounded-bg bg-[#E85D04]/20 flex items-center justify-center mb-4">
            <svg class="w-6 h-6 text-[#E85D04]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6l4 2"></path><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 18v6l-4 2"></path><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m4.93 4.93 1.414 1.414A8 8 0 1114.172 9.828l1.414 1.414A6.003 6.003 0 009.828 14.172l1.414-1.414A4.5 4.5 0 014.93 4.93z"></path>
            </svg>
          </div>
          <h3 class="text-xl font-semibold text-[white] group-hover:text-[#E85D04] transition-colors">ARMORY</h3>
          <p class="text-gray-400 mt-2">Unified game library with custom cover art, launch arguments, and instant native process execution.</p>
        </div>
      </div>

      <!-- 03 // STACK -->
      <div class="group rounded-lg bg-[rgba(18,22,28,0.8)] backdrop-blur border border-white/5 hover:border-[#E85D04] transition-colors">
        <div class="p-6">
          <div class="w-12 h-12 rounded-bg bg-[#38BDF8]/20 flex items-center justify-center mb-4">
            <svg class="w-6 h-6 text-[#38BDF8]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v6l6-6h6l6 6v-6h6l-6 6v-6h-6l-6-6v6H5l-6-6v6H2a4 4 0 01-4-4V7a4 4 0 014-4h6a4 4 0 014 4v6h2z"></path>
            </svg>
          </div>
          <h3 class="text-xl font-semibold text-[white] group-hover:text-[#38BDF8] transition-colors">STACK</h3>
          <p class="text-gray-400 mt-2">Categorized desktop software, developer tools, and utilities into zero-clutter workspaces.</p>
        </div>
      </div>

      <!-- 04 // ACTIVE SYSTEM FOLDERS -->
      <div class="group rounded-lg bg-[rgba(18,22,28,0.8)] backdrop-blur border border-white/5 hover:border-[#38BDF8] transition-colors">
        <div class="p-6">
          <div class="w-12 h-12 rounded-bg bg-[#38BDF8]/20 flex items-center justify-center mb-4">
            <svg class="w-6 h-6 text-[#38BDF8]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v6l6-6h6l6 6v-6h6l-6 6v-6h-6l-6-6v6H5l-6-6v6H2a4 4 0 01-4-4V7a4 4 0 014-4h6a4 4 0 014 4v6h2z"></path>
            </svg>
          </div>
          <h3 class="text-xl font-semibold text-[white] group-hover:text-[#38BDF8] transition-colors">SYSTEM FOLDERS</h3>
          <p class="text-gray-400 mt-2">Asynchronous Rust directory scanning with live filesystem watchers and instant file-count badges.</p>
        </div>
      </div>
    </div>
  </div>
</section>
<!-- Benchmarks.svelte - Visual comparison of FERRO vs Electron -->
<section class="py-20 md:py-32 bg-[rgba(18,20,24,0.8)] backdrop-blur rounded-lg">
  <div class="max-w-[1400px] mx-auto px-6">
    <h2 class="text-3xl font-bold text-[white] mb-12 text-center">Resource & Speed Benchmarks</h2>
    
    <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
      <!-- Idle Memory Footprint -->
      <div class="p-6 rounded-lg bg-[rgba(18,22,28,0.8)] backdrop-blur-border">
        <h3 class="text-xl font-semibold text-[white] mb-4">Idle Memory Footprint</h3>
        <div class="flex items-center gap-4">
          <div class="w-6 h-6 rounded-full bg-[#E85D04]"></div>
          <div>
            <p class="text-3xl font-bold text-[#E85D04]">38 MB</p>
            <p class="text-gray-400">FERRO</p>
          </div>
        </div>
        <div class="mt-3">
          <p class="text-gray-400">320 MB</p>
          <p class="text-gray-500 text-sm">Electron Launchers</p>
        </div>
      </div>
      
      <div class="progress-bar bg-gray-700 rounded-full h-4 mb-2">
        <div class="bg-gradient-to-r from-[#E85D04] to-[#F48C06] rounded-full h-4 w-1/4" title="FERRO: 38 MB (12%)"></div>
        <div class="bg-gradient-to-r from-red-500 to-orange-500 rounded-full h-4 w-3/4" title="Electron: 320 MB (80%)"></div>
      </div>
    </div>

      <!-- Installer Package Size -->
      <div class="p-6 rounded-lg bg-[rgba(18,22,28,0.8)] backdrop-blur-border">
        <h3 class="text-xl font-semibold text-[white] mb-4">Installer Package Size</h3>
        <div class="flex items-center gap-4">
          <div class="w-6 h-6 rounded-full bg-[#E85D04]"></div>
          <div>
            <p class="text-3xl font-bold text-[#E85D04]">~4.8 MB</p>
            <p class="text-gray-400">FERRO</p>
          </div>
        </div>
        <div class="mt-3">
          <p class="text-gray-400">115 MB</p>
          <p class="text-gray-500 text-sm">Electron Launchers</p>
        </div>
      </div>

      <div class="progress-bar bg-gray-700 rounded-full h-4 mb-2">
        <div class="bg-gradient-to-r from-[#E85D04] to-[#F48C06] rounded-full h-4 w-1/25" title="FERRO: ~4.8 MB (4%)"></div>
        <div class="bg-gradient-to-r from-red-500 to-orange-500 rounded-full h-4 w-24/25" title="Electron: 115 MB (80%)"></div>
      </div>
    </div>

      <!-- 100k File Search Latency -->
      <div class="p-6 rounded-lg bg-[rgba(18,22,28,0.8)] backdrop-blur-border">
        <h3 class="text-xl font-semibold text-[white] mb-4">100,000-File Search Latency</h3>
        <div class="flex items-center gap-4">
          <div class="w-6 h-6 rounded-full bg-[#E85D04]"></div>
          <div>
            <p class="text-3xl font-bold text-[#E85D04]">15 ms</p>
            <p class="text-gray-400">FERRO (SQLite FTS5)</p>
          </div>
        </div>
        <div class="mt-3">
          <p class="text-gray-400">140+ ms</p>
          <p class="text-gray-500 text-sm">Electron Launchers</p>
        </div>
      </div>

      <!-- Cold Boot to Interactive -->
      <div class="p-6 rounded-lg bg-[rgba(18,22,28,0.8)] backdrop-blur-border">
        <h3 class="text-xl font-semibold text-[white] mb-4">Cold Boot to Interactive</h3>
        <div class="flex items-center gap-4">
          <div class="w-6 h-6 rounded-full bg-[#E85D04]"></div>
          <div>
            <p class="text-3xl font-bold text-[#E85D04]">180 ms</p>
            <p class="text-gray-400">FERRO</p>
          </div>
        </div>
        <div class="mt-3">
          <p class="text-gray-400">2.1 seconds</p>
          <p class="text-gray-500 text-sm">Electron Launchers</p>
        </div>
      </div>
    </div>
  </div>
</section>
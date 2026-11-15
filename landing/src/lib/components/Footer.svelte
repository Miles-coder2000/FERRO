<!-- Footer.svelte - Footer with links -->
<footer class="py-12 bg-[rgba(10,13,16,0.8)] backdrop-blur border-t border-white/5">
  <div class="max-w-[1400px] mx-auto px-6">
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
      <!-- FERRO Emblem -->
      <div class="self-center">
        <div class="w-12 h-12 rounded-full bg-[url('/assets/ferro-logo.svg')] bg-no-repeat bg-center mb-4"></div>
        <span class="text-xl font-bold uppercase tracking-wider text-[#E85D04]">FERRO</span>
      </div>

      <!-- Nav Links -->
      <nav class="col-span-2 md:col-span-3">
        <ul class="space-y-3 text-gray-400">
          <li><a href="#" class="hover:text-[#E85D04] transition-colors">GitHub Repository</a></li>
          <li><a href="#" class="hover:text-[#E85D04] transition-colors">Latest Releases</a></li>
          <li><a href="#" class="hover:text-[#E85D04] transition-colors">Report a Bug (Issues)</a></li>
        </ul>
      </nav>

      <!-- Architecture Security -->
      <div>
        <h4 class="text-sm font-medium text-[#E85D04] mb-3">Rust Architecture</h4>
        <ul class="text-sm text-gray-400 space-y-1">
          <li>Native Rust + WebView2</li>
          <li>Hardened Local Execution</li>
          <li>Path Canonicalization</li>
        </ul>
      </div>

      <!-- License -->
      <div class="col-span-2 text-center text-xs text-gray-500">
        <p>© 2024 FERRO Launcher</p>
        <p>Source License: MIT</p>
      </div>
    </div>
  </div>
</footer>
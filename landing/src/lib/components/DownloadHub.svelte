<!-- DownloadHub.svelte - Download and quick-start hub -->
<section class="py-20 md:py-32 bg-[rgba(18,20,24,0.8)] backdrop-blur rounded-lg">
  <div class="max-w-[1400px] mx-auto px-6">
    <h2 class="text-3xl font-bold text-[white] mb-12 text-center">Download & Quick-Start Hub</h2>
    
    <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
      <!-- Card A: Windows Installer -->
      <div class="p-6 rounded-lg bg-[rgba(18,22,28,0.8)] backdrop-blur-border border border-white/5">
        <h3 class="text-xl font-semibold text-[white] mb-4">Windows Installer (FERRO_x64-setup.exe) [RECOMMENDED]</h3>
        <p class="text-gray-400 mb-4">Guided NSIS installer, automatic WebView2 verification, Start Menu integration, and clean Windows uninstaller.</p>
        <div class="flex flex-col sm:flex-row gap-3">
          <button 
            class="flex-1 bg-[#E85D04] text-white font-medium px-4 py-2 rounded-lg hover:bg-[#F48C06] transition-colors"
            aria-label="Download Windows Installer">
            Download Installer
          </button>
          <div class="text-sm text-gray-400">
            <span id="installer-version">v0.1.0</span>
            <span class="ml-2">• Free & Open Source</span>
          </div>
        </div>
        <p class="text-xs text-gray-500 mt-3">Guided NSIS installer with automatic WebView2 verification</p>
      </div>

      <!-- Card B: Portable Executable -->
      <div class="p-6 rounded-lg bg-[rgba(18,22,28,0.8)] backdrop-blur-border border border-white/5">
        <h3 class="text-xl font-semibold text-[white] mb-4">Portable Executable (ferro-launcher.exe)</h3>
        <p class="text-gray-400 mb-4">Single zero-install binary; drop onto any folder or USB drive and run immediately.</p>
        <div class="flex flex-col sm:flex-row gap-3">
          <button 
            class="flex-1 bg-[#E85D04] text-white font-medium px-4 py-2 rounded-lg hover:bg-[#F48C06] transition-colors"
            aria-label="Download Portable Executable">
            Download .exe
          </button>
          <div class="text-sm text-gray-400">
            <span id="portable-version">v0.1.0</span>
            <span class="ml-2">• Copyable SHA-256</span>
          </div>
        </div>
        <p class="text-xs text-gray-500 mt-3">Copy: <code>Get-FileHash .\ferro-launcher.exe -Algorithm SHA256</code></p>
      </div>
    </div>

    <!-- Build From Source Accordion -->
    <div class="mt-8 p-6 rounded-lg bg-[rgba(18,22,28,0.8)] backdrop-blur-border">
      <h3 class="text-xl font-semibold text-[white] mb-4">Build From Source (For Developers)</h3>
      <p class="text-gray-400 mb-3">Copy these commands to build FERRO from source:</p>
      <pre class="bg-gray-900 text-sm text-gray-300 rounded p-4 overflow-auto">
<code>git clone https://github.com/Miles-coder2000/FERRO.git
cd FERRO
npm install
npx tauri build --no-bundle</code></pre>
    </div>

    <!-- System Requirements -->
    <div class="mt-8 pt-8 border-t border-white/5">
      <h3 class="text-xl font-semibold text-[white] mb-4">System Requirements</h3>
      <p class="text-gray-400 text-sm">Windows 10 (version 1809+) or Windows 11 (x64), Microsoft Edge WebView2 Runtime, 50 MB disk space</p>
    </div>
  </div>
</section>
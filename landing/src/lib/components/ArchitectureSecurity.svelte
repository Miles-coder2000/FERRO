<!-- ArchitectureSecurity.svelte - Rust architecture & security guarantees -->
<section class="py-20 md:py-32 bg-[rgba(18,20,24,0.8)] backdrop-blur rounded-lg">
  <div class="max-w-[1400px] mx-auto px-6">
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
      <!-- Native Rust + WebView2 Engine -->
      <div>
        <h2 class="text-2xl font-bold text-[white] mb-6">Native Rust + WebView2 Engine</h2>
        <p class="text-gray-300 mb-4">Uses Windows' built-in Edge WebView2 instead of shipping a redundant 120 MB Chromium runtime. This keeps the binary size under 8 MB while providing full access to native Web capabilities.</p>
        <ul class="list-disc list-inside text-gray-400 space-y-2">
          <li>Edge WebView2 runtime (system-installed)</li>
          <li>No bundled Chromium</li>
          <li>< 8 MB binary size</li>
          <li>Full Win32 API access via Rust</li>
        </ul>
      </div>

      <!-- Hardened Local Execution -->
      <div>
        <h2 class="text-2xl font-bold text-[white] mb-6">Hardened Local Execution</h2>
        <p class="text-gray-300 mb-4">Zero shell-string interpolation eliminates command injection. Strict path canonicalization guards prevent traversal attacks. OS native credential storage via keyring crate.</p>
        <ul class="list-disc list-inside text-gray-400 space-y-2">
          <li>No shell-string interpolation</li>
          <li>Path canonicalization guards</li>
          <li>OS native credential storage</li>
          <li>Parameterized SQLite queries</li>
        </ul>
      </div>
    </div>
  </div>
</section>
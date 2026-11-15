<!-- App.svelte - Root HUD shell -->
<div class="min-h-screen bg-gray-950 text-gray-400 font-sans antialiased">
  <!-- Sticky Navbar -->
  <TopBar />
  
  <!-- Main Content -->
  <div class="pt-20 pb-40">
    <!-- Hero Section -->
    <HeroForgeDemo />
    
    <!-- Divider -->
    <div class="my-16 border-y border-white/10"></div>
    
    <!-- Benchmarks -->
    <Benchmarks />
    
    <!-- Divider -->
    <div class="my-16 border-y border-white/10"></div>
    
    <!-- Sectors Grid -->
    <SectorsGrid />
    
    <!-- Divider -->
    <div class="my-16 border-y border-white/10"></div>
    
    <!-- Architecture & Security -->
    <ArchitectureSecurity />
    
    <!-- Divider -->
    <div class="my-16 border-y border-white/10"></div>
    
    <!-- Download Hub -->
    <DownloadHub />
    
    <!-- Divider -->
    <div class="my-16 border-y border-white/10"></div>
    
    <!-- Footer -->
    <Footer />
  </div>
</div>
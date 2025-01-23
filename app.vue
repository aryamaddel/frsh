<template>
  <div id="app" class="min-h-screen bg-gray-50 text-gray-800">
    <header class="bg-white shadow">
      <div class="container mx-auto px-4 py-4 flex justify-between items-center">
        <h1 class="text-2xl font-bold text-green-500">frsh</h1>
        <nav>
          <ul class="flex space-x-4">
            <li>
              <NuxtLink to="/" class="hover:text-green-500">Home</NuxtLink>
            </li>
            <li>
              <NuxtLink to="/blog" class="hover:text-green-500">Blog</NuxtLink>
            </li>
            <li>
              <NuxtLink to="/about" class="hover:text-green-500">About</NuxtLink>
            </li>
          </ul>
        </nav>
      </div>
    </header>

    <main class="container mx-auto px-4 py-8">
      <NuxtPage />
    </main>

    <NuxtRouteAnnouncer />

    <footer class="bg-gray-200 py-4 mt-8">
      <div class="container mx-auto px-4 text-center text-sm text-gray-600">
        <p>&copy; 2025 frsh. All rights reserved.</p>
      </div>
    </footer>
  </div>
</template>

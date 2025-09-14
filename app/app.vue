<script setup lang="ts">
useHead({
  title: "frsh",
  meta: [
    {
      name: "description",
      content:
        "Sharing my journey through technology, projects I've built over the years, and anything tech-related.",
    },
  ],
});

useSeoMeta({
  title: "frsh",
  ogTitle: "tech blog",
  description:
    "Sharing my journey through technology, projects I've built over the years, and anything tech-related.",
  ogDescription:
    "Sharing my journey through technology, projects I've built over the years, and anything tech-related.",
});

onMounted(() => {
  const currentHour = new Date().getHours();

  if (!(currentHour >= 8 && currentHour < 18)) {
    document.documentElement.classList.add("dark");
  } else {
    document.documentElement.classList.remove("dark");
  }
});
</script>

<template>
  <div
    id="app"
    class="min-h-screen flex flex-col bg-gradient-to-br from-gray-50 to-gray-100 dark:from-gray-900 dark:to-gray-800 text-gray-800 dark:text-gray-200 antialiased transition-colors duration-300"
  >
    <header
      class="bg-white dark:bg-gray-800 shadow-md sticky top-0 z-50 backdrop-blur-sm bg-white/90 dark:bg-gray-800/90 transition-colors duration-300"
    >
      <div class="container mx-auto px-4 sm:px-6 lg:px-8 py-4">
        <div class="flex justify-between items-center">
          <NuxtLink
            to="/"
            class="text-2xl sm:text-3xl font-extrabold text-green-500 hover:text-green-600 dark:text-green-400 dark:hover:text-green-300 transition-colors duration-200 tracking-tight"
          >
            frsh
          </NuxtLink>

          <nav class="relative">
            <ul class="flex items-center space-x-2 sm:space-x-6 font-medium">
              <li>
                <NuxtLink
                  to="/blog"
                  class="px-3 py-2 rounded-lg hover:bg-green-50 hover:text-green-600 dark:hover:bg-green-900/30 dark:hover:text-green-400 transition-all duration-200 active:scale-95"
                >
                  Blog
                </NuxtLink>
              </li>
              <li>
                <NuxtLink
                  to="https://github.com/aryamaddel/frsh"
                  class="px-3 py-2 rounded-lg hover:bg-green-50 hover:text-green-600 dark:hover:bg-green-900/30 dark:hover:text-green-400 transition-all duration-200 active:scale-95"
                >
                  Github
                </NuxtLink>
              </li>

              <li>
                <NuxtLink
                  to="/about"
                  class="px-3 py-2 rounded-lg hover:bg-green-50 hover:text-green-600 dark:hover:bg-green-900/30 dark:hover:text-green-400 transition-all duration-200 active:scale-95"
                >
                  About
                </NuxtLink>
              </li>
            </ul>
          </nav>
        </div>
      </div>
    </header>

    <main class="container mx-auto px-4 sm:px-6 lg:px-8 py-12 flex-grow">
      <NuxtPage />
    </main>

    <NuxtRouteAnnouncer />

    <footer
      class="bg-white dark:bg-gray-800 shadow-inner mt-16 transition-colors duration-300"
    >
      <div class="container mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div class="flex flex-col items-center justify-center space-y-4">
          <p class="text-sm text-gray-600 dark:text-gray-400">
            &copy; 2025 frsh. All rights reserved.
          </p>
        </div>
      </div>
    </footer>
  </div>
</template>

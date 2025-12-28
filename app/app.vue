<script setup lang="ts">
const colorMode = useColorMode();

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

const toggleTheme = () => {
  colorMode.preference = colorMode.preference === "dark" ? "light" : "dark";
};
</script>

<style>
@import "@/assets/css/glass-reflection.css";
</style>

<template>
  <div
    id="app"
    class="min-h-screen flex flex-col text-gray-800 dark:text-gray-200 antialiased transition-all duration-500 relative"
    style="
      background-image: url(&quot;/background.gif&quot;);
      background-size: cover;
      background-position: center;
      background-attachment: fixed;
      background-repeat: no-repeat;
    "
  >
    <header
      class="bg-white/80 dark:bg-gray-900/80 border-b border-gray-300 dark:border-gray-700 sticky top-0 z-50 backdrop-blur-md transition-all duration-500"
    >
      <div class="container mx-auto px-4 sm:px-6 lg:px-8 py-4">
        <div class="flex justify-around items-center">
          <NuxtLink
            to="/"
            class="glass-reflection text-2xl sm:text-3xl font-extrabold text-emerald-600 hover:text-emerald-700 dark:text-emerald-400 dark:hover:text-emerald-300 transition-all duration-300 tracking-tight"
          >
            frsh
          </NuxtLink>

          <nav>
            <ul class="flex items-center space-x-2 sm:space-x-6 font-medium">
              <li>
                <NuxtLink
                  to="/blog"
                  class="glass-reflection px-3 py-2 border border-transparent hover:border-emerald-500/50 hover:bg-emerald-500/5 hover:text-emerald-600 dark:hover:text-emerald-400 transition-all duration-300"
                >
                  Blog
                </NuxtLink>
              </li>
              <li>
                <NuxtLink
                  to="https://github.com/aryamaddel/frsh"
                  class="glass-reflection px-3 py-2 border border-transparent hover:border-emerald-500/50 hover:bg-emerald-500/5 hover:text-emerald-600 dark:hover:text-emerald-400 transition-all duration-300"
                >
                  Github
                </NuxtLink>
              </li>
            </ul>
          </nav>
        </div>
      </div>
    </header>

    <ColorScheme>
      <ThemeToggle :isDark="colorMode.value === 'dark'" @toggle="toggleTheme" />
    </ColorScheme>

    <main class="container mx-auto px-4 sm:px-6 lg:px-8 py-12 flex-grow">
      <NuxtPage />
    </main>

    <NuxtRouteAnnouncer />

    <footer
      class="bg-white/80 dark:bg-gray-900/80 border-t border-gray-300 dark:border-gray-700 mt-16 backdrop-blur-md transition-all duration-500"
    >
      <div class="container mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div class="flex flex-col items-center justify-center space-y-4">
          <p class="text-sm text-gray-200">
            &copy; 2025 frsh. All rights reserved.
          </p>
        </div>
      </div>
    </footer>
  </div>
</template>

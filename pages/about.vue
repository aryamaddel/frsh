<script setup lang="ts"></script>

<template>
  <main class="container mx-auto px-4 py-12 max-w-5xl">
    <section class="text-center mb-16">
      <h1 class="text-5xl font-bold mb-6 text-gray-800 dark:text-gray-100">
        We Post
      </h1>
      <p class="text-xl text-gray-600 dark:text-gray-300 max-w-2xl mx-auto">
        A group of passionate Computer Science students from Pune, India,
        working on innovative solutions.
      </p>
    </section>

    <div class="grid md:grid-cols-3 gap-8">
      <NuxtLink
        to="https://aryamaddel.github.io/"
        target="_blank"
        rel="noopener noreferrer"
      >
        <article
          class="bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-xl shadow-sm hover:shadow-md hover:border-green-200 dark:hover:border-green-700 transition-all duration-200"
        >
          <div class="p-6">
            <div class="aspect-square mb-4 overflow-hidden rounded-lg">
              <nuxt-img
                src="team/Arya.jpg"
                alt="Arya Maddel"
                class="w-full h-full object-contain bg-gray-50 dark:bg-gray-900"
              />
            </div>
            <h3
              class="text-center text-2xl font-semibold text-gray-800 dark:text-gray-200"
            >
              Arya Maddel
            </h3>
          </div>
        </article>
      </NuxtLink>

      <NuxtLink
        to="https://anishmaiseker.github.io/"
        target="_blank"
        rel="noopener noreferrer"
      >
        <article
          class="bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-xl shadow-sm hover:shadow-md hover:border-green-200 dark:hover:border-green-700 transition-all duration-200"
        >
          <div class="p-6">
            <div class="aspect-square mb-4 overflow-hidden rounded-lg">
              <nuxt-img
                src="team/Anish.jpg"
                alt="Anish Maiseker"
                class="w-full h-full object-contain bg-gray-50 dark:bg-gray-900"
              />
            </div>
            <h3
              class="text-center text-2xl font-semibold text-gray-800 dark:text-gray-200"
            >
              Anish Maisekar
            </h3>
          </div>
        </article>
      </NuxtLink>

      <NuxtLink
        to="https://ruchirkalokhe.github.io/portfolio"
        target="_blank"
        rel="noopener noreferrer"
      >
        <article
          class="bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-xl shadow-sm hover:shadow-md hover:border-green-200 dark:hover:border-green-700 transition-all duration-200"
        >
          <div class="p-6">
            <div class="aspect-square mb-4 overflow-hidden rounded-lg">
              <nuxt-img
                src="team/Ruchir.jpg"
                alt="Ruchir Kalohke"
                class="w-full h-full object-contain bg-gray-50 dark:bg-gray-900"
              />
            </div>
            <h3
              class="text-center text-2xl font-semibold text-gray-800 dark:text-gray-200"
            >
              Ruchir Kalohke
            </h3>
          </div>
        </article>
      </NuxtLink>

      <NuxtLink
        to="https://ruchirkalokhe.github.io/"
        target="_blank"
        rel="noopener noreferrer"
        class="md:col-start-2"
      >
        <article
          class="bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-xl shadow-sm hover:shadow-md hover:border-green-200 dark:hover:border-green-700 transition-all duration-200"
        >
          <div class="p-6">
            <div class="aspect-square mb-4 overflow-hidden rounded-lg">
              <nuxt-img
                src="team/Nimish.jpg"
                alt="Nimish Shinde"
                class="w-full h-full object-contain bg-gray-50 dark:bg-gray-900"
              />
            </div>
            <h3
              class="text-center text-2xl font-semibold text-gray-800 dark:text-gray-200"
            >
              Nimish Shinde
            </h3>
          </div>
        </article>
      </NuxtLink>
    </div>
  </main>
</template>

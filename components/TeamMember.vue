<template>
  <NuxtLink :to="link" target="_blank" rel="noopener noreferrer">
    <article
      class="bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-xl shadow-sm hover:shadow-md hover:border-green-200 dark:hover:border-green-700 transition-all duration-200"
    >
      <div class="p-6">
        <div class="aspect-square mb-4 overflow-hidden rounded-lg">
          <nuxt-img
            :src="image"
            :alt="name"
            class="w-full h-full object-contain bg-gray-50 dark:bg-gray-900"
          />
        </div>
        <h3
          class="text-center text-2xl font-semibold text-gray-800 dark:text-gray-200"
        >
          {{ name }}
        </h3>
      </div>
    </article>
  </NuxtLink>
</template>

<script setup lang="ts">

defineProps({
  name: String,
  image: String,
  link: String,
});
</script>

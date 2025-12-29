<script setup lang="ts">
const { data: posts } = await useAsyncData("blog-posts", () =>
  queryCollection("blog")
    .select("id", "author", "date", "title", "description", "path")
    .order("date", "DESC")
    .all()
);
</script>

<template>
  <div class="">
    <div class="container mx-auto px-4 py-12 max-w-5xl">
      <h1
        class="text-4xl font-bold text-center mb-12 text-gray-900 dark:text-gray-100"
      >
        Blog Posts
      </h1>

      <div v-if="posts?.length === 0" class="text-center py-12">
        <p class="text-gray-600 dark:text-gray-400 text-lg">No posts found</p>
      </div>

      <ul v-else class="grid gap-6">
        <li v-for="post in posts" :key="post.id" class="group">
          <NuxtLink
            :to="post.path"
            class="glass-reflection block p-6 bg-white/60 dark:bg-gray-900/60 border border-gray-300 dark:border-gray-700 backdrop-blur-sm hover:border-emerald-500 dark:hover:border-emerald-500 hover:bg-white/80 dark:hover:bg-gray-900/80 transition-all duration-300"
          >
            <h2
              class="text-2xl font-semibold mb-3 text-gray-900 dark:text-gray-100 group-hover:text-emerald-600 dark:group-hover:text-emerald-400 transition-all duration-300"
            >
              {{ post.title }}
            </h2>
            <p class="text-gray-600 dark:text-gray-300 mb-4 line-clamp-2">
              {{ post.description }}
            </p>

            <div
              class="flex flex-wrap items-center gap-4 text-sm text-gray-500 dark:text-gray-400"
            >
              <span class="flex items-center">
                <span class="mr-2">📅</span>
                {{ new Date(post.date).toLocaleDateString() }}
              </span>
              <span class="flex items-center">
                <span class="mr-2">👤</span>
                {{ post.author }}
              </span>
            </div>
          </NuxtLink>
        </li>
      </ul>
    </div>
  </div>
</template>

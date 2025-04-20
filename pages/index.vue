<script setup lang="ts">
const { data: latestPosts } = await useAsyncData("latest-posts", () =>
  queryCollection("blog")
    .order("date", "DESC")
    .select("id", "title", "description", "path", "date", "author")
    .limit(2)
    .all()
);
</script>

<template>
  <main class="container mx-auto px-4 py-12 max-w-5xl">
    <section class="text-center mb-16">
      <h1 class="text-5xl font-bold mb-6 text-gray-800 dark:text-gray-100">
        Welcome to frsh
      </h1>
      <p class="text-xl text-gray-600 dark:text-gray-300 max-w-2xl mx-auto">
        Sharing my journey through technology, projects I've built over the
        years, and anything tech-related.
      </p>
    </section>

    <section>
      <div
        class="flex flex-col md:flex-row items-start md:items-center justify-between mb-8 space-y-4 md:space-y-0"
      >
        <h2 class="text-3xl font-bold text-gray-800 dark:text-gray-100">
          Latest Posts
        </h2>
        <NuxtLink
          to="/blog"
          class="inline-flex items-center px-4 py-2 rounded-lg bg-green-50 text-green-600 hover:bg-green-100 dark:bg-green-900/30 dark:text-green-400 dark:hover:bg-green-800/50 transition-colors duration-200"
        >
          View All Posts
          <span class="ml-2">→</span>
        </NuxtLink>
      </div>

      <div class="grid md:grid-cols-2 gap-8">
        <article
          v-for="post in latestPosts"
          :key="post.id"
          class="bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-xl shadow-sm hover:shadow-md hover:border-green-200 dark:hover:border-green-700 transition-all duration-200"
        >
          <NuxtLink :to="post.path" class="block p-6">
            <div class="mb-4">
              <h3
                class="text-2xl font-semibold text-gray-800 dark:text-gray-200 mb-3"
              >
                {{ post.title }}
              </h3>
              <p class="text-gray-600 dark:text-gray-300 line-clamp-2">
                {{ post.description }}
              </p>
            </div>

            <div
              class="flex items-center justify-between text-sm text-gray-500 dark:text-gray-400"
            >
              <span class="flex items-center">
                {{ post.author }}
              </span>
              <span class="flex items-center">
                <span class="mr-2">📅</span>
                {{ new Date(post.date).toLocaleDateString() }}
              </span>
            </div>
          </NuxtLink>
        </article>
      </div>
    </section>
  </main>
</template>

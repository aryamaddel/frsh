<script setup lang="ts">
// Fetch latest posts
const { data: latestPosts } = await useAsyncData('latest-posts', () =>
  queryCollection('blog')
    .order('date', 'DESC')
    .select('id', 'title', 'description', 'path', 'date', 'author', 'tags')
    .limit(2)
    .all()
)
</script>

<template>
  <main class="container mx-auto px-4 py-12 max-w-5xl">
    <!-- Hero Section -->
    <section class="text-center mb-16">
      <h1 class="text-5xl font-bold mb-6 text-gray-800">Welcome to frsh</h1>
      <p class="text-xl text-gray-600 max-w-2xl mx-auto">
        Sharing my journey through technology, projects I've built over the years, and anything tech-related.
      </p>
    </section>

    <!-- Latest Posts Section -->
    <section>
      <div class="flex items-center justify-between mb-8">
        <h2 class="text-3xl font-bold text-gray-800">Latest Posts</h2>
        <NuxtLink to="/blog"
          class="inline-flex items-center px-4 py-2 rounded-lg bg-green-50 text-green-600 hover:bg-green-100 transition-colors duration-200">
          View All Posts
          <span class="ml-2">→</span>
        </NuxtLink>
      </div>

      <div class="grid md:grid-cols-2 gap-8">
        <article v-for="post in latestPosts" :key="post.id"
          class="bg-white border border-gray-200 rounded-xl shadow-sm hover:shadow-md hover:border-green-200 transition-all duration-200">
          <NuxtLink :to="post.path" class="block p-6">
            <div class="mb-4">
              <ul class="flex flex-wrap gap-2 mb-3">
                <li v-for="tag in post.tags" :key="tag"
                  class="px-3 py-1 bg-green-50 text-green-600 rounded-full text-sm font-medium">
                  {{ tag }}
                </li>
              </ul>
              <h3 class="text-2xl font-semibold text-gray-800 mb-3">{{ post.title }}</h3>
              <p class="text-gray-600 line-clamp-2">{{ post.description }}</p>
            </div>

            <div class="flex items-center justify-between text-sm text-gray-500">
              <span class="flex items-center">
                <span class="mr-2">👤</span>
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
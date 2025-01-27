<script setup lang="ts">
const { data: posts } = await useAsyncData('blog', () =>
  queryCollection('blog')
    .select('id', 'title', 'path')
    .all()
)

console.dir(posts.value)
</script>

<template>
  <div class="container mx-auto px-4 py-8">
    <h1 class="text-4xl font-bold text-center mb-8">Blog Posts</h1>
    <ul>
      <li v-for="post in posts" :key="post.id">
        <NuxtLink :to="post.path">{{ post.title }}</NuxtLink>
      </li>
    </ul>
  </div>
</template>

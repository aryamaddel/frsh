<script setup lang="ts">

const searchQuery = ref('')
const selectedAuthor = ref('')
const selectedTag = ref('')

const { data: allPosts } = await useAsyncData('all-blog-posts', () =>
  queryCollection('blog')
    .select('id', 'title', 'description', 'path', 'date', 'author', 'tags')
    .order('date', 'DESC')
    .all()
)

const filteredPosts = computed(() => {
  if (!allPosts.value) return []
  return allPosts.value.filter(post =>
    (!searchQuery.value || post.title.toLowerCase().includes(searchQuery.value.toLowerCase())) &&
    (!selectedAuthor.value || post.author === selectedAuthor.value) &&
    (!selectedTag.value || post.tags.includes(selectedTag.value))
  )
})

const authors = computed(() => [...new Set(allPosts.value?.map(post => post.author) || [])])
const tags = computed(() => [...new Set(allPosts.value?.flatMap(post => post.tags) || [])])
</script>

<template>
  <div class="container mx-auto px-4 py-8">
    <h1 class="text-4xl font-bold text-center mb-8">Blog Posts</h1>

    <div class="mb-4">
      <input type="search" v-model="searchQuery" placeholder="Search" class="w-full p-2 border rounded">
    </div>

    <div class="mb-4 flex space-x-4">
      <select v-model="selectedAuthor" class="p-2 border rounded">
        <option value="">All Authors</option>
        <option v-for="author in authors" :key="author" :value="author">{{ author }}</option>
      </select>
      <select v-model="selectedTag" class="p-2 border rounded">
        <option value="">All Tags</option>
        <option v-for="tag in tags" :key="tag" :value="tag">{{ tag }}</option>
      </select>
    </div>

    <ul>
      <li v-for="post in filteredPosts" :key="post.id" class="mb-6">
        <NuxtLink :to="post.path" class="block p-4 border rounded-lg hover:bg-gray-100">
          <h2 class="text-2xl font-semibold mb-2">{{ post.title }}</h2>
          <p class="text-gray-600 mb-2">{{ post.description }}</p>
          <p class="text-sm text-gray-500 mb-2">{{ new Date(post.date).toLocaleDateString() }}</p>
          <p class="text-sm font-medium mb-2">Author: {{ post.author }}</p>
          <ul class="flex flex-wrap">
            <li v-for="tag in post.tags" :key="tag" class="mr-2 mb-1 px-2 py-1 bg-gray-200 rounded-full text-xs">
              {{ tag }}
            </li>
          </ul>
        </NuxtLink>
      </li>
    </ul>
  </div>
</template>
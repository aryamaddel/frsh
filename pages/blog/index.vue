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
  <div class="container mx-auto px-4 py-8 max-w-5xl">
    <h1 class="text-4xl font-bold text-center mb-12 text-gray-800">Blog Posts</h1>

    <div class="mb-8 space-y-4 sm:space-y-0 sm:flex sm:space-x-4">
      <input type="search" v-model="searchQuery" placeholder="Search posts..."
        class="w-full sm:w-1/2 p-3 border border-gray-200 rounded-lg shadow-sm focus:ring-2 focus:ring-green-500 focus:border-green-500 outline-none transition-colors duration-200">

      <div class="flex space-x-4 w-full sm:w-1/2">
        <select v-model="selectedAuthor"
          class="w-1/2 p-3 border border-gray-200 rounded-lg shadow-sm focus:ring-2 focus:ring-green-500 focus:border-green-500 outline-none transition-colors duration-200">
          <option value="">All Authors</option>
          <option v-for="author in authors" :key="author" :value="author">{{ author }}</option>
        </select>

        <select v-model="selectedTag"
          class="w-1/2 p-3 border border-gray-200 rounded-lg shadow-sm focus:ring-2 focus:ring-green-500 focus:border-green-500 outline-none transition-colors duration-200">
          <option value="">All Tags</option>
          <option v-for="tag in tags" :key="tag" :value="tag">{{ tag }}</option>
        </select>
      </div>
    </div>

    <ul class="space-y-6">
      <li v-for="post in filteredPosts" :key="post.id">
        <NuxtLink :to="post.path"
          class="block p-6 bg-white border border-gray-200 rounded-xl shadow-sm hover:shadow-md hover:border-green-200 transition-all duration-200">
          <h2 class="text-2xl font-semibold mb-3 text-gray-800 group-hover:text-green-600">{{ post.title }}</h2>
          <p class="text-gray-600 mb-4 line-clamp-2">{{ post.description }}</p>

          <div class="flex flex-wrap items-center gap-4 text-sm">
            <span class="flex items-center text-gray-500">
              <span class="mr-2">📅</span>
              {{ new Date(post.date).toLocaleDateString() }}
            </span>

            <span class="flex items-center text-gray-500">
              <span class="mr-2">👤</span>
              {{ post.author }}
            </span>
          </div>

          <ul class="flex flex-wrap gap-2 mt-4">
            <li v-for="tag in post.tags" :key="tag"
              class="px-3 py-1 bg-green-50 text-green-600 rounded-full text-sm font-medium hover:bg-green-100 transition-colors duration-200">
              {{ tag }}
            </li>
          </ul>
        </NuxtLink>
      </li>
    </ul>
  </div>
</template>
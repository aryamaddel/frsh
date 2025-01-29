<script setup lang="ts">
const searchQuery = ref("");
const selectedAuthor = ref("");

const { data: allPostsForMeta } = await useAsyncData("blog-meta", () =>
  queryCollection("blog").all()
);

const { data: filteredPosts } = await useAsyncData(
  `blog-posts-${searchQuery.value}-${selectedAuthor.value}`,
  () => {
    const query = queryCollection("blog")
      .select("id", "author", "date", "title", "description", "path")
      .order("date", "DESC");

    if (searchQuery.value) {
      query.where("title", "LIKE", `%${searchQuery.value}%`);
    }
    if (selectedAuthor.value) {
      query.where("author", "=", selectedAuthor.value);
    }

    return query.all();
  },
  { watch: [searchQuery, selectedAuthor] }
);

const authors = computed(() => [
  ...new Set(allPostsForMeta.value?.map((post) => post.author) || []),
]);
</script>

<template>
  <div class="min-h-screen bg-gray-50">
    <div class="container mx-auto px-4 py-12 max-w-5xl">
      <h1 class="text-4xl font-bold text-center mb-12 text-gray-900">
        Blog Posts
      </h1>

      <div class="mb-8 flex flex-col sm:flex-row gap-4">
        <div class="relative flex-1">
          <input type="search" v-model="searchQuery" placeholder="Search posts..."
            class="w-full p-3 pl-10 bg-white border border-gray-300 rounded-lg shadow-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition-all duration-200" />
          <span class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400">
            🔍
          </span>
        </div>

        <div class="w-full sm:w-64">
          <select v-model="selectedAuthor"
            class="w-full p-3 bg-white border border-gray-300 rounded-lg shadow-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition-all duration-200 appearance-none">
            <option value="">All Authors</option>
            <option v-for="author in authors" :key="author" :value="author">
              {{ author }}
            </option>
          </select>
        </div>
      </div>

      <div v-if="filteredPosts?.length === 0" class="text-center py-12">
        <p class="text-gray-600 text-lg">
          No posts found matching your criteria
        </p>
      </div>

      <ul v-else class="grid gap-6">
        <li v-for="post in filteredPosts" :key="post.id" class="group">
          <NuxtLink :to="post.path"
            class="block p-6 bg-white border border-gray-200 rounded-xl shadow-sm hover:shadow-lg hover:border-blue-200 transition-all duration-200">
            <h2 class="text-2xl font-semibold mb-3 text-gray-900 group-hover:text-blue-600 transition-colors">
              {{ post.title }}
            </h2>
            <p class="text-gray-600 mb-4 line-clamp-2">
              {{ post.description }}
            </p>

            <div class="flex flex-wrap items-center gap-4 text-sm text-gray-500">
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

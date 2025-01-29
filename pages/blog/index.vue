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
      .select('id', "author", "date", "title", "description", "path")
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
  <div class="container mx-auto px-4 py-8 max-w-5xl">
    <h1 class="text-4xl font-bold text-center mb-12 text-gray-800">
      Blog Posts
    </h1>

    <div class="mb-8 space-y-4 sm:space-y-0 sm:flex sm:space-x-4">
      <input type="search" v-model="searchQuery" placeholder="Search posts..."
        class="w-full sm:w-1/2 p-3 border border-gray-200 rounded-lg shadow-sm focus:ring-2 focus:ring-green-500 focus:border-green-500 outline-none transition-colors duration-200" />

      <div class="flex space-x-4 w-full sm:w-1/2">
        <select v-model="selectedAuthor"
          class="w-1/2 p-3 border border-gray-200 rounded-lg shadow-sm focus:ring-2 focus:ring-green-500 focus:border-green-500 outline-none transition-colors duration-200">
          <option value="">All Authors</option>
          <option v-for="author in authors" :key="author" :value="author">
            {{ author }}
          </option>
        </select>
      </div>
    </div>

    <ul class="space-y-6">
      <li v-for="post in filteredPosts" :key="post.id">
        <NuxtLink :to="post.path"
          class="block p-6 bg-white border border-gray-200 rounded-xl shadow-sm hover:shadow-md hover:border-green-200 transition-all duration-200">
          <h2 class="text-2xl font-semibold mb-3 text-gray-800 group-hover:text-green-600">
            {{ post.title }}
          </h2>
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
        </NuxtLink>
      </li>
    </ul>
  </div>
</template>

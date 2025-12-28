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
  <div class="">
    <div class="container mx-auto px-4 py-12 max-w-5xl">
      <h1
        class="text-4xl font-bold text-center mb-12 text-gray-900 dark:text-gray-100"
      >
        Blog Posts
      </h1>

      <div class="mb-8 flex flex-col sm:flex-row gap-4">
        <div class="relative flex-1">
          <input
            type="search"
            v-model="searchQuery"
            placeholder="Search posts..."
            class="w-full p-3 pl-10 bg-white/60 dark:bg-gray-900/60 border border-gray-300 dark:border-gray-700 backdrop-blur-sm focus:border-emerald-500 dark:focus:border-emerald-500 focus:bg-white/80 dark:focus:bg-gray-900/80 outline-none transition-all duration-300 dark:text-gray-100 dark:placeholder-gray-500"
          />
          <span
            class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400 dark:text-gray-500 transition-all duration-300"
          >
            🔍
          </span>
        </div>

        <div class="w-full sm:w-64">
          <select
            v-model="selectedAuthor"
            class="w-full p-3 bg-white/60 dark:bg-gray-900/60 border border-gray-300 dark:border-gray-700 backdrop-blur-sm focus:border-emerald-500 dark:focus:border-emerald-500 focus:bg-white/80 dark:focus:bg-gray-900/80 outline-none transition-all duration-300 appearance-none dark:text-gray-100 cursor-pointer"
          >
            <option value="">All Authors</option>
            <option v-for="author in authors" :key="author" :value="author">
              {{ author }}
            </option>
          </select>
        </div>
      </div>

      <div v-if="filteredPosts?.length === 0" class="text-center py-12">
        <p class="text-gray-600 dark:text-gray-400 text-lg">
          No posts found matching your criteria
        </p>
      </div>

      <ul v-else class="grid gap-6">
        <li v-for="post in filteredPosts" :key="post.id" class="group">
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

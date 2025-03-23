<script setup lang="ts">
const route = useRoute();
const { data: post } = await useAsyncData(`post-${route.path}`, () =>
  queryCollection("blog").path(route.path).first()
);
const { data: surroundData } = await useAsyncData(
  `surround-${route.path}`,
  () => queryCollectionItemSurroundings("blog", route.path)
);
</script>

<template>
  <div class="max-w-3xl mx-auto space-y-8">
    <NuxtLink
      to="/blog"
      class="inline-flex items-center text-green-500 hover:text-green-600 font-medium transition-colors duration-200"
    >
      <span class="mr-2">←</span>
      Back to Blog List
    </NuxtLink>
    <div class="text-gray-500 text-sm space-x-2">
      <span>{{ post?.author }}</span>
      <span>•</span>
      <span>{{
        new Date(
          post?.date || "Publication Date not known"
        ).toLocaleDateString()
      }}</span>
    </div>
    <article class="bg-white rounded-2xl shadow-sm p-6 sm:p-8">
      <ContentRenderer
        v-if="post"
        :value="post"
        class="prose prose-green max-w-none prose-img:rounded-xl prose-headings:text-gray-800 prose-pre:bg-gray-100 prose-pre:text-gray-800 prose-pre:p-4 prose-pre:rounded-lg prose-code:bg-gray-100 prose-code:text-gray-800 prose-code:px-1 prose-code:rounded prose-pre:prose-code:bg-transparent prose-pre:prose-code:p-0"
      />
      <div v-else class="text-gray-500 text-center py-12">Post not found</div>
    </article>
    <nav
      class="flex justify-between items-center pt-8 border-t border-gray-200"
    >
      <NuxtLink
        v-if="surroundData?.[0]"
        :to="surroundData[0].path"
        class="inline-flex items-center px-4 py-2 rounded-lg hover:bg-green-50 text-green-500 hover:text-green-600 transition-all duration-200 active:scale-95"
      >
        <span class="mr-2">←</span>
        {{ surroundData[0].title }}
      </NuxtLink>
      <div v-else class="flex-1"></div>
      <NuxtLink
        v-if="surroundData?.[1]"
        :to="surroundData[1].path"
        class="inline-flex items-center px-4 py-2 rounded-lg hover:bg-green-50 text-green-500 hover:text-green-600 transition-all duration-200 active:scale-95"
      >
        {{ surroundData[1].title }}
        <span class="ml-2">→</span>
      </NuxtLink>
      <div v-else class="flex-1"></div>
    </nav>
  </div>
</template>

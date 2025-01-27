<script setup lang="ts">
const route = useRoute()
const { data: blogPost } = await useAsyncData(() => { return queryCollection('blog').path(route.path).first() })
</script>

<template>
  <NuxtLink to="/blog">Back to blog List</NuxtLink>
  <ContentRenderer v-if="blogPost" :value="blogPost" class="prose" />
  <div v-else>Post not found</div>
</template>
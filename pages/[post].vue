<script setup lang="ts">
const route = useRoute()
const { data: blogPost } = await useAsyncData(route.path, () => {
  return queryCollection('content').path(`/blog${route.path}`).first()
})
</script>

<template>
  <ContentRenderer v-if="blogPost" :value="blogPost" />
  <div v-else>Post not found</div>
</template>

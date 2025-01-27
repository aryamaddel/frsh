<script setup lang="ts">
const route = useRoute()
const { data: post } = await useAsyncData(() => { return queryCollection('blog').path(route.path).first() })
</script>

<template>
  <NuxtLink to="/blog">Back to blog List</NuxtLink>
  <ContentRenderer v-if="post" :value="post" class="prose" />
  <div v-else>Post not found</div>
</template>
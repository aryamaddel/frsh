<script setup lang="ts">
const route = useRoute()
const { data: post } = await useAsyncData(route.path, () => queryCollection('blog').path(route.path).first())

const { data: surroundData } = await useAsyncData('surround', () => queryCollectionItemSurroundings('blog', route.path))

console.dir(surroundData.value)
</script>

<template>
  <NuxtLink to="/blog">Back to blog List</NuxtLink>
  <ContentRenderer v-if="post" :value="post" class="prose" />
  <div v-else>Post not found</div>

  <div class="flex justify-between">
    <NuxtLink v-if="surroundData?.[0]" :to="surroundData[0].path">
      ← {{ surroundData[0].title }}
    </NuxtLink>
    <NuxtLink v-if="surroundData?.[1]" :to="surroundData[1].path">
      {{ surroundData[1].title }} →
    </NuxtLink>
  </div>
</template>
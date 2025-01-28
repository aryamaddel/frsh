<script setup lang="ts">

const searchQuery = ref('')

const { data: posts } = await useAsyncData('blog', () =>
  queryCollection('blog')
    .select('id', 'title','description', 'path', 'date', 'author', 'tags')
    .order('date', 'DESC')
    .all()
)

console.dir(posts.value)

</script>

<template>
  <div class="container mx-auto px-4 py-8">
    <h1 class="text-4xl font-bold text-center mb-8">Blog Posts</h1>

    <input type="search" placeholder="Search" v-model="searchQuery"/>

    <ul>
      <li v-for="post in posts" :key="post.id">
        <NuxtLink :to="post.path">
          <h2>{{ post.title }}</h2>

          <p>
            {{ post.description }}
          </p>

          <p>{{ new Date(post.date).toLocaleDateString() }}</p>
          <p>
            {{ post.author }}
          </p>
          <ul v-for="tag in post.tags">
            <li>{{ tag }}</li>
          </ul>
        </NuxtLink>
      </li>
    </ul>
  </div>
</template>

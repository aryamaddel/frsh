<script setup lang="ts">
const { data: navigationData } = await useAsyncData(() => { return queryCollectionNavigation('content') })

</script>

<template>
    <div class="container mx-auto px-4 py-8">
        <h1 class="text-4xl font-bold text-center mb-8">Blog Posts</h1>
        <nav>
            <ul v-if="navigationData">
                <li v-for="item in navigationData[0].children" :key="item.path">
                    <NuxtLink :to="item.path.replace('/blog', '')">{{ item.title }}</NuxtLink>
                </li>
            </ul>
            <p v-else>No blog posts available.</p>
        </nav>
    </div>
</template>
<template>
    <div class="space-y-12">
        <section class="hero bg-gradient-to-r from-green-500 to-blue-500 text-white py-20">
            <div class="container mx-auto px-4 text-center">
                <h1 class="text-5xl font-bold mb-4">Discover the Future of Technology</h1>
                <p class="text-xl mb-8 max-w-2xl mx-auto">
                    Explore cutting-edge insights, innovative trends, and deep-dive analyses
                    that shape the tech landscape.
                </p>
                <div class="space-x-4">
                    <NuxtLink to="/blog"
                        class="bg-white text-green-600 px-6 py-3 rounded-full font-semibold hover:bg-gray-100 transition">
                        Read Latest Posts
                    </NuxtLink>
                    <NuxtLink to="/about"
                        class="border-2 border-white text-white px-6 py-3 rounded-full hover:bg-white hover:text-green-500 transition">
                        About Us
                    </NuxtLink>
                </div>
            </div>
        </section>

        <section class="featured-posts container mx-auto px-4">
            <h2 class="text-3xl font-bold mb-8 text-center">Featured Articles</h2>
            <div class="grid md:grid-cols-3 gap-8">
                <article v-for="post in featuredPosts" :key="post.id"
                    class="bg-white shadow-md rounded-lg overflow-hidden hover:shadow-xl transition">
                    <img :src="post.imageUrl" :alt="post.title" class="w-full h-48 object-cover" />
                    <div class="p-6">
                        <h3 class="text-xl font-semibold mb-2">{{ post.title }}</h3>
                        <p class="text-gray-600 mb-4">{{ post.excerpt }}</p>
                        <NuxtLink :to="`/blog/${post.slug}`" class="text-green-600 hover:text-green-800 font-medium">
                            Read More →
                        </NuxtLink>
                    </div>
                </article>
            </div>
        </section>

        <section class="topics bg-gray-100 py-16">
            <div class="container mx-auto px-4 text-center">
                <h2 class="text-3xl font-bold mb-8">Explore Our Topics</h2>
                <div class="flex flex-wrap justify-center gap-4">
                    <span v-for="topic in topics" :key="topic"
                        class="bg-white px-4 py-2 rounded-full text-gray-700 shadow-sm hover:bg-green-50 transition">
                        {{ topic }}
                    </span>
                </div>
            </div>
        </section>
    </div>
</template>

<script setup>
const featuredPosts = [
    {
        id: 1,
        title: 'AI and the Future of Work',
        excerpt: 'Exploring how artificial intelligence is reshaping industries and job landscapes.',
        imageUrl: '/api/placeholder/400/250',
        slug: 'ai-future-of-work'
    },
    {
        id: 2,
        title: 'Quantum Computing Breakthroughs',
        excerpt: 'Recent advancements pushing the boundaries of computational power.',
        imageUrl: '/api/placeholder/400/250',
        slug: 'quantum-computing'
    },
    {
        id: 3,
        title: 'Sustainable Tech Innovations',
        excerpt: 'How technology is driving environmental solutions and green initiatives.',
        imageUrl: '/api/placeholder/400/250',
        slug: 'sustainable-tech'
    }
]

const topics = [
    'Artificial Intelligence',
    'Machine Learning',
    'Cloud Computing',
    'Cybersecurity',
    'Web Development',
    'Data Science',
    'Blockchain',
    'Internet of Things'
]
</script>
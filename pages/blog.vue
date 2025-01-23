<template>
    <div class="container mx-auto px-4 py-8">
      <h1 class="text-4xl font-bold mb-8 text-center">Tech Insights & Stories</h1>
  
      <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
        <article 
          v-for="post in blogPosts" 
          :key="post.id" 
          class="bg-white shadow-md rounded-lg overflow-hidden hover:shadow-xl transition"
        >
          <img 
            :src="post.imageUrl" 
            :alt="post.title" 
            class="w-full h-48 object-cover"
          />
          <div class="p-6">
            <div class="flex justify-between items-center mb-4">
              <span class="text-sm text-gray-500">{{ post.date }}</span>
              <span class="text-sm bg-green-100 text-green-800 px-2 py-1 rounded-full">
                {{ post.category }}
              </span>
            </div>
            <h2 class="text-xl font-semibold mb-2">{{ post.title }}</h2>
            <p class="text-gray-600 mb-4">{{ post.excerpt }}</p>
            <div class="flex justify-between items-center">
              <NuxtLink 
                :to="`/blog/${post.slug}`" 
                class="text-green-600 hover:text-green-800 font-medium"
              >
                Read More →
              </NuxtLink>
              <div class="flex items-center space-x-2">
                <img 
                  :src="post.authorAvatar" 
                  :alt="post.author" 
                  class="w-8 h-8 rounded-full"
                />
                <span class="text-sm text-gray-700">{{ post.author }}</span>
              </div>
            </div>
          </div>
        </article>
      </div>
  
      <div class="flex justify-center mt-12">
        <div class="inline-flex space-x-2">
          <button 
            @click="prevPage" 
            :disabled="currentPage === 1"
            class="px-4 py-2 bg-gray-200 rounded disabled:opacity-50"
          >
            Previous
          </button>
          <span class="px-4 py-2 bg-green-500 text-white rounded">
            Page {{ currentPage }}
          </span>
          <button 
            @click="nextPage" 
            :disabled="currentPage === totalPages"
            class="px-4 py-2 bg-gray-200 rounded disabled:opacity-50"
          >
            Next
          </button>
        </div>
      </div>
    </div>
  </template>
  
  <script setup>
  const currentPage = ref(1)
  const postsPerPage = 6
  
  const blogPosts = [
    {
      id: 1,
      title: 'The Rise of Generative AI',
      excerpt: 'Exploring the transformative potential of generative AI across industries.',
      imageUrl: '/api/placeholder/400/250',
      slug: 'generative-ai-rise',
      date: 'January 15, 2025',
      category: 'Artificial Intelligence',
      author: 'Sarah Chen',
      authorAvatar: '/api/placeholder/80/80'
    },
    {
      id: 2,
      title: 'Web3: Beyond the Hype',
      excerpt: 'A critical look at the promises and challenges of decentralized web technologies.',
      imageUrl: '/api/placeholder/400/250',
      slug: 'web3-technology',
      date: 'January 22, 2025',
      category: 'Blockchain',
      author: 'Michael Rodriguez',
      authorAvatar: '/api/placeholder/80/80'
    },
    {
      id: 3,
      title: 'Sustainable Cloud Computing',
      excerpt: 'How major tech companies are reducing their carbon footprint in data centers.',
      imageUrl: '/api/placeholder/400/250',
      slug: 'cloud-sustainability',
      date: 'January 29, 2025',
      category: 'Cloud Computing',
      author: 'Emma Watson',
      authorAvatar: '/api/placeholder/80/80'
    },
    {
      id: 4,
      title: 'Machine Learning in Healthcare',
      excerpt: 'Innovative applications of AI in medical diagnosis and treatment.',
      imageUrl: '/api/placeholder/400/250',
      slug: 'ml-healthcare',
      date: 'February 5, 2025',
      category: 'Machine Learning',
      author: 'Dr. Alex Kim',
      authorAvatar: '/api/placeholder/80/80'
    },
    {
      id: 5,
      title: 'Cybersecurity Trends 2025',
      excerpt: 'Key strategies to protect against emerging digital threats.',
      imageUrl: '/api/placeholder/400/250',
      slug: 'cybersecurity-trends',
      date: 'February 12, 2025',
      category: 'Cybersecurity',
      author: 'David Lee',
      authorAvatar: '/api/placeholder/80/80'
    },
    {
      id: 6,
      title: 'The Future of Quantum Computing',
      excerpt: 'Breakthroughs that are bringing quantum computing closer to reality.',
      imageUrl: '/api/placeholder/400/250',
      slug: 'quantum-computing-future',
      date: 'February 19, 2025',
      category: 'Quantum Computing',
      author: 'Rachel Green',
      authorAvatar: '/api/placeholder/80/80'
    }
  ]
  
  const totalPages = computed(() => Math.ceil(blogPosts.length / postsPerPage))
  
  const nextPage = () => {
    if (currentPage.value < totalPages.value) {
      currentPage.value++
    }
  }
  
  const prevPage = () => {
    if (currentPage.value > 1) {
      currentPage.value--
    }
  }
  </script>
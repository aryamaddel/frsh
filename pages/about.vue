<template>
  <main class="container mx-auto px-4 py-12 max-w-5xl">
    <section class="text-center mb-16">
      <h1 class="text-5xl font-bold mb-6 text-gray-800">We Post</h1>
      <p class="text-xl text-gray-600 max-w-2xl mx-auto">
        A group of passionate Computer Science students from Pune, India,
        working on innovative solutions.
      </p>
    </section>

    <div class="grid md:grid-cols-3 gap-8">
      <article
        class="bg-white border border-gray-200 rounded-xl shadow-sm hover:shadow-md hover:border-green-200 transition-all duration-200"
      >
        <div class="p-6">
          <div class="aspect-square mb-4 overflow-hidden rounded-lg">
            <nuxt-img
              src="team/Arya.jpg"
              alt="Arya Maddel"
              class="w-full h-full object-contain bg-gray-50"
            />
          </div>
          <h3 class="text-2xl font-semibold text-gray-800 mb-3">Arya Maddel</h3>
          <p class="text-gray-600 mb-4">
            Computer Science Student from Pune, India
          </p>
          <div class="flex flex-wrap gap-2">
            <NuxtLink
              to="https://aryamaddel.github.io/"
              target="_blank"
              rel="noopener noreferrer"
              class="inline-flex items-center px-4 py-2 rounded-lg bg-green-50 text-green-600 hover:bg-green-100 transition-colors duration-200"
            >
              Portfolio
              <span class="ml-2">→</span>
            </NuxtLink>
            <NuxtLink
              to="https://github.com/aryamaddel"
              target="_blank"
              rel="noopener noreferrer"
              class="inline-flex items-center px-4 py-2 rounded-lg bg-gray-50 text-gray-600 hover:bg-gray-100 transition-colors duration-200"
            >
              GitHub
              <span class="ml-2">→</span>
            </NuxtLink>
          </div>
        </div>
      </article>

      <article
        class="bg-white border border-gray-200 rounded-xl shadow-sm hover:shadow-md hover:border-green-200 transition-all duration-200"
      >
        <div class="p-6">
          <div class="aspect-square mb-4 overflow-hidden rounded-lg">
            <nuxt-img
              src="team/Anish.jpg"
              alt="Anish Maiseker"
              class="w-full h-full object-contain bg-gray-50"
            />
          </div>
          <h3 class="text-2xl font-semibold text-gray-800 mb-3">
            Anish Maisekar
          </h3>
          <p class="text-gray-600 mb-4">
            Computer Science Student from Pune, India
          </p>
          <div class="flex flex-wrap gap-2">
            <NuxtLink
              to="https://anishmaiseker.github.io/"
              target="_blank"
              rel="noopener noreferrer"
              class="inline-flex items-center px-4 py-2 rounded-lg bg-green-50 text-green-600 hover:bg-green-100 transition-colors duration-200"
            >
              Portfolio
              <span class="ml-2">→</span>
            </NuxtLink>
            <NuxtLink
              to="https://github.com/TheAnishShin"
              target="_blank"
              rel="noopener noreferrer"
              class="inline-flex items-center px-4 py-2 rounded-lg bg-gray-50 text-gray-600 hover:bg-gray-100 transition-colors duration-200"
            >
              GitHub
              <span class="ml-2">→</span>
            </NuxtLink>
          </div>
        </div>
      </article>

      <article
        class="bg-white border border-gray-200 rounded-xl shadow-sm hover:shadow-md hover:border-green-200 transition-all duration-200"
      >
        <div class="p-6">
          <div class="aspect-square mb-4 overflow-hidden rounded-lg">
            <nuxt-img
              src="team/Ruchir.jpg"
              alt="Ruchir Kalohke"
              class="w-full h-full object-contain bg-gray-50"
            />
          </div>
          <h3 class="text-2xl font-semibold text-gray-800 mb-3">
            Ruchir Kalohke
          </h3>
          <p class="text-gray-600 mb-4">AIDS Student from Pune, India</p>
          <div class="flex flex-wrap gap-2">
            <NuxtLink
              to="https://ruchirkalokhe.github.io/portfolio"
              target="_blank"
              rel="noopener noreferrer"
              class="inline-flex items-center px-4 py-2 rounded-lg bg-green-50 text-green-600 hover:bg-green-100 transition-colors duration-200"
            >
              Portfolio
              <span class="ml-2">→</span>
            </NuxtLink>
            <NuxtLink
              to="https://github.com/TheGamingSaga"
              target="_blank"
              rel="noopener noreferrer"
              class="inline-flex items-center px-4 py-2 rounded-lg bg-gray-50 text-gray-600 hover:bg-gray-100 transition-colors duration-200"
            >
              GitHub
              <span class="ml-2">→</span>
            </NuxtLink>
          </div>
        </div>
      </article>

      <article
        class="bg-white border border-gray-200 rounded-xl shadow-sm hover:shadow-md hover:border-green-200 transition-all duration-200 md:col-start-2"
      >
        <div class="p-6">
          <div class="aspect-square mb-4 overflow-hidden rounded-lg">
            <nuxt-img
              src="team/Nimish.jpg"
              alt="Nimish Shinde"
              class="w-full h-full object-contain bg-gray-50"
            />
          </div>
          <h3 class="text-2xl font-semibold text-gray-800 mb-3">
            Nimish Shinde
          </h3>
          <p class="text-gray-600 mb-4">AIDS Student from IIT Madras</p>
          <div class="flex flex-wrap gap-2">
            <NuxtLink
              to="https://ruchirkalokhe.github.io/"
              target="_blank"
              rel="noopener noreferrer"
              class="inline-flex items-center px-4 py-2 rounded-lg bg-green-50 text-green-600 hover:bg-green-100 transition-colors duration-200"
            >
              Portfolio
              <span class="ml-2">→</span>
            </NuxtLink>
            <NuxtLink
              to="https://github.com/whynimoisalive"
              target="_blank"
              rel="noopener noreferrer"
              class="inline-flex items-center px-4 py-2 rounded-lg bg-gray-50 text-gray-600 hover:bg-gray-100 transition-colors duration-200"
            >
              GitHub
              <span class="ml-2">→</span>
            </NuxtLink>
          </div>
        </div>
      </article>
    </div>
  </main>
</template>

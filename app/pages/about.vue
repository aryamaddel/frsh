<script setup lang="ts"></script>

<template>
    <main class="container mx-auto px-4 py-12 max-w-5xl">
        <section class="text-center mb-16">
            <h1
                class="text-5xl font-bold mb-6 text-gray-800 dark:text-gray-100"
            >
                We Post
            </h1>
            <p
                class="text-xl text-gray-600 dark:text-gray-300 max-w-2xl mx-auto"
            >
                A group of passionate Computer Science students from Pune,
                India, working on innovative solutions.
            </p>
        </section>

        <div class="grid md:grid-cols-3 gap-8">
            <TeamMember
                name="Arya Maddel"
                image="team/Arya.jpg"
                link="https://aryamaddel.dev/"
            />
            <TeamMember
                name="Ruchir Kalohke"
                image="team/Ruchir.jpg"
                link="https://x.com/ruchirkalokhe"
            />
        </div>
    </main>
</template>

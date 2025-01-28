import { defineCollection, defineContentConfig, z } from '@nuxt/content'

export default defineContentConfig({
    collections: {
        blog: defineCollection({
            source: 'blog/*.md',
            type: 'page',
            schema: z.object({
                author: z.string(),
                date: z.date(),
                tags: z.array(z.string()),
            })
        })
    }
})

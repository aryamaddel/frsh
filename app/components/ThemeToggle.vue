<script setup lang="ts">
defineProps<{
  isDark: boolean;
}>();

defineEmits<{
  toggle: [];
}>();
</script>

<template>
  <button
    @click="$emit('toggle')"
    title="Toggle theme"
    aria-label="Toggle theme"
    class="fixed top-0 right-0 z-[60] cursor-pointer group transition-transform duration-300"
  >
    <!-- LIGHT MODE -->
    <svg
      v-if="!isDark"
      viewBox="0 0 100 100"
      class="w-16 h-16 text-amber-500 group-hover:text-amber-600 transition-colors duration-500"
    >
      <!-- corner background -->
      <polygon points="0,0 100,0 100,100" fill="currentColor" />

      <!-- ears -->
      <rect x="50" y="8" width="10" height="12" rx="2" fill="#fff4d6" />
      <rect x="74" y="8" width="10" height="12" rx="2" fill="#fff4d6" />

      <!-- head -->
      <rect x="48" y="20" width="40" height="30" rx="4" fill="#fff4d6" />

      <!-- eyes -->
      <rect
        x="58"
        y="32"
        width="4"
        height="4"
        fill="#3a2a00"
        class="origin-center group-hover:scale-y-50 transition-transform duration-300"
      />
      <rect
        x="74"
        y="32"
        width="4"
        height="4"
        fill="#3a2a00"
        class="origin-center group-hover:scale-y-50 transition-transform duration-300"
      />

      <!-- nose -->
      <rect x="67" y="38" width="4" height="3" fill="#d97706" />

      <!-- mouth -->
      <rect x="66" y="42" width="2" height="2" fill="#3a2a00" />
      <rect x="70" y="42" width="2" height="2" fill="#3a2a00" />

      <!-- whiskers -->
      <rect x="54" y="38" width="10" height="1" fill="#3a2a00" />
      <rect x="76" y="38" width="10" height="1" fill="#3a2a00" />

      <!-- paws -->
      <rect x="56" y="50" width="10" height="6" rx="2" fill="#fff4d6" />
      <rect x="70" y="50" width="10" height="6" rx="2" fill="#fff4d6" />

      <!-- tail base -->
      <rect
        x="86"
        y="46"
        width="10"
        height="4"
        rx="2"
        fill="#fff4d6"
        class="group-hover:translate-x-1 transition-transform duration-300"
      />

      <!-- tail tip -->
      <rect
        x="90"
        y="42"
        width="6"
        height="4"
        rx="2"
        fill="#fff4d6"
        class="group-hover:-translate-y-1 transition-transform duration-300"
      />
    </svg>

    <!-- DARK MODE -->
    <svg
      v-else
      viewBox="0 0 100 100"
      class="w-16 h-16 text-indigo-400 group-hover:text-indigo-300 transition-colors duration-500"
    >
      <!-- corner background -->
      <polygon points="0,0 100,0 100,100" fill="currentColor" />

      <!-- ears -->
      <rect x="50" y="8" width="10" height="12" rx="2" fill="#1f2933" />
      <rect x="74" y="8" width="10" height="12" rx="2" fill="#1f2933" />

      <!-- head -->
      <rect x="48" y="20" width="40" height="30" rx="4" fill="#1f2933" />

      <!-- glowing eyes -->
      <rect
        x="58"
        y="32"
        width="4"
        height="4"
        fill="#a5f3fc"
        class="origin-center group-hover:scale-y-50 transition-transform duration-300"
      />
      <rect
        x="74"
        y="32"
        width="4"
        height="4"
        fill="#a5f3fc"
        class="origin-center group-hover:scale-y-50 transition-transform duration-300"
      />

      <!-- nose -->
      <rect x="67" y="38" width="4" height="3" fill="#60a5fa" />

      <!-- mouth -->
      <rect x="66" y="42" width="2" height="2" fill="#a5f3fc" />
      <rect x="70" y="42" width="2" height="2" fill="#a5f3fc" />

      <!-- whiskers -->
      <rect x="54" y="38" width="10" height="1" fill="#a5f3fc" />
      <rect x="76" y="38" width="10" height="1" fill="#a5f3fc" />

      <!-- paws -->
      <rect x="56" y="50" width="10" height="6" rx="2" fill="#1f2933" />
      <rect x="70" y="50" width="10" height="6" rx="2" fill="#1f2933" />

      <!-- tail -->
      <rect
        x="86"
        y="46"
        width="10"
        height="4"
        rx="2"
        fill="#1f2933"
        class="group-hover:translate-x-1 transition-transform duration-300"
      />
      <rect
        x="90"
        y="42"
        width="6"
        height="4"
        rx="2"
        fill="#1f2933"
        class="group-hover:-translate-y-1 transition-transform duration-300"
      />
    </svg>
  </button>
</template>

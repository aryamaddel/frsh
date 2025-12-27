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
    class="fixed top-0 right-0 z-[60] cursor-pointer group"
    aria-label="Toggle theme"
  >
    <!-- Light mode - Sun staircase -->
    <svg
      v-if="!isDark"
      class="w-16 h-16 text-amber-400 hover:text-amber-500 transition-all duration-300"
      viewBox="0 0 100 100"
    >
      <!-- Staircase steps - 8 steps for finer detail -->
      <rect x="0" y="0" width="100" height="12.5" fill="currentColor" />
      <rect x="12.5" y="12.5" width="87.5" height="12.5" fill="currentColor" />
      <rect x="25" y="25" width="75" height="12.5" fill="currentColor" />
      <rect x="37.5" y="37.5" width="62.5" height="12.5" fill="currentColor" />
      <rect x="50" y="50" width="50" height="12.5" fill="currentColor" />
      <rect x="62.5" y="62.5" width="37.5" height="12.5" fill="currentColor" />
      <rect x="75" y="75" width="25" height="12.5" fill="currentColor" />
      <rect x="87.5" y="87.5" width="12.5" height="12.5" fill="currentColor" />
    </svg>

    <!-- Dark mode - Moon staircase -->
    <svg
      v-else
      class="w-16 h-16 text-indigo-400 hover:text-indigo-300 transition-all duration-300"
      viewBox="0 0 100 100"
    >
      <!-- Staircase steps - 8 steps for finer detail -->
      <rect x="0" y="0" width="100" height="12.5" fill="currentColor" />
      <rect x="12.5" y="12.5" width="87.5" height="12.5" fill="currentColor" />
      <rect x="25" y="25" width="75" height="12.5" fill="currentColor" />
      <rect x="37.5" y="37.5" width="62.5" height="12.5" fill="currentColor" />
      <rect x="50" y="50" width="50" height="12.5" fill="currentColor" />
      <rect x="62.5" y="62.5" width="37.5" height="12.5" fill="currentColor" />
      <rect x="75" y="75" width="25" height="12.5" fill="currentColor" />
      <rect x="87.5" y="87.5" width="12.5" height="12.5" fill="currentColor" />
    </svg>
  </button>
</template>

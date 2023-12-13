<template>
  <component :is="typeComponentMap[type]" class="Menu" />
</template>
<script lang="ts" setup>
import Menu from './components/Menu.vue';
defineOptions({ name: 'Menu' });
defineProps({
  type: {
    type: String,
    default: 'Menu',
  },
});

const typeComponentMap = { Menu };
</script>

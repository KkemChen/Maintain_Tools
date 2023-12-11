<template>
    <component :is="typeComponentMap[type]" class="Info" />
</template>
<script lang="ts" setup>
import Info from './components/Info.vue';
defineOptions({ name: 'Info' });
defineProps({
    type: {
        type: String,
        default: 'Info',
    },
});

const typeComponentMap = { Info };
</script>
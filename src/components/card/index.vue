<template>
    <component :is="typeComponentMap[type]" class="v-charts" />
</template>
<script lang="ts" setup>
import Card from './components/Card.vue';
defineProps({
    type: {
        type: String,
        default: 'Card',
    },
});

const typeComponentMap = { Card };
</script>
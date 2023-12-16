<script setup>
defineOptions({ name: 'CustomLoading' });
</script>
<template>
  <div class="custom-loading">
    <svg class="circular" xmlns="http://www.w3.org/2000/svg" viewBox="-10, -10, 50, 50">
      <path
        class="path"
        d=" M 30 15
            L 28 17
            M 25.61 25.61
            A 15 15, 0, 0, 1, 15 30
            A 15 15, 0, 1, 1, 27.99 7.5
            L 15 15"
        style="stroke-width: 4px; fill: rgba(0, 0, 0, 0)"
      />
    </svg>
  </div>
</template>
<style lang="scss" scoped>
.custom-loading {
  .circular {
    margin-right: 3px;
    width: 21px;
    height: 21px;
    animation: loading-rotate 2s linear infinite;
    .path {
      animation: loading-dash 1.5s ease-in-out infinite;
      stroke-dasharray: 90, 150;
      stroke-dashoffset: 0;
      stroke-width: 2;
      stroke: #2f2f2f;
      stroke-linecap: round;
    }
  }
}
</style>

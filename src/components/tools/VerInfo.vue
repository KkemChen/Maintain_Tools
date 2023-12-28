<template>
  <el-table :data="tableData" :show-overflow-tooltip="true" stripe id="myTable">
    <el-table-column fixed prop="name" label="Name" :width="columnWidths.date" />
    <el-table-column prop="date" label="Date" :width="columnWidths.name" />
    <el-table-column prop="commit" label="Commit" :width="columnWidths.address" />
  </el-table>
</template>

<script lang="ts" setup>
import { ref, onMounted, onUnmounted } from 'vue';

interface TableDataItem {
  name: string;
  date: string;
  commit: string;
}

const tableData = ref<TableDataItem[]>([
  {
    name: 'StreamServer',
    date: '2023-12-25',
    commit: '2147b4ff7d4921b3ce0febed0b6c267f161d6d63',
  },
  {
    name: 'StreamServer',
    date: '2023-12-25',
    commit: '2147b4ff7d4921b3ce0febed0b6c267f161d6d63',
  },
  {
    name: 'StreamServer',
    date: '2023-12-25',
    commit: '2147b4ff7d4921b3ce0febed0b6c267f161d6d63',
  },
  {
    name: 'StreamServer',
    date: '2023-12-25',
    commit: '2147b4ff7d4921b3ce0febed0b6c267f161d6d63',
  },
  {
    name: 'StreamServer',
    date: '2023-12-25',
    commit: '2147b4ff7d4921b3ce0febed0b6c267f161d6d63',
  },
  {
    name: 'StreamServer',
    date: '2023-12-25',
    commit: '2147b4ff7d4921b3ce0febed0b6c267f161d6d63',
  },
]);

const columnWidths = ref({
  // date: '25%',
  // name: '25%',
  // address: '50%',
  date: '230px',
  name: '230px',
  address: '460px',
});

// 调整列宽的函数;
const adjustColumnWidths = () => {
  const containerWidth = document.getElementsByClassName('el-tabs__content')[0].offsetWidth - 84; // 获取表格容器的宽度

  columnWidths.value = {
    date: `${containerWidth * 0.25}px`, // 25%
    name: `${containerWidth * 0.25}px`, // 25%
    address: `${containerWidth * 0.5}px`, // 50%
  };
  console.log(columnWidths.value);
};

// 在组件挂载后和窗口大小变化时调整列宽
onMounted(() => {
  window.addEventListener('resize', adjustColumnWidths);
  adjustColumnWidths(); // 初始调整
});

// 在组件卸载前清理事件监听器
onUnmounted(() => {
  window.removeEventListener('resize', adjustColumnWidths);
});
</script>

<style scoped>
:deep(.el-table__header) {
  width: 100%;
}
:deep(.el-scrollbar__view) {
  width: 100%;
}
:deep(.el-table__body) {
  width: 100%;
}
:deep(.el-table__row) {
  width: 100%;
}
tbody {
  width: 100%;
}
</style>

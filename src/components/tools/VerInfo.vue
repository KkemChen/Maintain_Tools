<template>
  <el-table :data="tableData" border style="width: 100%" id="myTable">
    <el-table-column prop="name" label="Name" :width="columnWidths.date" />
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
]);

const columnWidths = ref({
  date: '25%',
  name: '25%',
  address: '50%',
});

// 调整列宽的函数
const adjustColumnWidths = () => {
  const containerWidth = document.getElementById('myTable').offsetWidth - 20; // 获取表格容器的宽度
  columnWidths.value = {
    date: `${containerWidth * 0.25}px`, // 25%
    name: `${containerWidth * 0.25}px`, // 25%
    address: `${containerWidth * 0.5}px`, // 50%
  };
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

<style scoped></style>

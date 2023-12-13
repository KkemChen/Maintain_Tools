<script lang="ts" setup>
import { ref, onMounted, onUnmounted, computed } from 'vue';

// 计算每列的宽度
const nameColumnWidth = ref('');
const otherColumnWidth = ref('');

const updateColumnWidth = () => {
  const tableElement = document.getElementById('Process-Info'); // 使用表格的 ID
  if (tableElement) {
    nameColumnWidth.value = `${(tableElement.clientWidth - 20) * 0.4}px`;
    otherColumnWidth.value = `${(tableElement.clientWidth - 20) * 0.2}px`;
  }
};

onMounted(() => {
  updateColumnWidth();
  window.addEventListener('resize', updateColumnWidth);
});

onUnmounted(() => {
  window.removeEventListener('resize', updateColumnWidth);
});

interface TableDataItem {
  PID: string;
  Name: string;
  CPU: Number;
  Memory: Number;
}

const tableData = ref<TableDataItem[]>([
  { PID: '0', Name: 'systemd', CPU: 0, Memory: 0.3 },
  { PID: '1', Name: 'bash', CPU: 0, Memory: 1.7 },
  { PID: '631', Name: 'atd', CPU: 0, Memory: 652 },
  { PID: '11306', Name: 'nginx', CPU: 0, Memory: 2.4 },
  { PID: '11306', Name: 'nginx', CPU: 0, Memory: 2.4 },
  { PID: '11306', Name: 'nginx', CPU: 0, Memory: 2.4 },
  { PID: '11306', Name: 'nginx', CPU: 0, Memory: 2.4 },
  { PID: '11306', Name: 'nginx', CPU: 0, Memory: 2.4 },
  { PID: '11306', Name: 'nginx', CPU: 0, Memory: 2.4 },
]);

// 格式化函数
const formatMem = (row, column, cellValue, index) => {
  return `${cellValue} MiB`;
};

const formatCpu = (row, column, cellValue, index) => {
  return `${cellValue} %`;
};
</script>

<template>
  <el-table
    :data="tableData"
    height="100%"
    style="width: 100%; font-size: 10px"
    label="auto"
    :cell-style="{ padding: '0px' }"
    id="Process-Info"
  >
    <el-table-column prop="PID" label="PID" :width="otherColumnWidth" />
    <el-table-column prop="Name" label="Name" :width="nameColumnWidth" />
    <el-table-column prop="CPU" label="CPU" :width="otherColumnWidth" :formatter="formatCpu" />
    <el-table-column prop="Memory" label="Memory" :width="otherColumnWidth" :formatter="formatMem" />
  </el-table>
</template>

<style scoped>
.el-table {
  padding: 0px 10px;
}
</style>

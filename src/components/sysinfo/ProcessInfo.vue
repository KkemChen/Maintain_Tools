<script lang="ts" setup>
import { ref, onMounted, onUnmounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api';

// 计算每列的宽度
const nameColumnWidth = ref('');
const otherColumnWidth = ref('');

const updateColumnWidth = () => {
  const tableElement = document.getElementById('Process-Info'); // 使用表格的 ID
  if (tableElement) {
    nameColumnWidth.value = `${(tableElement.clientWidth - 20) * 0.25}px`;
    otherColumnWidth.value = `${(tableElement.clientWidth - 20) * 0.15}px`;
  }
};

interface TableDataItem {
  PID: string;
  Name: string;
  Virt: string;
  Res: string;
  CPU: string;
  Memory: string;
}

const tableData = ref<TableDataItem[]>([
  // { PID: '0', Name: 'systemd', CPU: 0, Memory: 0.3 },
  // { PID: '1', Name: 'bash', CPU: 0, Memory: 1.7 },
  // { PID: '631', Name: 'atd', CPU: 0, Memory: 652 },
  // { PID: '11306', Name: 'nginx', CPU: 0, Memory: 2.4 },
  // { PID: '11306', Name: 'nginx', CPU: 0, Memory: 2.4 },
  // { PID: '11306', Name: 'nginx', CPU: 0, Memory: 2.4 },
  // { PID: '11306', Name: 'nginx', CPU: 0, Memory: 2.4 },
  // { PID: '11306', Name: 'nginx', CPU: 0, Memory: 2.4 },
  // { PID: '11306', Name: 'nginx', CPU: 0, Memory: 2.4 },
]);

const fetchProcessInfo = () => {
  invoke('get_process_info', { host: '192.168.1.172:6622' })
    .then((dataStr) => {
      // console.log(dataStr);
      const data = JSON.parse(dataStr);
      tableData.value = data.map((item: any) => ({
        PID: item.pid,
        Name: item.command,
        Virt: item.virt,
        Res: item.res,
        CPU: item.cpu,
        Memory: item.mem,
      }));
      // console.log(tableData.value);
    })
    .catch((error) => {
      console.error('Error fetching Process info:', error);
    });
};

let intervalId: number | undefined;

// 格式化函数
const formatMem = (row, column, cellValue, index) => {
  return `${cellValue} MiB`;
};

const formatPercentage = (row, column, cellValue, index) => {
  return `${cellValue} %`;
};

onMounted(() => {
  fetchProcessInfo();
  updateColumnWidth();
  intervalId = setInterval(() => {
    fetchProcessInfo(); // 定时获取 CPU 信息
  }, 3000);
  window.addEventListener('resize', updateColumnWidth);
});

onUnmounted(() => {
  if (intervalId !== undefined) {
    clearInterval(intervalId); // 清除定时器
  }
  window.removeEventListener('resize', updateColumnWidth);
});
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
    <el-table-column prop="Virt" label="Virt" :width="otherColumnWidth" />
    <el-table-column prop="Res" label="Res" :width="otherColumnWidth" />
    <el-table-column prop="CPU" label="CPU" :width="otherColumnWidth" :formatter="formatPercentage" />
    <el-table-column prop="Memory" label="Memory" :width="otherColumnWidth" :formatter="formatPercentage" />
  </el-table>
</template>

<style scoped>
.el-table {
  padding: 0px 10px;
}
</style>

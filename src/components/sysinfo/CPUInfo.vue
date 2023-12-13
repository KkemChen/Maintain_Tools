<script lang="ts" setup>
import { ref, onMounted, onUnmounted, nextTick } from 'vue';
import { invoke } from '@tauri-apps/api';

interface TableDataItem {
  Index: string;
  Useage: Number;
}

const tableData = ref<TableDataItem[]>([
  // { Index: "CPU0", Useage: 86 },
  // { Index: "CPU1", Useage: 36 },
  // { Index: "CPU2", Useage: 62 },
  // { Index: "CPU3", Useage: 50 },
  // { Index: "CPU4", Useage: 32 },
  // { Index: "CPU5", Useage: 66 },
]);

const getProgressStatus = (percentage) => {
  if (percentage <= 50) return 'success';
  if (percentage > 50 && percentage < 70) return 'warning';
  return 'exception';
};

const firstColumnWidth = ref(''); // 存储第一列的宽度
const secondColumnWidth = ref(''); // 存储第二列的宽度

const updateColumnWidths = () => {
  const tableElement = document.getElementById('CPU-Info'); // 替换为您的表格元素选择器
  if (tableElement) {
    const tableWidth = tableElement.clientWidth;
    firstColumnWidth.value = `${(tableWidth - 20) * 0.2}px`; // 30% 的宽度
    secondColumnWidth.value = `${(tableWidth - 20) * 0.8}px`; // 70% 的宽度
  }
};

const fetchCPUInfo = () => {
  invoke('get_cpu_info')
    .then((dataStr) => {
      const data = JSON.parse(dataStr);
      tableData.value = data.map((item, index) => ({
        Index: `CPU${index}`,
        Useage: Math.floor(item.usage),
      }));
    })
    .catch((error) => {
      console.error('Error fetching CPU info:', error);
    });
};

let intervalId: number | undefined;

onMounted(() => {
  updateColumnWidths();
  fetchCPUInfo();
  window.addEventListener('resize', updateColumnWidths);

  intervalId = setInterval(() => {
    fetchCPUInfo(); // 定时获取 CPU 信息
    nextTick(() => {
      updateColumnWidths();
    });
  }, 3000);
});

onUnmounted(() => {
  window.removeEventListener('resize', updateColumnWidths);
  if (intervalId !== undefined) {
    clearInterval(intervalId);
  }
});
</script>

<template>
  <el-table
    :data="tableData"
    height="100%"
    style="width: 100%; font-size: 10px"
    label="auto"
    :cell-style="{ padding: '0px' }"
    id="CPU-Info"
  >
    <el-table-column prop="Index" label="CPU" :width="firstColumnWidth" />
    <el-table-column label="Useage" :width="secondColumnWidth">
      <template v-slot="{ row }">
        <el-progress
          :text-inside="true"
          :stroke-width="15"
          :percentage="row.Useage"
          :status="getProgressStatus(row.Useage)"
        />
      </template>
    </el-table-column>
  </el-table>
</template>

<style scoped>
.el-table {
  padding: 0px 10px;
}
</style>

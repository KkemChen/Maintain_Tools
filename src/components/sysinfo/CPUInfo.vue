<script lang="ts" setup>
import { ref, onMounted, onUnmounted, nextTick } from 'vue';

interface TableDataItem {
  index: string;
  usage: number;
}

const props = defineProps({
  option: {
    type: Array<TableDataItem>,
    default: () => [],
  },
});

const tableData = ref<TableDataItem[]>([
  // { Index: "CPU0", usage: 86 },
  // { Index: "CPU1", usage: 36 },
  // { Index: "CPU2", usage: 62 },
  // { Index: "CPU3", usage: 50 },
  // { Index: "CPU4", usage: 32 },
  // { Index: "CPU5", usage: 66 },
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

const assignCPUInfo = () => {
  tableData.value = props.option.map((item, index) => ({
    index: `CPU${index}`,
    usage: Math.floor(item.usage),
  }));
};

let intervalId: number | undefined;

onMounted(() => {
  updateColumnWidths();
  assignCPUInfo();
  window.addEventListener('resize', updateColumnWidths);

  intervalId = setInterval(() => {
    assignCPUInfo();
    nextTick(() => {
      updateColumnWidths();
    });
  }, 2000);
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
    <el-table-column prop="index" label="CPU" :width="firstColumnWidth" />
    <el-table-column label="usage" :width="secondColumnWidth">
      <template v-slot="{ row }">
        <el-progress
          :text-inside="true"
          :stroke-width="15"
          :percentage="row.usage"
          :status="getProgressStatus(row.usage)"
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

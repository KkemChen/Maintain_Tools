<script lang="ts" setup>
import { ref, onMounted, onUnmounted, computed } from 'vue';

interface TableDataItem {
  name: string;
  mounted_on: string;
  size: string;
  used: string;
  avail: string;
}

const props = defineProps({
  option: {
    type: Array<TableDataItem>,
    default: () => [],
  },
});

const tableData = ref<TableDataItem[]>([
  // { Disk: 'sda3', MP: '/', Size: 908, Used: 63, Avail: 844 },
  // { Disk: 'sda2', MP: '/boot', Size: 240, Used: 63, Avail: 177 },
  // { Disk: 'sda1', MP: '/srv', Size: 908, Used: 63, Avail: 844 },
  // { Disk: 'sda0', MP: '/tmp', Size: 240, Used: 63, Avail: 177 },
]);

// 计算每列的宽度
const ColumnWidth = ref('');

const updateColumnWidth = () => {
  const tableElement = document.getElementById('Disk-Info'); // 使用表格的 ID
  if (tableElement) {
    const numberOfColumns = 5; // 列数
    ColumnWidth.value = `${(tableElement.clientWidth - 20) / numberOfColumns}px`;
  }
};

const assignDiskDetail = () => {
  tableData.value = props.option.map((item, index) => ({
    name: item.name,
    mounted_on: item.mounted_on,
    size: item.size,
    used: item.used,
    avail: item.avail,
  }));
};

let intervalId: number | undefined;

onMounted(() => {
  updateColumnWidth();
  assignDiskDetail();
  intervalId = setInterval(() => {
    assignDiskDetail(); // 定时获取 disk 信息
  }, 2000);
  window.addEventListener('resize', updateColumnWidth);
});

onUnmounted(() => {
  if (intervalId !== undefined) {
    clearInterval(intervalId); // 清除定时器
  }
  window.removeEventListener('resize', updateColumnWidth);
});

const rowClassName = ({ row, rowIndex }) => {
  // 嵌入的convertToMega函数
  const convertToMega = (str) => {
    // 检查是否为"0"或以"0"开头的单位（如0M，0G）
    if (str.trim() === '0' || /^0[MG]?$/i.test(str.trim())) {
      return 0;
    }

    const matches = str.match(/(\d+(?:\.\d+)?)([MGT])/i);
    if (!matches) return NaN;

    const num = parseFloat(matches[1]);
    const unit = matches[2].toUpperCase();

    switch (unit) {
      case 'T':
        return num * 1024 * 1024;
      case 'G':
        return num * 1024;
      case 'M':
        return num;
      default:
        return NaN;
    }
  };

  const usedM = convertToMega(row.used);
  const sizeM = convertToMega(row.size);

  // 用于调试

  if (!isNaN(usedM) && !isNaN(sizeM)) {
    const usedRatio = usedM / sizeM;

    if (usedRatio > 0.8) {
      return 'highlight-red';
    } else if (usedRatio > 0.6) {
      return 'highlight-yellow';
    }
  }
  return '';
};
</script>

<template>
  <el-table
    :data="tableData"
    height="100%"
    style="width: 100%; font-size: 10px"
    label="auto"
    :cell-style="{ padding: '0px' }"
    id="Disk-Info"
    :row-class-name="rowClassName"
  >
    <el-table-column prop="name" label="Disk" :width="ColumnWidth" />
    <el-table-column prop="mounted_on" label="MP" :width="ColumnWidth" />
    <el-table-column prop="size" label="Size" :width="ColumnWidth" />
    <el-table-column prop="used" label="Used" :width="ColumnWidth" />
    <el-table-column prop="avail" label="Avail" :width="ColumnWidth" />
  </el-table>
</template>

<style>
.el-table {
  padding: 0px 10px;
}

.highlight-green {
  --el-table-tr-bg-color: #67c23a !important;
}
.highlight-yellow {
  --el-table-tr-bg-color: #e6a23c !important;
}
.highlight-red {
  --el-table-tr-bg-color: #f56c6c !important;
}
</style>

<script lang="ts" setup>
import { ref, onMounted, onUnmounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api';
// 计算每列的宽度
const ColumnWidth = ref('');

const updateColumnWidth = () => {
  const tableElement = document.getElementById('Disk-Info'); // 使用表格的 ID
  if (tableElement) {
    const numberOfColumns = 5; // 列数
    ColumnWidth.value = `${(tableElement.clientWidth - 20) / numberOfColumns}px`;
  }
};

interface TableDataItem {
  Disk: string;
  MP: string;
  Size: string;
  Used: string;
  Avail: string;
}

const tableData = ref<TableDataItem[]>([
  // { Disk: 'sda3', MP: '/', Size: 908, Used: 63, Avail: 844 },
  // { Disk: 'sda2', MP: '/boot', Size: 240, Used: 63, Avail: 177 },
  // { Disk: 'sda1', MP: '/srv', Size: 908, Used: 63, Avail: 844 },
  // { Disk: 'sda0', MP: '/tmp', Size: 240, Used: 63, Avail: 177 },
]);

const fetchDiskInfo = () => {
  invoke('get_disk_info', { host: '192.168.1.172:6622' })
    .then((dataStr) => {
      // console.log(dataStr);
      const data = JSON.parse(dataStr);
      tableData.value = data.map((item: any) => ({
        Disk: item.name,
        MP: item.mounted_on,
        Size: item.size,
        Used: item.used,
        Avail: item.avail,
      }));
      // console.log(tableData.value);
    })
    .catch((error) => {
      console.error('Error fetching disk info:', error);
    });
};

let intervalId: number | undefined;

onMounted(() => {
  updateColumnWidth();
  fetchDiskInfo();
  intervalId = setInterval(() => {
    fetchDiskInfo(); // 定时获取 disk 信息
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
    id="Disk-Info"
  >
    <el-table-column prop="Disk" label="Disk" :width="ColumnWidth" />
    <el-table-column prop="MP" label="MP" :width="ColumnWidth" />
    <el-table-column prop="Size" label="Size" :width="ColumnWidth" />
    <el-table-column prop="Used" label="Used" :width="ColumnWidth" />
    <el-table-column prop="Avail" label="Avail" :width="ColumnWidth" />
  </el-table>
</template>

<style scoped>
.el-table {
  padding: 0px 10px;
}
</style>

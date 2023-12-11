<script lang="ts" setup>
import { ref, onMounted, onUnmounted, computed } from 'vue';

// 计算每列的宽度
const ColumnWidth = ref('');

const updateColumnWidth = () => {
  const tableElement = document.getElementById('Disk-Info'); // 使用表格的 ID
  if (tableElement) {
    const numberOfColumns = 5; // 列数
    ColumnWidth.value = `${(tableElement.clientWidth-20) / numberOfColumns}px`;
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
    Disk: string;
    MP: string;
    Size: Number;
    Used: Number;
    Avail: Number;
}

const tableData = ref<TableDataItem[]>([
  { Disk: 'sda3', MP: '/', Size: 908, Used: 63, Avail: 844},
  { Disk: 'sda2', MP: '/boot', Size: 240, Used: 63, Avail: 177},
  { Disk: 'sda1', MP: '/srv', Size: 908, Used: 63, Avail: 844},
  { Disk: 'sda0', MP: '/tmp', Size: 240, Used: 63, Avail: 177}
]);

// 格式化函数
const format = (row, column, cellValue, index) => {
  return `${cellValue} G`;
};

</script>


<template>
    <el-table :data="tableData" height="100%" style="width: 100%; font-size: 10px;" label="auto" 
    :cell-style="{padding:'0px'}" id="Disk-Info">
      <el-table-column prop="Disk" label="Disk"  :width="ColumnWidth"/>
      <el-table-column prop="MP" label="MP"  :width="ColumnWidth"/>
      <el-table-column prop="Size" label="Size"  :width="ColumnWidth" :formatter="format"/>
      <el-table-column prop="Used" label="Used"  :width="ColumnWidth" :formatter="format"/>
      <el-table-column prop="Avail" label="Avail"  :width="ColumnWidth" :formatter="format"/>
      
    </el-table>
</template>

<style scoped>
.el-table{
  padding: 0px 10px;
}
</style>
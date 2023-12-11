<script setup>
// const ColumnWidth = ref(''); 

// const updateColumnWidths = () => {
//   const tableElement = document.querySelector('#Disk-info'); // 替换为您的表格元素选择器
//   if (tableElement) {
//     const tableWidth = tableElement.clientWidth;
//     ColumnWidth.value = `${tableWidth * 0.16}px`;  
//   }
// };

// onMounted(() => {
//   updateColumnWidths();
//   window.addEventListener('resize', updateColumnWidths);
// });

// onUnmounted(() => {
//   window.removeEventListener('resize', updateColumnWidths);
// });

</script>


<template>
    <el-table :data="tableData" height="100%" style="width: 100%; font-size: 10px;" label="auto" 
    :cell-style="{padding:'0px'}" id="Disk-info">
      <el-table-column prop="Disk" label="Disk"  :width="ColumnWidth"/>
      <el-table-column prop="MP" label="MP"  :width="ColumnWidth"/>
      <el-table-column prop="FS" label="FS"  :width="ColumnWidth"/>
      <el-table-column prop="Size" label="Size"  :width="ColumnWidth"/>
      <el-table-column prop="Used" label="Used"  :width="ColumnWidth"/>
      <el-table-column prop="Avail" label="Avail"  :width="ColumnWidth"/>
      
    </el-table>
    test
</template>
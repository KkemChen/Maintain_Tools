<script setup>
import { ref,onMounted,onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const tableData = ref([]);      //存储从RUST获取的表格数据

async function fetchTableData() {
    try {
        const jsonStr = await invoke("get_cpu_info");
        const data = JSON.parse(jsonStr);
        console.log(data);
        tableData.value = data;
  } catch (error) {
      console.error("Error fetching table data: ", error);
  }
}

let intervalId;

onMounted(() => {
  // 在组件挂载后立即调用一次，并设置定时器
  fetchTableData();
  intervalId = setInterval(fetchTableData, 5000); // 每隔 5000 毫秒（5秒）调用一次
});

onUnmounted(() => {
  // 在组件卸载时清除定时器
  clearInterval(intervalId);
});
</script>


<template>
    <table class="styled-table">
        <tr v-for="(row,rowIndex) in tableData" :key="rowIndex">
        <td v-for="(cell,cellIndex) in row" :key="cellIndex">
            {{ cell }}
        </td>
    </tr>
    </table>
</template>
  

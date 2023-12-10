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
  
<style scoped>
.styled-table {
    width: 100%;
    border-collapse: collapse;
    margin: 25px 0;
    font-size: 0.9em;
    font-family: sans-serif;
    min-width: 400px;
    box-shadow: 0 0 20px rgba(0, 0, 0, 0.15);
}

.styled-table thead tr {
    background-color: #009879;
    color: #ffffff;
    text-align: left;
}

.styled-table th,
.styled-table td {
    padding: 12px 15px;
}

.styled-table tbody tr {
    border-bottom: 1px solid #dddddd;
}

.styled-table tbody tr:nth-of-type(even) {
    background-color: #f3f3f3;
}

.styled-table tbody tr:last-of-type {
    border-bottom: 2px solid #009879;
}

.styled-table tbody tr.active-row {
    font-weight: bold;
    color: #009879;
}
</style>
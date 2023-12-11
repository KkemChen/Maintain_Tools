<script setup>
import CPUInfo from "../components/sysinfo/CPUInfo.vue";
import DiskInfo from "../components/sysinfo/DiskInfo.vue";
import ProcessInfo from "../components/sysinfo/ProcessInfo.vue";
import Pie from "../components/v-charts/index.vue";
import { ref, onMounted, nextTick } from "vue";
const chartsOption = ref({
  cpuChart: {
    data: 65.89,
  },
  memoryChart: null,
  loadChart: null,
  diskChart: null,
});
onMounted(() => {
  setInterval(() => {
    nextTick(() => {
      chartsOption.value.cpuChart.data = (Math.random() * 100).toFixed(2);
      console.log(chartsOption.value);
    });
  }, 1000);
});
</script>

<template>
  <el-row :gutter="10">
    <el-col :span="24"
      ><div class="grid-content ep-bg-purple">
        <el-card class="box-card">
          <Pie :option="chartsOption.cpuChart" />
        </el-card>
      </div>
    </el-col>
  </el-row>
  <el-row :gutter="5">
    <el-col :span="8"
      ><div class="grid-content ep-bg-purple">
        <el-card class="box-card">
          <CPUInfo />
        </el-card></div
    ></el-col>
    <el-col :span="8"
      ><div class="grid-content ep-bg-purple">
        <el-card class="box-card">
          <DiskInfo />
        </el-card></div
    ></el-col>
    <el-col :span="8"
      ><div class="grid-content ep-bg-purple">
        <el-card class="box-card">
          <div v-for="o in 4" :key="o" class="text item">
            {{ "List item " + o }}
          </div>
        </el-card>
      </div></el-col
    >
  </el-row>
  <el-row :gutter="10">
    <el-col :span="24"
      ><div class="grid-content ep-bg-purple">
        <el-card class="box-card">
          <ProcessInfo />
        </el-card></div
    ></el-col>
  </el-row>
</template>

<style scoped>
.el-row {
  height: calc((calc(95%) - 15px) / 3);
  margin-bottom: 5px;
}

.el-col {
  height: 100%;
}
.grid-content {
  border: none;
  height: 100%;
}

.box-card {
  width: 100%;
  height: 100%;
  background-color: lightgreen;
}

:deep(.el-card__body) {
  padding: 0px !important;
  height: 100%;
}
</style>

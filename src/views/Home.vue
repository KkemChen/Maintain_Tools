<script setup>
import CPUInfo from "../components/sysinfo/CPUInfo.vue";
import DiskInfo from "../components/sysinfo/DiskInfo.vue";
import ProcessInfo from "../components/sysinfo/ProcessInfo.vue";
import Pie from "../components/v-charts/index.vue";
import IOInfo from "../components/sysinfo/IOInfo.vue"
import { ref, onMounted, nextTick } from "vue";

const pieWidth = '200px';

const chartsOption = ref({
  cpuChart: {
    title: 'CPU',
    data: 65.89
  },
  memoryChart: {
    title: 'Memory',
    data: 65.89
  },
  loadChart: {
    title: 'Load',
    data: 65.89
  },
  diskChart: {
    title: 'Disk',
    data: 65.89
  },
});
onMounted(() => {
  setInterval(() => {
    nextTick(() => {
      chartsOption.value.cpuChart.data = (Math.random() * 100).toFixed(2);
      chartsOption.value.memoryChart.data = (Math.random() * 100).toFixed(2);
      chartsOption.value.loadChart.data = (Math.random() * 100).toFixed(2);
      chartsOption.value.diskChart.data = (Math.random() * 100).toFixed(2);
    });
  }, 1000);
});
</script>

<template>
  <el-row :gutter="10">
    <el-col :span="24"
      ><div class="grid-content ep-bg-purple">
        <el-card class="box-card">
          <div class="flex-pie">
            <Pie id="cpu" :width="pieWidth" :option="chartsOption.cpuChart" />
            <Pie id="memory" :width="pieWidth" :option="chartsOption.memoryChart" />
            <Pie id="load" :width="pieWidth" :option="chartsOption.loadChart" />
            <Pie id="disk" :width="pieWidth" :option="chartsOption.diskChart" />
          </div>
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
          <IOInfo />
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

<style scoped lang="scss">
.el-row {
  height: calc((calc(95%) - 15px) / 3);
  margin-bottom: 5px;
  .el-col {
    height: 100%;
    .grid-content {
      border: none;
      height: 100%;
        .box-card {
          width: 100%;
          height: 100%;
          /* background-color: lightgreen; */
        }
        .flex-pie {
          display: flex;
          flex-direction: row;
          justify-content: flex-start;
          flex-wrap: nowrap;
        }
        :deep(.el-card__body) {
          padding: 0px !important;
          height: 100%;
        }
    }
  }
}
</style>

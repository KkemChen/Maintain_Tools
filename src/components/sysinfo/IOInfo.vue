<script lang="ts" setup>
import * as echarts from 'echarts';
import { onMounted, onUnmounted, ref, watch } from 'vue';

interface TableDataItem {
  device: string;
  receive: string | number;
  transmit: string | number;
}

const props = defineProps({
  option: {
    type: Array<TableDataItem>,
    default: () => [],
  },
});

let myChart: echarts.ECharts;

const chartData = ref({
  xAxisData: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'],
  inputData: [0, 0, 0, 0, 0, 0, 0],
  outputData: [0, 0, 0, 0, 0, 0, 0],
});

let intervalId: number | undefined;

onMounted(() => {
  const chartDom = document.getElementById('IO-Info');
  if (chartDom) {
    myChart = echarts.init(chartDom);
    updateChart();
  }

  intervalId = setInterval(() => {
    updateChartData(props.option);
  }, 2000);

  const resizeChart = () => {
    if (myChart) {
      myChart.resize();
    }
  };

  window.addEventListener('resize', resizeChart);
});

const updateChartData = (newData: any[]) => {
  // 计算新的 inputData 和 outputData
  let totalInputData = 0;
  let totalOutputData = 0;
  newData.forEach((item: { receive: number; transmit: number }) => {
    totalInputData += item.receive;
    totalOutputData += item.transmit;
  });

  // 移除第一个元素并添加新计算的数据到末尾
  chartData.value.inputData.shift();
  chartData.value.outputData.shift();
  chartData.value.inputData.push(totalInputData);
  chartData.value.outputData.push(totalOutputData);

  // 更新图表
  updateChart();
};

const updateChart = () => {
  if (myChart) {
    const option = {
      title: {
        text: '',
      },
      tooltip: {
        trigger: 'axis',
        axisPointer: {
          type: 'cross',
          label: {
            backgroundColor: '#6a7985',
          },
        },
      },
      legend: {
        data: ['Input', 'Output'],
      },
      toolbox: {
        feature: {
          saveAsImage: false,
        },
      },
      grid: {
        left: '3%',
        right: '4%',
        bottom: '3%',
        containLabel: true,
      },
      xAxis: [
        {
          type: 'category',
          boundaryGap: false,
          data: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'],
        },
      ],
      yAxis: [
        {
          type: 'value',
        },
      ],
      series: [
        {
          name: 'Input',
          type: 'line',
          stack: 'Total',
          areaStyle: {},
          emphasis: {
            focus: 'series',
          },
          data: chartData.value.inputData,
        },
        {
          name: 'Output',
          type: 'line',
          stack: 'Total',
          areaStyle: {},
          emphasis: {
            focus: 'series',
          },
          data: chartData.value.outputData,
        },
      ],
    };
    myChart.setOption(option);
  }
};

// watch(chartData, updateChart);

onUnmounted(() => {
  if (myChart) {
    myChart.dispose();
  }
  if (intervalId !== undefined) {
    clearInterval(intervalId); // 清除定时器
  }
});
</script>

<template>
  <div id="IO-Info" style="height: 100%"></div>
</template>

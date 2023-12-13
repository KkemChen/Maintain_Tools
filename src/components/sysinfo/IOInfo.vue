<script setup>
import * as echarts from 'echarts';
import { onMounted, onUnmounted } from 'vue';

let myChart = null;

onMounted(() => {
  const chartDom = document.getElementById('IO-Info');
  if (chartDom) {
    myChart = echarts.init(chartDom);
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
          data: [120, 132, 101, 134, 90, 230, 210],
        },
        {
          name: 'Output',
          type: 'line',
          stack: 'Total',
          areaStyle: {},
          emphasis: {
            focus: 'series',
          },
          data: [220, 182, 191, 234, 290, 330, 310],
        },
      ],
    };
    myChart.setOption(option);
  }

  const resizeChart = () => {
    if (myChart) {
      myChart.resize();
    }
  };

  window.addEventListener('resize', resizeChart);
});

onUnmounted(() => {
  if (myChart) {
    myChart.dispose();
  }
});
</script>

<template>
  <div id="IO-Info" style="height: 100%"></div>
</template>

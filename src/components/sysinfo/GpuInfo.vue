<script lang="ts" setup>
import * as echarts from 'echarts';
import { onMounted, onUnmounted, ref, watch } from 'vue';

interface TableDataItem {
  name: string;
  index: number;
  usage: number;
  total_memory: number;
  used_memory: number;
  power: number;
  power_limit: number;
}

const props = defineProps({
  option: {
    type: Array<TableDataItem>,
    default: () => [],
  },
});
type BarLabelOption = NonNullable<echarts.BarSeriesOption['label']>;

const labelOption: BarLabelOption = {
  show: true,
  position: 'inside' as BarLabelOption['position'],
  distance: 15 as BarLabelOption['distance'],
  align: 'left' as BarLabelOption['align'],
  verticalAlign: 'middle' as BarLabelOption['verticalAlign'],
  rotate: 90 as BarLabelOption['rotate'],
  formatter: '{c}  {name|{a}}',
  fontSize: 13,
  rich: {
    name: {},
  },
};

const chartData = ref<{
  devName: string[];
  series: any[]; // 根据你的需要，这里也可以指定具体的类型
}>({
  devName: [],
  series: [
    {
      name: 'usage',
      type: 'bar',
      barGap: 0,
      label: labelOption,
      emphasis: {
        focus: 'series',
      },
      data: [],
      color: '#409EFF',
    },
    {
      name: 'mem',
      type: 'bar',
      label: labelOption,
      emphasis: {
        focus: 'series',
      },
      data: [],
      color: '#67C23A',
    },
    {
      name: 'power',
      type: 'bar',
      label: labelOption,
      emphasis: {
        focus: 'series',
      },
      data: [],
      color: '#E6A23C',
    },
  ],
});

let myChart: echarts.ECharts;

const updateChart = () => {
  if (myChart) {
    const chartOption = {
      /*       title: {
        text: 'GPU info',
        left: 'center',
        textStyle: {
          // 主标题文本的样式
          color: '#b1b3b8',
          fontSize: 10,
        },
      }, */
      animation: false,
      tooltip: {
        trigger: 'axis',
        axisPointer: {
          type: 'shadow',
        },
      },
      legend: {
        data: ['usage', 'mem', 'power'],
        top: '3%',
      },
      toolbox: {
        show: false,
        orient: 'vertical',
        left: 'right',
        top: 'center',
        feature: {
          mark: { show: true },
          dataView: { show: true, readOnly: false },
          magicType: { show: true, type: ['line', 'bar', 'stack'] },
          restore: { show: true },
          saveAsImage: { show: true },
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
          axisTick: { show: false },
          data: chartData.value.devName,
        },
      ],
      yAxis: [
        {
          type: 'value',
          axisLabel: {
            show: false, // 将 show 设置为 false，隐藏数值标签
          },
        },
      ],
      series: chartData.value.series,
    };
    myChart.setOption(chartOption);
  }
};

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

function extractLastNumber(str) {
  const match = str.match(/\d+$/); // 匹配末尾的数字
  return match ? match[0] : ''; // 返回匹配到的数字，如果没有则返回空字符串
}

const updateChartData = (newData: any[]) => {
  console.log(newData);

  chartData.value.devName = [];
  chartData.value.series[0].data = [];
  chartData.value.series[1].data = [];
  chartData.value.series[2].data = [];
  props.option.forEach((item: TableDataItem, index) => {
    chartData.value.devName.push(extractLastNumber(item.name));
    // 对于使用率（usage）
    chartData.value.series[0].data.push({
      value: item.usage, // 柱状条的高度
      label: {
        show: true,
        formatter: '{c} %', // 格式化显示的内容，{c} 表示该数据点的值
      },
    });

    // 对于内存使用率
    const memUsage = ((item.used_memory / item.total_memory) * 100).toFixed(1);

    chartData.value.series[1].data.push({
      value: memUsage, // 柱状条的高度（内存使用率）
      label: {
        show: true,
        formatter: () => `${item.used_memory} MiB`, // 显示实际使用的内存
      },
    });

    // 对于功率使用率
    const powerUsage = ((item.power / item.power_limit) * 100).toFixed(1);
    chartData.value.series[2].data.push({
      value: powerUsage, // 柱状条的高度（功率使用率）
      label: {
        show: true,
        formatter: () => `${item.power} W`, // 格式化显示的内容，显示百分比
      },
    });
  });

  // 更新图表
  updateChart();
};

watch(chartData, updateChart);

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

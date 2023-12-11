<script setup>
// import Card from "../components/Card.vue"
import Pie from '../components/Pie.vue'
import { ref, onMounted, nextTick } from "vue"

const chartsOption = ref({
    cpuChart: {
        data: 65.89
    },
    memoryChart: null,
    loadChart: null,
    diskChart: null
})

onMounted(() => {
    setInterval(()=>{
        
        nextTick(() => {
            chartsOption.value.cpuChart.data = (Math.random() * 100).toFixed(2);
        });
    }, 1000);
});


</script>

<template>
    <!-- <h1>home</h1> -->
    
    <div class="main-container">
        <el-row :gutter="12" class="main-row">
            <el-col :span="6" class = "pie-col">
                <!-- <img src="../assets/hamburger.png"/> -->
                <Pie id="test" :option="option"/>
            </el-col>
            <el-col :span="6" class = "pie-col">
                <img src="../assets/hamburger.png"/>
            </el-col>
            <el-col :span="6" class = "pie-col">
                <img src="../assets/hamburger.png"/>
            </el-col>
            <el-col :span="6" class = "pie-col">
                <img src="../assets/hamburger.png"/>
            </el-col>
        </el-row>
        <el-row :gutter="10">
            <el-col :span="24">
                <div class="grid-content ep-bg-purple">
                    <el-card class="box-card">
                        <div v-for="o in 4" :key="o" class="text item">{{ 'List item ' + o }}</div>
                    </el-card>
                </div>
            </el-col>
        </el-row>
        <el-row :gutter="5">
            <el-col :span="8" >
                <div class="grid-content ep-bg-purple" >
                    <el-card class="box-card">
                    <div v-for="o in 4" :key="o" class="text item">{{ 'List item ' + o }}</div>
                    </el-card>
                </div>
            </el-col>
            <el-col :span="8" >
                <div class="grid-content ep-bg-purple" >
                    <el-card class="box-card">
                    <div v-for="o in 4" :key="o" class="text item">{{ 'List item ' + o }}</div>
                    </el-card>
                </div>
            </el-col>
            <el-col :span="8" >
                <div class="grid-content ep-bg-purple" >
                    <el-card class="box-card">
                    <div v-for="o in 4" :key="o" class="text item">{{ 'List item ' + o }}</div>
                    </el-card>
                </div>
            </el-col>
        </el-row>
        <el-row :gutter="10">
            <el-col :span="24">
                <div class="grid-content ep-bg-purple" >
                    <el-card class="box-card">
                    <div v-for="o in 4" :key="o" class="text item">{{ 'List item ' + o }}</div>
                    </el-card>
                </div>
            </el-col>
        </el-row>
    </div>

</template>


<style scoped>
.el-row {
  /* margin-bottom: 20px; */
  min-height: calc((calc(95%) - 15px) /3) ;
  margin-bottom: 5px;
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


</style>
  
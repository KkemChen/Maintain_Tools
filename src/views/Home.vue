<script setup>
// import Card from "../components/Card.vue"
import {ref } from "vue"
// const height = "33%";
const option = ref({
    data: 65.62,
    id:"test"
})
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
        <el-row :gutter="12" class="main-row">
            <el-card shadow="always"> Always </el-card>
        </el-row>
        <el-row :gutter="12" class="main-row">
            <el-card shadow="always"> Always </el-card>
        </el-row>
    </div>

</template>


<style scoped>

.main-container {
  display: flex;
  flex-direction: column;
  height: calc(100% - 60px); /* 调整为所需的高度 */
}

.main-row {
  flex: 1; /* 均分空间 */
  height: 33%;
  margin-top: 5px;
}

.pie-col{
    height: 100%;
    padding: 5px;
}

.pie-card{
    height: 100%;
}

img {
  height: 100%;
  width: auto;
  object-fit: contain;
  display: block; 
}


</style>
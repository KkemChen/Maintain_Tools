
<template>
    <div>
        <el-card :body-style="{ height: height }" class="home-card">
            <div class="header">
                <span>{{ header }}</span>
                <slot name="header-r" />
            </div>

            <div class="body">
                <slot name="body"></slot>
            </div>
        </el-card>
    </div>
</template>

<script setup lang="ts">
defineOptions({ name: 'CardWithHeader' });
defineProps({
    header: String,
    height: String,
});
</script>

<style scoped lang="scss">
.home-card {
     height: 100%;
     //width: 100%;
     .body{
        width: 100%;
     }
    .header {
        span {
            position: relative;
            font-size: 16px;
            font-weight: 500;
            margin-left: 18px;
            &::before {
                position: absolute;
                top: 4px;
                left: -13px;
                width: 4px;
                height: 14px;
                content: '';
                background: darkgray;
                border-radius: 10px;
            }
        }
    }

    div:nth-child(2) {
        margin-top: 20px;
    }
}
</style>
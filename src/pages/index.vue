<script setup lang="ts">

import {onMounted, ref} from "vue";
import CountTo from "@/components/CountTo.vue";

const panels = ref([
  {
    title: '支付订单',
    subTitle: '总订单量',
    value: 232,
    unit: '笔',
    subValue: '11',
  },
  {
    title: '支付订单2',
    subTitle: '总订单量2',
    value: 453,
    unit: '笔',
    subValue: '10000',
  },
  {
    title: '支付订单3',
    subTitle: '总订单量3',
    value: 322,
    unit: '笔',
    subValue: '10000',
  },
  {
    title: '支付订单4',
    subTitle: '总订单量4',
    value: 532,
    unit: '笔',
    subValue: '10000',
  },
])

let show = ref(true);

onMounted(()=>{
  setTimeout(() => {
    show.value = false;
  },3000)
})

</script>

<template>
  <div>
    <el-row :gutter="20">
      <template v-if="show">
      <el-col :span="6" v-for="i in 4" :key="i">
        <el-skeleton style="height: 100px" animated loading>
          <template #template>
            <el-card shadow="hover" class="border-0">
              <template #header>
                <div class="flex justify-between">
                  <el-skeleton-item variant="text" style="width: 50%"></el-skeleton-item>
                  <el-skeleton-item variant="text" style="width: 10%"></el-skeleton-item>
                </div>
              </template>
              <span class="text-3xl font-bold text-gray-500">
             <el-skeleton-item variant="h3" style="width: 70%"></el-skeleton-item>
          </span>
              <el-divider/>
              <div class="flex justify-between text-sm text-gray-500">
                <el-skeleton-item variant="text" style="width: 50%"></el-skeleton-item>
                <el-skeleton-item variant="text" style="width: 10%"></el-skeleton-item>
              </div>
            </el-card>

          </template>
        </el-skeleton>
      </el-col>
      </template>
      <el-col :span="6" :offset="0" v-for="(item,index) in panels" :key="index" v-else>
        <el-card shadow="hover" class="border-0">
          <template #header>
            <div class="flex justify-between">
              <span class="text-sm">{{ item.title }}</span>
              <el-tag effect="plain" type="info">{{ item.unit }}</el-tag>
            </div>
          </template>
          <span class="text-3xl font-bold text-gray-500">
<!--            {{ item.value }}-->
            <CountTo :value="item.value"></CountTo>
          </span>
          <el-divider/>
          <div class="flex justify-between text-sm text-gray-500">
            <span>{{ item.subTitle }}</span>
            <span>{{ item.subValue }}</span>
          </div>
        </el-card>
      </el-col>
    </el-row>
  </div>


</template>

<style scoped>

</style>
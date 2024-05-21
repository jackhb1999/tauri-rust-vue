<script lang="ts">
import {defineComponent, reactive, toRefs} from 'vue'
import {getGoodsList} from "../api/tpi.ts";
import {InitData} from "../type/goods.ts";

export default defineComponent({
  setup(){
    const data = reactive(new InitData())
    getGoodsList().then(res =>{
      console.log(8,res)
    })
    return {...toRefs((data))}
  }
})
</script>

<template>
<div>
  <div class="select-box">
    <el-form :inline="true" :model="selectData" class="demo-form-inline">
      <el-form-item label="标题">
        <el-input v-model="selectData.title" placeholder="请输入关键字" clearable />
      </el-form-item>
      <el-form-item label="详情">
        <el-input v-model="selectData.introduce" placeholder="请输入关键字" clearable />
      </el-form-item>
      <el-form-item>
        <el-button type="primary" @click="onSubmit">查询</el-button>
      </el-form-item>
    </el-form>
  </div>
</div>
</template>

<style scoped>

</style>
<script lang="ts">
import {defineComponent} from 'vue'
import {getGoodsList} from "../api/tpi.ts";

export default defineComponent({
  setup(){
    getGoodsList().then(res =>{
      console.log(8,res)
    })
    return {}
  }
})
</script>

<template>
<div>
  商品列表
</div>
</template>

<style scoped>

</style>
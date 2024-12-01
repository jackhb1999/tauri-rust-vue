<script lang="ts">
import {computed, defineComponent, onMounted, toRefs, watch} from 'vue'



export default defineComponent({
  setup() {

    const click = () => {
      window.location.href='https://weread.qq.com'
    }

    return {click}
  }
})
</script>

<template>
  <el-card style="max-width: 480px">
    <el-button
        :type="info"
        link
        @click="click"
    >
            跳转
    </el-button>
  </el-card>
  <div class="ml-3">
    <p class="text-sm font-medium text-gray-900">fan</p>
    <p class="text-sm text-gray-500">jack.h.b.fan@outlook.com</p>
  </div>
</template>

<style scoped>
</style>
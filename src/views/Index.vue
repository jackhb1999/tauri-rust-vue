<script lang="ts">
import {computed, defineComponent, onMounted, reactive, toRefs, watch} from 'vue'



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
</template>

<style scoped>

</style>
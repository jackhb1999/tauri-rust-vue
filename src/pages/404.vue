<script setup lang="ts">

</script>

<template>
  <el-result
      icon="warning"
      title="404提示"
      sub-title="您找的页面暂时无法查看"
  >
    <template #extra>
      <el-button type="primary" @click="$router.push('/')">回到首页</el-button>
    </template>
  </el-result>
</template>

<style scoped>

</style>
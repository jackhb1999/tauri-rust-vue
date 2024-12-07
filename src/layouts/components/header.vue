<script setup lang="ts">

</script>

<template>
  <div class="flex">
  <span>
    <el-icon><Compass/></el-icon>
    Arc
  </span>
    <el-icon>
      <Fold/>
    </el-icon>
    <el-icon>
      <Refresh/>
    </el-icon>
    <div>
      <el-icon>
        <FullScreen/>
      </el-icon>
      <el-dropdown>
    <span class="el-dropdown-link">
        <el-avatar :size="25" src="https://cube.elemecdn.com/3/7c/3ea6beec64369c2642b92c6726f1epng.png"/>
      未知用户名
      <el-icon class="el-icon--right">
        <arrow-down/>
      </el-icon>
    </span>
        <template #dropdown>
          <el-dropdown-menu>
            <el-dropdown-item>Action 1</el-dropdown-item>
            <el-dropdown-item>Action 2</el-dropdown-item>
            <el-dropdown-item>Action 3</el-dropdown-item>
            <el-dropdown-item disabled>Action 4</el-dropdown-item>
            <el-dropdown-item divided>Action 5</el-dropdown-item>
          </el-dropdown-menu>
        </template>
      </el-dropdown>
    </div>
  </div>

</template>

<style scoped>

</style>
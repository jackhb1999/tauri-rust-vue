<script setup lang="ts">
import Header from './components/header.vue'
import Menu from './components/menu.vue'
import TagList from './components/tagList.vue'


</script>

<template>
  <el-container>
    <el-header><Header/></el-header>
    <el-container>
      <el-aside><Menu/></el-aside>
      <el-main>
        <TagList/>
        <router-view></router-view>
      </el-main>
    </el-container>
  </el-container>
</template>

<style scoped>

</style>
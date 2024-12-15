<script setup lang="ts">
import Header from './components/header.vue'
import Menu from './components/menu.vue'
import TagList from './components/tagList.vue'
import {useAsideWidthStore} from "@/store/asideWidth.ts";
const asideWidthStore = useAsideWidthStore()


</script>

<template>
  <el-container>
    <el-header><Header/></el-header>
    <el-container>
      <el-aside :width="asideWidthStore.asideWidth"><Menu/></el-aside>
      <el-main>
        <TagList/>
        <router-view></router-view>
      </el-main>
    </el-container>
  </el-container>
</template>

<style scoped>
.el-aside {
  transform: all 0.2s;
}
</style>
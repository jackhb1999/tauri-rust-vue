<script setup lang="ts">
import Header from './components/header.vue'
import Menu from './components/menu.vue'
import TagList from './components/tagList.vue'
import {useAsideWidthStore} from "@/store/asideWidth.ts";

const asideWidthStore = useAsideWidthStore()


</script>

<template>
  <el-container>
    <el-header>
      <Header/>
    </el-header>
    <el-container>
      <el-aside :width="asideWidthStore.asideWidth">
        <Menu/>
      </el-aside>
      <el-main>
        <TagList/>
        <router-view v-slot="{Component}">
          <transition name="fade">
            <keep-alive :max="7">
              <component :is="Component"/>
            </keep-alive>
          </transition>
        </router-view>
      </el-main>
    </el-container>
  </el-container>
</template>

<style scoped>
.el-aside {
  transform: all 0.2s;
}
.fade-enter-from{
  opacity: 0;
}
.fade-enter-to{
  opacity: 1;
}
.fade-leave-from{
  opacity: 1;
}
.fade-leave-to{
  opacity: 0;
}
.fade-enter-active,.fade-leave-active{
  transition: all 0.2s;
}
.fade-enter-active{
  trasition-delay: 0.2s;
}
</style>
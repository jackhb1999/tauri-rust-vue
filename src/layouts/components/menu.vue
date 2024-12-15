<script setup lang="ts">

import {useRoute, useRouter} from "vue-router";
import {useAsideWidthStore} from "@/store/asideWidth.ts";
import {ref} from "vue";

const router = useRouter();
const asideWidthStore = useAsideWidthStore()
const route = useRoute()

const asideMenus = [{
  "name": "后台面板",
  "icon": "help",
  "child": [
    {
      "name": "主控台",
      "icon": "home-filled",
      "frontpath": "/",
    }
  ]
}, {
  "name": "商城管理",
  "icon": "shopping-bag",
  "child": [
    {
      "name": "商品管理",
      "icon": "home-filled",
      "frontpath": "/goods/list",
    }
  ]
}]

const handleSelect = (path: string) => router.push(path)

// 默认选中
const defaultActive = ref(route.path)
</script>

<template>
  <div class="menu" :style="{width:asideWidthStore.asideWidth}">
    <el-menu unique-opened
        :default-active="defaultActive"
        class="border-0"
        @select="handleSelect"
        :collapse="asideWidthStore.asideFold"
        :collapse-transition="false"
    >
      <template v-for="(item,index) in asideMenus" :key="index">
        <el-sub-menu v-if="item.child && item.child.length > 0"
                     :index="index">
          <template #title>
            <el-icon>
              <component :is="item.icon"/>
            </el-icon>
            <span>{{ item.name }}</span>
          </template>

          <el-menu-item
              v-for="(ite,ind) in item.child" :key="ind"
              :index="ite.frontpath">
            <template #title>
              <el-icon>
                <component :is="ite.icon"/>
              </el-icon>
              <span>{{ ite.name }}</span>
            </template>
          </el-menu-item>
        </el-sub-menu>
        <el-menu-item v-else :index="index">
          <el-icon>
            <component :is="item.icon"/>
          </el-icon>
          <template #title>{{ item.name }}</template>
        </el-menu-item>
      </template>
    </el-menu>
  </div>
</template>

<style scoped>
.menu {
  transform: all 0.2s;
  width: 250px;
  top: 7vh;
  bottom: 0;
  left: 0;
  overflow-y: auto;
  overflow-x: hidden;
  @apply shadow-md fixed bg-light-50;
}
.menu::-webkit-scrollbar {
  width: 1px;
}
</style>
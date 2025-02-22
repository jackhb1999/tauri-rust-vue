<script setup lang="ts">

import {useRoute, useRouter} from "vue-router";
import {useAsideWidthStore} from "@/store/asideWidth.ts";
import {ref} from "vue";
import MenuItem from "@/layouts/components/menuItem/index.vue";

const router = useRouter();
const asideWidthStore = useAsideWidthStore()
const route = useRoute()

const asideMenus = router.getRoutes().find(item => item.path === '/')?.children
// .find(item => item.path === '/app')?.children

const handleSelect = (path: string) => router.push(path)

// 默认选中
const defaultActive = ref(route.path)


</script>

<script lang="ts">
export default {
  name: "Menu",
}
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
      <MenuItem :menuList="asideMenus"/>
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
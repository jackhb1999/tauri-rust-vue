<script setup lang="ts">

import {useRoute, useRouter} from "vue-router";
import {useAsideWidthStore} from "@/store/asideWidth.ts";
import {ref} from "vue";
import MenuItem from "@/layouts/components/menu/index.vue";

const router = useRouter();
const asideWidthStore = useAsideWidthStore()
const route = useRoute()

const asideMenus = router.getRoutes()
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
      <!--      <template v-for="(item,index) in asideMenus" :key="index">-->

      <!--        <el-sub-menu v-if="item.children && item.children.length > 0"-->
      <!--                     :index="index">-->
      <!--          <template #title>-->
      <!--            <el-icon>-->
      <!--              <component :is="item.meta.icon"/>-->
      <!--            </el-icon>-->
      <!--            <span>{{ item.meta.title }}</span>-->
      <!--          </template>-->
      <!--          <MenuItem :menuList="item.children"/>-->
      <!--          &lt;!&ndash;          <el-menu-item&ndash;&gt;-->
      <!--          &lt;!&ndash;              v-for="(ite,ind) in item.children" :key="ind"&ndash;&gt;-->
      <!--          &lt;!&ndash;              :index="ite.path">&ndash;&gt;-->
      <!--          &lt;!&ndash;            <template #title>&ndash;&gt;-->
      <!--          &lt;!&ndash;              <el-icon>&ndash;&gt;-->
      <!--          &lt;!&ndash;                <component :is="ite.meta.icon"/>&ndash;&gt;-->
      <!--          &lt;!&ndash;              </el-icon>&ndash;&gt;-->
      <!--          &lt;!&ndash;              <span>{{ ite.meta.title }}</span>&ndash;&gt;-->
      <!--          &lt;!&ndash;            </template>&ndash;&gt;-->
      <!--          &lt;!&ndash;          </el-menu-item>&ndash;&gt;-->
      <!--        </el-sub-menu>-->
      <!--        <el-menu-item v-else :index="item.path">-->
      <!--          <el-icon>-->
      <!--            <component :is="item.meta.icon"/>-->
      <!--          </el-icon>-->
      <!--          <template #title>{{ item.meta.title }}</template>-->
      <!--        </el-menu-item>-->
      <!--      </template>-->
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
<script lang="ts">
export default {
  name: "MenuItem",
  props: {
    menuList: {
      type: Array<any>,
      default: [],
    },
  },
}
</script>

<template>
  <template v-for="(item,index) in menuList" :key="index">
    <!--        有子路由-->
    <el-sub-menu v-if="item.children && item.children.length > 0"
                 :index="item.path">
      <template #title>
        <el-icon>
          <component :is="item.meta.icon"/>
        </el-icon>
        <span>{{ item.meta.title }}</span>
      </template>
      <MenuItem :menuList="item.children"/>
    </el-sub-menu>
    <!--        无子路由-->
    <el-menu-item v-else :index="item.path">
      <el-icon>
        <component :is="item.meta.icon"/>
      </el-icon>
      <template #title>{{ item.meta.title }}</template>
    </el-menu-item>
  </template>
</template>

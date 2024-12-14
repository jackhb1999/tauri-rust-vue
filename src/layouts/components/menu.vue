<script setup lang="ts">
const asideMenus=[{}]
</script>

<template>
  <div class="menu">
    <el-menu
        default-active="2"
        class="border-0"
    >
      <el-sub-menu index="1">
        <template #title>
          <el-icon>
            <location/>
          </el-icon>
          <span>Navigator One</span>
        </template>
        <el-menu-item-group>
          <template #title><span>Group One</span></template>
          <el-menu-item index="1-1">item one</el-menu-item>
          <el-menu-item index="1-2">item two</el-menu-item>
        </el-menu-item-group>
        <el-menu-item-group title="Group Two">
          <el-menu-item index="1-3">item three</el-menu-item>
        </el-menu-item-group>
        <el-sub-menu index="1-4">
          <template #title><span>item four</span></template>
          <el-menu-item index="1-4-1">item one</el-menu-item>
        </el-sub-menu>
      </el-sub-menu>
      <el-menu-item index="2">
        <el-icon>
          <icon-menu/>
        </el-icon>
        <template #title>Navigator Two</template>
      </el-menu-item>
      <el-menu-item index="3" disabled>
        <el-icon>
          <document/>
        </el-icon>
        <template #title>Navigator Three</template>
      </el-menu-item>
      <el-menu-item index="4">
        <el-icon>
          <setting/>
        </el-icon>
        <template #title>Navigator Four</template>
      </el-menu-item>
    </el-menu>
  </div>
</template>

<style scoped>
.menu{
  width: 250px;
  top: 7vh;
  bottom: 0;
  left: 0;
  overflow: auto;
  @apply shadow-md fixed bg-light-50;
}
</style>
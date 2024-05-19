<script lang="ts">
import {defineComponent} from 'vue'

export default defineComponent({
  name: "HomeView"
})
</script>

<template>
<div class="home">
  <el-container>
    <el-header height="28px">
      <el-row :gutter="20">
        <el-col :span="4">
          <img  class="logo" src="../assets/vue.svg" />
        </el-col>
        <el-col :span="16">
          <h2>后台管理系统</h2>
        </el-col>
        <el-col :span="4">
          <el-button class="quit_login">退出登陆</el-button>
        </el-col>
      </el-row>
    </el-header>
    <el-container>
      <el-aside width="125px">
        <el-menu
            default-active="2"
            class="el-menu-vertical-demo"
        >
          <el-menu-item index="1">
            <el-icon><icon-menu /></el-icon>
            <span>商品列表</span>
          </el-menu-item>
          <el-menu-item index="3" disabled>
            <el-icon><document /></el-icon>
            <span>Navigator</span>
          </el-menu-item>
          <el-menu-item index="4">
            <el-icon><setting /></el-icon>
            <span>Navigator</span>
          </el-menu-item>
        </el-menu>
      </el-aside>
      <el-main>Main</el-main>
    </el-container>
  </el-container>
</div>
</template>

<style lang="scss" scoped>
.logo{
height: 28px;
}
h2,quit_login{
  text-align: center;
  height: 28px;
  line-height: 28px;
}
.el-aside{
  .el-menu{
    height: calc(100vh - 28px);
  }
}
</style>
<script lang="ts">
import {defineComponent} from 'vue'
import {useRouter} from "vue-router";

export default defineComponent({
  name: "HomeView",
  setup() {
    const router = useRouter()
    console.log(9, router.getRoutes())
    const list = router.getRoutes().filter(v => v.meta.isShow)
    console.log(11,list)
    return {list}
  },
})
</script>

<template>
  <div class="home">
    <el-container>
      <el-header height="28px">
        <el-row :gutter="20">
          <el-col :span="4">
            <img class="logo" src="../assets/vue.svg"/>
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
              class="el-menu-vertical-demo"
              router
          >
            <el-menu-item v-for="item in list" :index="item.path" :key="item.path">
              <span>{{item.meta.title}}</span>
            </el-menu-item>

          </el-menu>
        </el-aside>
        <el-main>
          <router-view/>
        </el-main>
      </el-container>
    </el-container>
  </div>
</template>

<style lang="scss" scoped>
.logo {
  height: 28px;
}

h2, quit_login {
  text-align: center;
  height: 28px;
  line-height: 28px;
}

.el-aside {
  .el-menu {
    height: calc(100vh - 28px);
  }
}
</style>
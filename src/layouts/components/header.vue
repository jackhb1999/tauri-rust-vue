<script setup lang="ts">
import {confirm, toastByError, toastByFail, toastBySuccess} from "@/composables/util.ts"
import {useRouter} from "vue-router";
import {useUserInfoStore} from "@/store/userinfo.ts";
import {useFullscreen} from '@vueuse/core'
import {Crop, FullScreen, Lock, User} from "@element-plus/icons-vue";
import {reactive, ref, useTemplateRef} from "vue";

import type {FormInstance} from "element-plus";
import {login, updatePassword} from "@/api/tpi.ts";

import FormDrawer from '@/components/FormDrawer.vue'

const {
  isFullscreen, // 是否全屏状态
  toggle
} = useFullscreen()

const store = useUserInfoStore()
const router = useRouter()

function logout() {
  confirm("是否退出登录").then(() => {
    // 确定
    // logout().finally(()=>{
    // 清除当前的用户状态
    store.removeInfo()

    // 跳转回登录页
    toastBySuccess("退出登录成功！")
    router.push("/login")

    // })
  }).catch(() => {
    // 取消


  })
}

// 修改密码drawer
const formDrawer = useTemplateRef('formDrawerRef')

const commandHandle = (comm: String) => {
  switch (comm) {
    case 'logout':
      logout()
      break;
    case 'rePassword':
      // 修改密码操作
      formDrawer.value?.open()
      break;
    default:
      break;
  }
}

const pws = reactive({
  oldpassword: '',
  newpassword: '',
  newpasswordagen: '',
})

const rules = {
  oldpassword: [
    {required: true, message: '旧密码不能为空！', trigger: 'blur'},
  ],
  newpassword: [{required: true, message: '新密码不能为空！', trigger: 'blur'},
  ],
  newpasswordagen: [{required: true, message: '确认密码不能为空！', trigger: 'blur'},
  ],
}

const formRef = ref<FormInstance>()



const onSubmit = () => {
  formRef.value?.validate((isValid) => {
    if (isValid) {
     formDrawer.value?.showLoading()
      updatePassword(pws).then(res => {
        toastBySuccess('修改密码成功，请重新登录', 1500)

        // 跳转页面
        store.removeInfo()
        router.push("/login")
      }).catch(err => {
        toastByFail('网络连接错误')
      }).finally(() => {
      formDrawer.value?.hideLoading()
      })
    } else {
      toastByError('输入有误，请检查', 1600)
    }
  })
  console.log('submit!')
}

const refreshHandle = () => {
  location.reload()
}
</script>

<template>
  <div class="my-header">
  <span class="logo">
    <el-icon class="mr-1"><Compass/></el-icon>
    Are
  </span>
    <el-tooltip
        effect="dark"
        content="菜单折叠"
        placement="bottom"
    >
      <el-icon class="icon-btn">
        <Fold/>
      </el-icon>
    </el-tooltip>
    <el-tooltip
        effect="dark"
        content="刷新"
        placement="bottom"
    >
      <el-icon class="icon-btn" @click="refreshHandle">
        <Refresh/>
      </el-icon>
    </el-tooltip>
    <div class="ml-auto flex items-center">
      <el-tooltip v-if="false"
                  effect="dark"
                  content="全屏"
                  placement="bottom"
      >
        <el-icon class="icon-btn mr-1" @click="toggle">
          <FullScreen v-show="!isFullscreen"/>
          <Crop v-show="isFullscreen"/>
        </el-icon>
      </el-tooltip>
      <el-dropdown class="dropdown" @command="commandHandle">
    <span class="flex items-center text-light-50">
        <el-avatar class="mr-2" :size="25" src="https://cube.elemecdn.com/3/7c/3ea6beec64369c2642b92c6726f1epng.png"/>
      未知用户名
      <el-icon class="el-icon--right">
        <arrow-down/>
      </el-icon>
    </span>
        <template #dropdown>
          <el-dropdown-menu>
            <el-dropdown-item command="rePassword">修改密码</el-dropdown-item>
            <el-dropdown-item command="logout">退出登录</el-dropdown-item>
          </el-dropdown-menu>
        </template>
      </el-dropdown>
    </div>
  </div>

  <!--  <el-drawer v-model="drawer" title="修改密码" size="45%" :close-on-click-modal="false">-->

  <!--  </el-drawer>-->
  <form-drawer ref="formDrawerRef" title="修改密码" destroy-on-close @submit="onSubmit">
        <el-form ref="formRef" :model="pws" :rules="rules" class="w-[250px]"
               label-width="80px" size="small">
          <el-form-item prop="oldpassword" label="旧密码">
            <el-input v-model="pws.oldpassword" placeholder="请输入旧密码">

            </el-input>
          </el-form-item>
          <el-form-item prop="newpassword" label="新密码">
            <el-input v-model="pws.newpassword" placeholder="请输入新密码" type="password" show-password> />

            </el-input>
          </el-form-item>
          <el-form-item prop="newpasswordagen" label="确认密码">
            <el-input v-model="pws.newpasswordagen" placeholder="请输入确认密码" type="password" show-password> />
            </el-input>
          </el-form-item>
        </el-form>
  </form-drawer>
</template>

<style scoped>
.my-header {
  @apply flex items-center bg-indigo-500 text-light-50 fixed top-0 left-0 right-0 max-h-12;
  height: 7vh;
}

.logo {
  width: 120px;
  @apply flex justify-center items-center text-xl font-thin;
}

.icon-btn {
  @apply flex justify-center items-center max-h-11;
  width: 42px;
  height: 6.5vh;
  cursor: pointer;
}

.icon-btn:hover {
  @apply bg-indigo-400;
}

.my-header .dropdown {
  height: 6.5vh;
  cursor: pointer;
  @apply flex justify-center items-center mx-5 max-h-11;
}
</style>
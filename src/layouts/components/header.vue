<script setup lang="ts">
import {confirm, toastByError, toastByFail, toastBySuccess} from "@/composables/util.ts"
import {useRouter} from "vue-router";
import {useUserInfoStore} from "@/store/userinfo.ts";
import {useFullscreen} from '@vueuse/core'
import {Crop, FullScreen, Lock, User} from "@element-plus/icons-vue";
import {reactive, ref} from "vue";
import {LoginData} from "@/type/login.ts";
import type {FormInstance} from "element-plus";
import {login} from "@/api/tpi.ts";
import {setToken} from "@/composables/auth.ts";

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
const drawer = ref(false)

const commandHandle = (comm: String) => {
  switch (comm) {
    case 'logout':
      logout()
      break;
    case 'rePassword':
      // 修改密码操作
      drawer.value = true
      break;
    default:
      break;
  }
}

const pws = reactive({
  oldpassword:'',
  newpassword:'',
  newpasswordagen:'',
})

const rules = {
  username: [
    {required: true, message: '用户名不能为空！', trigger: 'blur'},
    {min: 3, max: 5, message: '用户名必须为3-5个字符', trigger: 'blur'},
  ],
  password: [{required: true, message: '请输入您的密码！', trigger: 'blur'},
    {min: 3, max: 5, message: '密码长度为 3 到 5 个字符！', trigger: 'blur'},],
}

const formRef = ref<FormInstance>()

const loading = ref(false)

const onSubmit = () => {
  formRef.value?.validate((isValid) => {
    if (isValid) {
      loading.value = true
      login(pws).then(res => {
        toastBySuccess('修改成功！', 1500)

         // 跳转页面
        router.push('/')
      }).catch(err => {
        toastByFail('网络连接错误')
      }).finally(() => {
        loading.value = false
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

  <el-drawer v-model="drawer" title="修改密码" size="45%" :close-on-click-modal="false">
    <el-form ref="formRef" :model="pws" :rules="rules" class="w-[250px]"
             @keyup.enter.native="onSubmit">
      <el-form-item prop="username">
        <el-input v-model="pws.oldpassword" placeholder="请输入用户名">
          <template #prefix>
            <el-icon>
              <User/>
            </el-icon>
          </template>
        </el-input>
      </el-form-item>
      <el-form-item prop="password">
        <el-input v-model="pws.newpassword" placeholder="请输入密码" type="password" show-password> />
          <template #prefix>
            <el-icon>
              <Lock/>
            </el-icon>
          </template>
        </el-input>
      </el-form-item>
      <el-form-item prop="password">
        <el-input v-model="pws.newpasswordagen" placeholder="请输入密码" type="password" show-password> />
          <template #prefix>
            <el-icon>
              <Lock/>
            </el-icon>
          </template>
        </el-input>
      </el-form-item>
      <el-form-item>
        <el-button class="w-[250px] transition ring-indigo-500" round type="primary"
                   @click="onSubmit" :loading="loading">登 录
        </el-button>
      </el-form-item>
    </el-form>
  </el-drawer>
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
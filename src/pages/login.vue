<script setup lang="ts">
import {reactive, ref} from 'vue'
import {User, Lock} from '@element-plus/icons-vue'
import type {FormInstance} from "element-plus";
import {login} from "@/api/tpi.ts";
import {LoginData} from "@/type/login.ts";



import {setToken} from "@/composables/auth.ts";
import {toastByError, toastByFail, toastBySuccess} from "@/composables/util.ts";
import {useRouter} from "vue-router";
// import {useStore} from "vuex";

// import router from "@/router";
const router = useRouter()
// const store = useStore()

const form = reactive(new LoginData())

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
      login(form).then(res => {
        toastBySuccess('登录成功！', 1500)

        // localStorage.setItem('token', res)
        setToken(res)

        // 获取用户信息
        // getInfo().then(resp=>{
        //   store.commit('SET_USERINFO', resp)
        // })

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
</script>

<template>
  <el-row class="login-container">
    <el-col :lg="16" :md="14" :sm="12" class="left">
      <div>
        <div>欢迎使用</div>
        <div>用过去的知识解决将来的问题</div>
      </div>
    </el-col>
    <el-col :lg="8" :md="10" :sm="12" class=" right ">
      <h2 class="font-bold text-3xl text-gray-800">登录系统</h2>
      <div class="flex items-center justify-center my-5 text-gray-300 space-x-2">
        <span class="h-[1px] w-16 bg-gray-200"></span>
        <span>账号密码登录</span>
        <span class="h-[1px] w-16 bg-gray-200"></span>
      </div>
      <el-form ref="formRef" :model="form" :rules="rules" class="w-[250px]"
      @keyup.enter.native="onSubmit">
        <el-form-item prop="username">
          <el-input v-model="form.username" placeholder="请输入用户名">
            <template #prefix>
              <el-icon>
                <User/>
              </el-icon>
            </template>
          </el-input>
        </el-form-item>
        <el-form-item prop="password">
          <el-input v-model="form.password" placeholder="请输入密码" type="password" show-password> />
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
    </el-col>
  </el-row>
</template>

<style scoped>

.login-container {
  @apply min-h-screen bg-indigo-500;
}

.login-container .left, .login-container .right {
  @apply flex items-center justify-center;
}

.login-container .right {
  @apply bg-light-50 flex-col;
}

.left > div > div:first-child {
  @apply font-bold text-5xl text-light-50 mb-4;
}

.left > div > div:last-child {
  @apply text-gray-200 text-lg;
}
</style>
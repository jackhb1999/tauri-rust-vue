<template>
  <div class="login-box">

    <el-form
        ref="ruleFormRef"
        :model="ruleForm"
        status-icon
        :rules="rules"
        label-width="80px"
        class="login-ruleForm"
    >
      <h2>后台管理系统</h2>
      <el-form-item label="账号：" prop="username">
        <el-input v-model="ruleForm.username" autocomplete="off"/>
      </el-form-item>

      <el-form-item label="密码：" prop="password">
        <el-input v-model="ruleForm.password" type="password" autocomplete="off"/>
      </el-form-item>
      <el-form-item>

        <el-button class="login_btn" type="primary" @click="submitForm(ruleFormRef)">
          提交
        </el-button>
        <el-button class="login_btn" @click="resetForm(ruleFormRef)">重置</el-button>
      </el-form-item>
    </el-form>
  </div>
</template>

<script lang="ts">
import {defineComponent, reactive, toRefs, ref} from 'vue'
import {LoginData} from "../type/login.ts";
import type {FormInstance, FormRules} from 'element-plus'
import {login} from '../api/tpi.ts'
import {useRouter} from "vue-router";


export default defineComponent({
  setup() {
    const data = reactive(new LoginData())
    let rules: {
      username: [
        { required: true, message: '请输入您的账号！', trigger: 'blur' },
        { min: 3, max: 5, message: '账号长度为 3 到 5 个字符！', trigger: 'blur' },
      ],
      password: [
        { required: true, message: '请输入您的密码！', trigger: 'blur' },
        { min: 3, max: 5, message: '密码长度为 3 到 5 个字符！', trigger: 'blur' },
      ],
    }
    // 登陆
    const ruleFormRef = ref<FormInstance>()
    const router = useRouter()
    const submitForm = (formEl: FormInstance | undefined) => {
      if (!formEl) return
      formEl.validate((valid) => {
        if (valid) {
          console.log(57)
          login(data.ruleForm).then(res => {
            console.log(58, res)
            localStorage.setItem('token', res)
            // 跳转页面
            router.push('/home')
          })
        } else {
          console.log('error submit!')
        }
      })
    }

    // 重置
    const resetForm = () => {
      data.ruleForm.username = ""
      data.ruleForm.password = ""
    }
    return {...toRefs(data), rules, resetForm, ruleFormRef, submitForm}
  },
})
</script>


<style lang="scss" scoped>
.login-box {
  width: 100%;
  height: 100%;
  background: url("../assets/bg.jpg") no-repeat center;
  background-size: cover;
  overflow: hidden;
  text-align: center;

  .login-ruleForm {
    width: 500px;
    margin: 200px auto;
    padding: 40px;
    border-radius: 20px;
  }

  .login_btn {
    margin: auto;
    width: 38%
  }

  h2 {
    margin-bottom: 10px;
  }
}
</style>
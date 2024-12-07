<script setup lang="ts">
import {confirm, toastBySuccess} from "@/composables/util.ts"
import {useRouter} from "vue-router";
import {useUserInfoStore} from "@/store/userinfo.ts";
import Admin from '@/layouts/admin.vue'

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
</script>

<template>
  首页内容


</template>

<style scoped>

</style>
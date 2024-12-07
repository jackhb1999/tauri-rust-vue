import {defineStore} from "pinia";
import {reactive, ReactiveEffect} from "vue";
import {removeToken} from "../composables/auth.ts";

export const useUserInfoStore = defineStore('userinfo', ()=>{
    const userInfo = reactive({})

    function getInfo(){
        return new Promise((resolve, reject)=>{
            // getUserInfo().then(res => {
            //
            // })
        })
    }

    function removeInfo(){
        // 移除 cookie 中的 token
        removeToken()
        // 清除当前用户状态

    }

    return {userInfo,getInfo,removeInfo}
})
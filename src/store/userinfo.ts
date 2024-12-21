import {defineStore} from "pinia";
import {reactive, ReactiveEffect} from "vue";
import {removeToken} from "../composables/auth.ts";
import {addRoutes} from "@/router";

export const useUserInfoStore = defineStore('userinfo', () => {
    let userInfo = reactive({
        user_code: '',
        username: '',
        depts: [],
        roles: [],
        menus: []
    })

    function getInfo() {
        return new Promise((resolve, reject) => {
            // getUserInfo().then(res => {
            //
            // })
        })
    }

    function setInfo(info: any) {
        userInfo.user_code = info.user_code
        userInfo.username = info.username
        userInfo.depts = info.depts
        userInfo.roles = info.roles
        userInfo.menus = info.menus
        addRoutes(userInfo.menus)
    }

    function removeInfo() {
        // 移除 cookie 中的 token
        removeToken()
        // 清除当前用户状态

    }

    return {userInfo, getInfo, removeInfo, setInfo}
})
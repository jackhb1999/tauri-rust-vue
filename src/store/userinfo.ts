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

  // 从本地存储中获取数据
    const loadUserInfoFromLocalStorage = () => {
        let storedUserInfo = localStorage.getItem('userInfo');
        if (storedUserInfo) {
            storedUserInfo = JSON.parse(storedUserInfo);
            userInfo.user_code = storedUserInfo.user_code;
            userInfo.username = storedUserInfo.username;
            userInfo.depts = storedUserInfo.depts;
            userInfo.roles = storedUserInfo.roles;
            userInfo.menus = storedUserInfo.menus;
        }
    };

    // 将数据保存到本地存储
    const saveUserInfoToLocalStorage = () => {
        localStorage.setItem('userInfo', JSON.stringify(userInfo));
    };

    // 在 store 初始化时加载数据
    loadUserInfoFromLocalStorage();
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
        // 保存数据到本地存储
        saveUserInfoToLocalStorage();
    }

    function removeInfo() {
        // 移除 cookie 中的 token
        removeToken()
        // 清除当前用户状态
        localStorage.removeItem('userInfo')

    }

    return {userInfo, getInfo, removeInfo, setInfo}
})
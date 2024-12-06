import router from "./index.ts";
import {getToken} from "../composables/auth.ts";
import {toastByError, toastByFail} from "../composables/util.ts";
import store from "../store";
// 全局前置守卫
router.beforeEach(async (to, from, next) => {
    const token = getToken()
    // 没有登录强制跳登录页
    if (!token && to.path != "/login") {
        toastByFail("请先登录！")
        return next({path: "/login"})
    }
    // 防止重复登录
    if (token && to.path != "/login") {
        toastByError("无需重复登录", 1500)
        return next({path: from.path ? from.path : '/'})
    }
    // 如果用户登录了，则自动获取用户信息，并保存在vuex中
    if(token){
        await store.dispatch('getinfo')
    }


    next()
})
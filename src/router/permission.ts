import {addRoutes, router} from "./index.ts";
import {getToken} from "../composables/auth.ts";
import {hideFullLoading, showFullLoading, toastByFail} from "../composables/util.ts";
import {useUserInfoStore} from "../store/userinfo.ts";
// 全局前置守卫
router.beforeEach(async (to, from, next) => {
    showFullLoading()
    const token = getToken()
    // console.log('从哪来', from)
    // console.log('到哪去', to)
    // 没有登录强制跳登录页
    if (!token && to.path != "/login") {
        if (from.path != "/") {
            toastByFail("请先登录！")
        }
        return next({path: "/login"})
    }
    // 防止重复登录
    // if (token && to.path != "/login") {
    //     toastByError("无需重复登录", 1500)
    //     return next({path: from.path && from.path != "/login" ? from.path : '/'})
    // }
    // 如果用户登录了，则自动获取用户信息，并保存在vuex中
    let hasNewRoutes = false
    if (token) {
        const store = useUserInfoStore()
        hasNewRoutes =  addRoutes( store.userInfo.menus)
        // hasNewRoutes = true
        // await store.getInfo()
    }

    // 设置页面标题
    let title = (to.meta.title ? to.meta.title + "-" : "") + "rustob.com"
    document.title = title

    return hasNewRoutes? next(to.fullPath):next()
})


// 全局后置守卫
router.afterEach((to, from, next) => {
    hideFullLoading()
})
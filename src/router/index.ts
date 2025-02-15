import {createRouter, createWebHistory, RouteRecordRaw} from 'vue-router'


import GoodsList from '@/pages/goods/list.vue'
import CategoryList from '@/pages/category/list.vue'
import {routes} from "@/router/routes.ts";


// 动态路由，用于匹配菜单动态添加路由
const asyncRoutes: Array<RouteRecordRaw> = [
    {
        path: '/goods/list',
        name: '/goods/list',
        component: GoodsList,
        meta: {
            title: "商品管理",
            icon: "home-filled"
        }
    },
    {
        path: '/category/list',
        name: '/category/list',
        component: CategoryList,
        meta: {
            title: "分类管理",
            icon: "home-filled"
        }
    },
]

export const router = createRouter({
    // 配置路由模式
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: routes
})

// 动态添加路由的方法
export function addRoutes(menus) {
    // 是否有新的路由
    let hasNewRouter = false
    // const findAndRoutes = (arr: Array) => {
    //     arr.forEach(item => {
    //         let ite = asyncRoutes.find(it => it.path == item.menu_path)
    //         if (ite && !router.hasRoute(ite.path)) {
    //             console.log(175,ite,router.getRoutes())
    //             router.addRoute("admin", ite)
    //             // router.getRoutes().find(it=>it.path=='/app')?.children.push(ite)
    //             hasNewRouter = true
    //         }
    //         if (item.child && item.child.length > 0) {
    //             findAndRoutes(item.child)
    //         }
    //     })
    // }
    // findAndRoutes(menus)
    router.addRoute("admin", {
        path: '/goods/list',
        name: '/goods/list',
        component: GoodsList,
        meta: {
            title: "商品管理",
            icon: "home-filled"
        }
    })
    router.addRoute("admin", asyncRoutes[1])
    console.log(184, router.getRoutes())
    return hasNewRouter
}



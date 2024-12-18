import {createRouter, createWebHistory, RouteRecordRaw} from 'vue-router'


import LoginView from '@/pages/login.vue'
import HomeView from '@/pages/index.vue'
import Admin from "@/layouts/admin.vue";
import GoodsList from '@/pages/goods/list.vue'
import CategoryList from '@/pages/category/list.vue'

// const routes: Array<RouteRecordRaw> = [
//     {
//         path: '/',
//         component: Admin,
//         // 子路由
//         children: [
//             {
//                 path: '/',
//                 name: 'home',
//                 component: HomeView,
//                 meta: {
//                     title: "后台首页"
//                 }
//             },
//             {
//                 path: '/goods/list',
//                 name: 'home',
//                 component: GoodsList,
//                 meta: {
//                     title: "商品管理"
//                 }
//             },
//             {
//                 path: '/goods/list',
//                 name: 'home',
//                 component: CategoryList,
//                 meta: {
//                     title: "分类管理"
//                 }
//             },
//         ]
//     },
//     {
//         path: '/login',
//         name: 'login',
//         component: LoginView,
//         meta: {
//             title: "登录页"
//         }
//     },
//
//     // {
//     //     path: '/',
//     //     name: 'home',
//     //     component: HomeView,
//     //     children: [
//     //         {
//     //             path: "goods",
//     //             name: "goods",
//     //             meta: {
//     //                 isShow: true,
//     //                 title:"商品列表"
//     //             },
//     //             component: () => import("@/views/GoodsView.vue")
//     //         },
//     //         {
//     //             path: "user",
//     //             name: "user",
//     //             meta: {
//     //                 isShow: true,
//     //                 title:"用户列表"
//     //             },
//     //             component: () => import("@/views/UserView.vue")
//     //         }
//     //         ,
//     //         {
//     //             path: "index",
//     //             name: "index",
//     //             meta: {
//     //                 isShow: true,
//     //                 title:"🧭导航"
//     //             },
//     //             component: () => import("../views/Index.vue")
//     //         },
//     //         {
//     //             path: '/:pathMatch(.*)*',
//     //             name: 'NotFound',
//     //             component: () => import('@/pages/404.vue')
//     //         },
//     //     ]
//     // },
//     {
//         path: '/about',
//         name: 'about',
//         // route level code-splitting
//         // this generates a separate chunk (About.[hash].js) for this route
//         // which is lazy-loaded when the route is visited.
//         component: () => import('../views/AboutView.vue')
//     },
//     {
//         path: '/:pathMatch(.*)*',
//         name: 'NotFound',
//         component: () => import('@/pages/404.vue')
//     },
//
// ]

// 默认路由，所有用户共享
const routes: Array<RouteRecordRaw> = [
    {
        path: '/',
        name: 'home',
        component: Admin,
    },
    {
        path: '/login',
        name: 'login',
        component: LoginView,
        meta: {
            title: "登录页"
        }
    },
    {
        path: '/:pathMatch(.*)*',
        name: 'NotFound',
        component: () => import('@/pages/404.vue')
    },
]

// 动态路由，用于匹配菜单动态添加路由
const asyncRoutes: Array<RouteRecordRaw> = [
    {
        path: '/',
        name: 'home',
        component: HomeView,
        meta: {
            title: "后台首页"
        }
    },
    {
        path: '/goods/list',
        name: '/goods/list',
        component: GoodsList,
        meta: {
            title: "商品管理"
        }
    },
    {
        path: '/category/list',
        name: '/category/list',
        component: CategoryList,
        meta: {
            title: "分类管理"
        }
    },
]

export const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: routes
})

// 动态添加路由的方法
export function addRoutes(menus) {
    const findAndRoutes = (arr: Array) => {
        arr.forEach(item => {
            let ite = asyncRoutes.find(it => it.path == item.frontpath)
            if (ite && !router.hasRoute(ite.path)) {
                router.addRoute("home", ite)
            }
            if (item.child && item.child.length > 0) {
                findAndRoutes(item.child)
            }
        })
    }
    findAndRoutes(menus)
}



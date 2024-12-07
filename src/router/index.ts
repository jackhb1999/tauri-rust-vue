import {createRouter, createWebHistory, RouteRecordRaw} from 'vue-router'


import LoginView from '@/pages/login.vue'
import HomeView from '@/pages/index.vue'
import Admin from "@/layouts/admin.vue";

const routes: Array<RouteRecordRaw> = [
    {
        path: '/',
        component:Admin,
        // 子路由
        children:[
            {
                path: '/',
                name: 'home',
                component:HomeView,
                meta:{
                    title:"后台首页"
                }
            },
        ]
    },
    {
        path: '/login',
        name: 'login',
        component:LoginView,
        meta:{
            title: "登录页"
        }
    },

    // {
    //     path: '/',
    //     name: 'home',
    //     component: HomeView,
    //     children: [
    //         {
    //             path: "goods",
    //             name: "goods",
    //             meta: {
    //                 isShow: true,
    //                 title:"商品列表"
    //             },
    //             component: () => import("@/views/GoodsView.vue")
    //         },
    //         {
    //             path: "user",
    //             name: "user",
    //             meta: {
    //                 isShow: true,
    //                 title:"用户列表"
    //             },
    //             component: () => import("@/views/UserView.vue")
    //         }
    //         ,
    //         {
    //             path: "index",
    //             name: "index",
    //             meta: {
    //                 isShow: true,
    //                 title:"🧭导航"
    //             },
    //             component: () => import("../views/Index.vue")
    //         },
    //         {
    //             path: '/:pathMatch(.*)*',
    //             name: 'NotFound',
    //             component: () => import('@/pages/404.vue')
    //         },
    //     ]
    // },
    {
        path: '/about',
        name: 'about',
        // route level code-splitting
        // this generates a separate chunk (About.[hash].js) for this route
        // which is lazy-loaded when the route is visited.
        component: () => import('../views/AboutView.vue')
    },

]

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: routes
})

export default router

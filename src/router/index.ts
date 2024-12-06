import {createRouter, createWebHistory, RouteRecordRaw} from 'vue-router'
import HomeView from '@/views/HomeView.vue'

import LoginView from '@/pages/Login.vue'

const routes: Array<RouteRecordRaw> = [
    {
        path: '/login',
        name: 'login',
        component:LoginView
    },
    {
        path: '/',
        name: 'home',
        component: HomeView,
        children: [
            {
                path: "goods",
                name: "goods",
                meta: {
                    isShow: true,
                    title:"商品列表"
                },
                component: () => import("@/views/GoodsView.vue")
            },
            {
                path: "user",
                name: "user",
                meta: {
                    isShow: true,
                    title:"用户列表"
                },
                component: () => import("@/views/UserView.vue")
            }
            ,
            {
                path: "index",
                name: "index",
                meta: {
                    isShow: true,
                    title:"🧭导航"
                },
                component: () => import("../views/Index.vue")
            },
            {
                path: '/:pathMatch(.*)*',
                name: 'NotFound',
                component: () => import('@/pages/404.vue')
            },
        ]
    },
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

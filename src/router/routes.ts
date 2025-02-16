import {RouteRecordRaw} from 'vue-router'


import LoginView from '@/pages/login.vue'
import HomeView from '@/pages/index.vue'
import Admin from "@/layouts/admin.vue";
import GoodsList from "@/pages/goods/list.vue";
import CategoryList from "@/pages/category/list.vue";


// 默认路由，所有用户共享
export const routes: Array<RouteRecordRaw> = [
    {
        path: '/app',
        name: 'admin',
        component: Admin,
        meta: {
            title: "主控台",
        },
        children: [
            {
                path: '/',
                name: 'home',
                component: HomeView,
                meta: {
                    title: "后台首页",
                    icon: "home-filled",
                },

            },
        ]
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
        path: '/404',
        name: '404',
        component: () => import('@/pages/404.vue')
    },
    {
        path: '/:pathMatch(.*)*',
        name: 'NotFound',
        // 直接指向或重定向
        // component: () => import('@/pages/404.vue')
        redirect: '/404'
    },
]


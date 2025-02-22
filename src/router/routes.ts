import {RouteRecordRaw} from 'vue-router'


import LoginView from '@/pages/login.vue'
import HomeView from '@/pages/index.vue'
import Admin from "@/layouts/admin.vue";
import GoodsList from "@/pages/goods/list.vue";
import CategoryList from "@/pages/category/list.vue";


// 默认路由，所有用户共享
export const routes: Array<RouteRecordRaw> = [

    {
        path: '/',
        name: 'admin',
        component: Admin,
        meta: {
            title: "",
        },
        children: [
            {
                path: '/app',
                name: 'home',
                component: HomeView,
                meta: {
                    title: "后台首页",
                    icon: "home-filled",
                },
            },
            {
                path: 'goods',
                name: 'goods',
                meta: {
                    title: "商品管理",
                    icon: "home-filled"
                },
                children: [
                    {
                        path: '/goods/list',
                        name: '/goods/list',
                        component: GoodsList,
                        meta: {
                            title: "商品列表",
                            icon: "home-filled"
                        }
                    }
                ]
            },
            {
                path: '/category/list',
                name: '/category/list',
                component:
                CategoryList,
                meta:
                    {
                        title: "分类管理",
                        icon:
                            "home-filled"
                    }
            },
            {
                path: '/acl',
                component: Admin,
                meta: {
                    title: "权限管理",
                    icon: "Lock"
                },
                children: [
                    {
                        path: '/acl/user',
                        name: '/acl/user',
                        component: () => import('@/pages/acl/user/index.vue'),
                        meta: {
                            title: "用户管理",
                            icon: "User"
                        }
                    },
                    {
                        path: '/acl/role',
                        name: '/acl/role',
                        component: () => import('@/pages/acl/role/index.vue'),
                        meta: {
                            title: "角色管理",
                            icon: "UserFilled"
                        }
                    },
                    {
                        path: '/acl/permission',
                        name: '/acl/permission',
                        component: () => import('@/pages/acl/permission/index.vue'),
                        meta: {
                            title: "权限管理",
                            icon: "Monitor"
                        }
                    },
                ]
            },
            {
                path: '/product',
                component: Admin,
                meta: {
                    title: "商品管理",
                    icon: "Lock"
                },
                children: [
                    {
                        path: '/product/tradmark',
                        name: '/product/tradmark',
                        component: () => import('@/pages/product/tradmark/index.vue'),
                        meta: {
                            title: "品牌管理",
                            icon: "User"
                        }
                    },
                    {
                        path: '/product/attr',
                        name: '/product/attr',
                        component: () => import('@/pages/product/attr/index.vue'),
                        meta: {
                            title: "属性管理",
                            icon: "User"
                        }
                    },
                    {
                        path: '/product/spu',
                        name: '/product/spu',
                        component: () => import('@/pages/product/spu/index.vue'),
                        meta: {
                            title: "SPU管理",
                            icon: "User"
                        }
                    },
                    {
                        path: '/product/sku',
                        name: '/product/sku',
                        component: () => import('@/pages/product/sku/index.vue'),
                        meta: {
                            title: "SKU管理",
                            icon: "User"
                        }
                    },
                ]
            },
        ]
    },
    {
        path: '/login',
        name: 'login',
        component: LoginView,
        meta: {
            title: "登录页",
            hidden:
                true,
        }
    }
    ,
    {
        path: '/404',
        name: '404',
        component: () => import('@/pages/404.vue'),
        meta:
            {
                title: "404",
                hidden:
                    true,
            }
    },
    {
        path: '/:pathMatch(.*)*',
        name: 'NotFound',
        // 直接指向或重定向
        // component: () => import('@/pages/404.vue')
        redirect: '/404',
        meta:
            {
                hidden: true,
            }
    }
    ,
]


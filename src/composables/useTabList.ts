import {onMounted, ref, watch} from 'vue'
import {onBeforeRouteUpdate, useRoute, useRouter} from "vue-router";
import {useCookies} from "@vueuse/integrations/useCookies";

export function useTabList() {

    const route = useRoute()
    const router = useRouter()
    const cookie = useCookies()

    const activeTab = ref(route.path)
    const tabList = ref([
        {
            title: '后台首页',
            path: '/app'
        },
    ])

    onMounted(() => {
        if (cookie.get('tabList')) {
            tabList.value = cookie.get('tabList')
        }
    })

    watch(tabList, (newVal) => {
        cookie.set('tabList', newVal)
    }, {deep: true, immediate: false})

    onBeforeRouteUpdate((to, from) => {
        activeTab.value = to.path

        if (tabList.value.some(item => item.path === to.path)) {
            return
        }
        tabList.value.push({
            title: to.meta.title,
            path: to.path
        })
        cookie.set('tabList', tabList.value)

    })

    const changeTab = (path: string) => {
        console.log(44, path)
        activeTab.value = path
        router.push(path)
    }

    const removeTab = (path: string) => {
        let tabs = tabList.value
        let current = activeTab.value
        if (current === path) {
            tabs.forEach((tab, index) => {
                if (tab.path === path) {
                    let nextTab = tabs[index + 1] || tabs[index - 1]
                    if (nextTab) {
                        current = nextTab.path
                    }
                }
            })
        }
        activeTab.value = current
        changeTab(current)
        tabList.value = tabs.filter(item => item.path !== path)
        // cookie.set('tabList', tabList.value)
    }

    const closeHandle = (command: string) => {
        if (command === 'closeOther') {
            tabList.value = tabList.value.filter(item => item.path === activeTab.value || item.path === '/app')
            // cookie.set('tabList', tabList.value)
        } else if (command === 'closeAll') {
            activeTab.value = '/app'
            tabList.value = [{
                title: '后台首页',
                path: '/app'
            }]
            // cookie.set('tabList', tabList.value)
        }
    }

    return {
        activeTab,
        tabList,
        changeTab,
        removeTab,
        closeHandle
    }
}
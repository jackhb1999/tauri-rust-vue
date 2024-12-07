import {createApp} from 'vue'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import App from './App.vue'
import router from './router'
import 'virtual:windi.css'
import * as ElementPlusIconsVue from '@element-plus/icons-vue'
// import store from "./store";

import "@/router/permission"
import {createPinia} from "pinia";

import "nprogress/nprogress.css"

const pinia = createPinia()

const app = createApp(App)
// app.use(store)
app.use(pinia)
app.use(router)
app.use(ElementPlus)

for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
    app.component(key, component)
}

app.mount("#app");


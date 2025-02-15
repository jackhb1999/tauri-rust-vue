import * as ElementPlusIconsVue from '@element-plus/icons-vue'
import {ListTable, PivotTable, PivotChart} from "@visactor/vtable";
import {App} from "@vue/runtime-core";

// 用于全局注册
export default {
    install(app: App) {
        for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
            app.component(key, component)
        }
        app.component('VueListTable', ListTable);
        app.component('VuePivotTable', PivotTable);
        app.component('VuePivotChart', PivotChart);
    }
}
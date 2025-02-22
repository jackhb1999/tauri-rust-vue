import {defineStore} from "pinia";
import {computed, ComputedRef, Ref} from "vue";
import {useLocalStorage} from "@vueuse/core";


export const useAsideWidthStore = defineStore('asideWidth', () => {

    const asideFold: Ref<boolean> = useLocalStorage('asideFold', false);
    const asideWidth: ComputedRef<string> = computed(() => asideFold.value ? "64px" : "250px")


    function handleAsideWidthChange() {
        console.log(12, asideWidth.value)
        asideFold.value = !asideFold.value

    }


    return {asideFold, asideWidth, handleAsideWidthChange}
})
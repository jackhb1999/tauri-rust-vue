import {defineStore} from "pinia";
import {computed, ComputedRef, Ref, ref} from "vue";


export const useAsideWidthStore = defineStore('asideWidth', () => {

    const asideFold:Ref<boolean> = ref(false)
    const asideWidth: ComputedRef<string> = computed(() => asideFold.value ?"64px":"250px")


    function handleAsideWidthChange() {
        console.log(12,asideWidth.value)
        asideFold.value = !asideFold.value
    }


    return {asideFold,asideWidth, handleAsideWidthChange}
})
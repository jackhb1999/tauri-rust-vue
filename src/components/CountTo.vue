<script setup lang="ts">

import gsap from "gsap";
import {reactive, watch} from "vue";

const props = defineProps({
  value:{
    type: Number,
    default: 0
  }
})

const num = reactive({
  count: 0
})

function AnimationToValue() {
  gsap.to(num, {
    count: props.value,
    duration: 0.5,
  })

}

AnimationToValue()

watch(()=>props.value,()=>{
  AnimationToValue()
})
</script>

<template>
{{num.count.toFixed(0)}}
</template>

<style scoped>

</style>
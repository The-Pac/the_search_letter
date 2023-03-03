<template>
  <div id="spin-container">
    <v-icon name="ri-loader-5-line" scale="3" fill="white" animation="spin"/>
  </div>
</template>

<script lang="ts">
import {defineComponent} from "vue";

export default defineComponent({
  name: "Loading-Component",
})
</script>

<style scoped lang="scss">

#spin-container {
  width: 100px;
  height: 100px;
}

</style>

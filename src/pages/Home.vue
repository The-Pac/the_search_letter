<template>
  <div class="main-app">
    <div id="house-container">
      <house/>
    </div>
  </div>
</template>

<script lang="ts">
import {defineComponent} from "vue";
import House from "../components/House.vue";

export default defineComponent({
  name: "Home-Page",
  components: {House},
})
</script>

<style lang="scss" scoped>


#house-container {
  display: flex;
  justify-content: center;
  align-items: center;
}
</style>

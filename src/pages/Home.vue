<template>
  <div class="main-app">
    <div id="house-container">
      <SearchComponent/>
    </div>
  </div>
</template>

<script lang="ts">
import {defineComponent} from "vue";
import SearchComponent from "../components/Search.vue";

export default defineComponent({
  name: "Home-Page",
  components: {SearchComponent},
})
</script>

<style lang="scss" scoped>


#house-container {
  display: flex;
  justify-content: center;
  align-items: center;
}
</style>

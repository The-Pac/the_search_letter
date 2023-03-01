<template>
  <div class="main-app">
    <div id="house-container">
      <SearchComponent v-if="chars_result.length === 0 " v-model:chars_result="chars_result"/>
      <ResultComponent v-else v-model:chars_result="chars_result"/>
    </div>
  </div>
</template>

<script lang="ts">
import {defineComponent} from "vue";
import SearchComponent from "../components/Search.vue";
import ResultComponent from "../components/Result.vue";

export default defineComponent({
  name: "Home-Page",
  components: {ResultComponent, SearchComponent},
  data() {
    return {
      chars_result: Array()
    }
  }
})
</script>

<style lang="scss" scoped>


#house-container {
  display: flex;
  justify-content: center;
  align-items: center;
}
</style>

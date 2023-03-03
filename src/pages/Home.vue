<template>
  <div class="main-app">
    <SearchComponent v-if="!test_result" v-model:test_result="test_result"/>
    <ResultComponent v-else v-model:test_result="test_result"/>
  </div>
</template>

<script lang="ts">
import {defineComponent} from "vue";
import SearchComponent from "../components/Search.vue";
import ResultComponent from "../components/Result.vue";
import Result from "../models/Result";

export default defineComponent({
  name: "Home-Page",
  components: {ResultComponent, SearchComponent},
  data() {
    return {
      test_result: Result
    }
  }
})
</script>

<style lang="scss" scoped>
.main-app {
  height: 90%;
  display: flex;
  justify-content: center;
  align-items: center;
}
</style>

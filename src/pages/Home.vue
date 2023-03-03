<template>
  <div class="main-app">
    <SearchComponent
        v-if="!finished"
        v-model:test_result="test_result"
        v-model:finished="finished"
        v-model:is_loading="is_loading"
    />
    <ResultComponent v-else v-model:test_result="test_result"/>
    <LoadingComponent v-if="is_loading"/>
  </div>
</template>

<script lang="ts">
import {defineComponent} from "vue";
import SearchComponent from "../components/Search.vue";
import ResultComponent from "../components/Result.vue";
import Result from "../app/models/Result";
import LoadingComponent from "../components/Loading.vue";

export default defineComponent({
  name: "Home-Page",
  components: {LoadingComponent, ResultComponent, SearchComponent},
  data() {
    return {
      is_loading: false,
      finished: false,
      test_result: Result
    }
  }
})
</script>

<style lang="scss" scoped>
.main-app {
  height: 90%;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}
</style>

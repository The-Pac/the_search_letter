<template>
  <div id="search-bar-container" v-if="filter_extensions.length > 0">
    <select multiple v-model="selected_extensions">
      <option disabled value="">Extension</option>
      <option v-for="extension in filter_extensions" :value="extension">
        <label>{{ extension }}</label>
      </option>
    </select>
    <input type="text" @change="on_change"/>
    <button :disabled="location.length === 0 || selected_extensions.length === 0" @click="on_search">
      <label>
        Search
      </label>
    </button>
  </div>
</template>

<script lang="ts">
import {defineComponent} from "vue";
import {invoke} from "@tauri-apps/api";

export default defineComponent({
  name: "Search-Component",
  data() {
    return {
      location: "",
      filter_extensions: [],
      selected_extensions: [],
      error: undefined
    }
  },
  mounted() {
    invoke("get_all_extensions").then((response: any) => {
      this.filter_extensions = response;
    })
  }
  , methods: {
    on_change(event: any) {
      this.location = event.target.value
    },
    on_search(event: any) {
      invoke("search_letters", {
        "location": this.location,
        "selectedExtensions": this.selected_extensions
      }).then((result: any) => {
        this.$emit('update:test_result', result)
      }).catch((error: any) => {
        this.error = error;
      })
    }
  }
})
</script>

<style lang="scss">
#search-bar-container {
  width: 90%;
  height: 10%;
  display: flex;
  justify-content: center;
  align-items: center;

  input {
    width: 70%;
    padding: 0 10px;
    height: 100%;
    border-top-right-radius: 0;
    border-bottom-right-radius: 0;
  }

  button {
    height: 100%;
    margin: 0;
    border-top-left-radius: 0;
    border-bottom-left-radius: 0;
  }
}
</style>

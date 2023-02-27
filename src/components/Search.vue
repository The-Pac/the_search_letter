<template>
  <div>
    <label>Path:</label>
    <input type="text" @change="on_change"/>
    <button :disabled="location.length === 0" @click="on_search">
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
      error: undefined
    }
  }, methods: {
    on_change(event: any) {
      this.location = event.target.value
    },
    on_search(event: any) {
      invoke("search_letters", {"location": this.location}).then((result: any) => {
        console.log(result)
      }).catch((error: any) => {
        this.error = error;
      })
    }
  }
})
</script>


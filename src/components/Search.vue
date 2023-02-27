<template>
  <div>
    <div>
      <label></label>
      <input type="text" @change=""/>
    </div>
    <div>
      <button :disabled="!location" @click="">
        <label>
          Search
        </label>
      </button>
    </div>
  </div>
</template>

<script lang="ts">
import {defineComponent} from "vue";
import {invoke} from "@tauri-apps/api";

export default defineComponent({
  name: "Search-Component",
  data() {
    return {
      location: undefined
    }
  }, methods: {
    on_search() {
      invoke("search_letters", {"location": location})
    }
  }
})
</script>


<template>
  <div id="result-container">
    <div>
      <label>Duration : {{ chars_result.duration }} secs</label>
    </div>
    <table>
      <thead>
      <tr>
        <th>Character</th>
        <th>Number</th>
        <!--        <th>Percents</th>-->
      </tr>
      </thead>
      <tbody>
      <tr v-for="row in chars_result">
        <td>{{ row.char }}</td>
        <td>{{ row.number }}</td>
        <!--        <td>{{ row.percents }}</td>-->
      </tr>
      </tbody>
    </table>
  </div>
</template>

<script lang="ts">
import {defineComponent} from "vue";

export default defineComponent({
  name: "Result-Component",
  props: {
    chars_result: Array(),
    duration: Number
  },
  mounted() {
    console.log(this.chars_result)
  }
})
</script>

<style lang="scss">
#result-container {
  width: 90%;
  height: 90%;
  display: flex;
  justify-content: center;
  align-content: center;
  flex-direction: column;

}
</style>
<template>
  <div id="result-container" v-if="test_result">
    <div id="result-duration-container">
      <label>Duration : {{ test_result.duration }} secs</label>
    </div>
    <div>
      <button @click="show_character = true">Characters</button>
      <button @click="show_character = false">Files</button>
    </div>
    <table v-if="show_character" id="result-table">
      <thead>
      <tr>
        <th>Ranking</th>
        <th>Character</th>
        <th>Number</th>
        <th>Percents</th>
      </tr>
      </thead>
      <tbody>
      <tr v-for="(row,index) in test_result.chars_result">
        <td>{{ index + 1 }}</td>
        <td>{{ row.char }}</td>
        <td>{{ row.number }}</td>
        <td>{{ row.percents }}</td>
      </tr>
      </tbody>
    </table>
    <table v-else id="result-table">
      <thead>
      <tr>
        <th>Ranking</th>
        <th>File name</th>
        <th>Size</th>
        <th>Percents</th>
      </tr>
      </thead>
      <tbody>
      <tr v-for="(row,index) in test_result.files_result">
        <td>{{ index + 1 }}</td>
        <td>{{ row.name }}</td>
        <td>{{ row.size }}</td>
        <td>{{ row.percents }}</td>
      </tr>
      </tbody>
    </table>
  </div>
</template>

<script lang="ts">
import {defineComponent} from "vue";
import Result from "../models/Result";

export default defineComponent({
  name: "Result-Component",
  data() {
    return {
      show_character: true
    }
  }, props: {
    test_result: Result
  }
})
</script>

<style lang="scss">
#result-container {
  width: 80%;
  height: 90%;
  display: flex;
  justify-content: center;
  align-content: center;
  flex-direction: column;

  #result-duration-container {
    width: 100%;
    background: #1C2026;
    border-radius: 5px;
    padding: 10px;
  }


  #result-table {
    margin: 20px;
    padding: 10px;
    height: 100%;
    width: 100%;
    display: flex;
    flex-direction: column;
    overflow-y: scroll;

    tbody {
      height: 100%;
      display: flex;
      flex-direction: column;
      align-items: center;
      width: 100%;

      tr {
        width: 100%;
        display: flex;
        justify-content: space-between;
      }
    }

    thead {
      width: 100%;
      display: flex;
      justify-content: center;
      align-items: center;

      tr {
        width: 100%;
        display: flex;
        justify-content: space-between;
        align-items: center;
      }
    }


  }

  #result-table::-webkit-scrollbar {
    width: 8px;
    height: 8px;
  }

  #result-table::-webkit-scrollbar-thumb {
    background: #cacaca;
    border-radius: 20px;
  }

  #result-table::-webkit-scrollbar-thumb:hover {
    background: #ababab;
  }

  #result-table::-webkit-scrollbar-track {
    background: transparent;
    border-radius: 0;
    box-shadow: inset 0 0 0 0 #F0F0F0;
    margin: 5px 0;
  }
}
</style>
<template>
  <router-view/>
</template>

<script lang="ts" setup>
</script>

<style lang="scss">
@keyframes page_show {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

#app {
  animation: page_show .5s;
  opacity: 1;
  width: 100%;
  height: 100%;
  display: flex;
  justify-content: center;
  align-content: center;
  flex-direction: column;
}

body {
  width: 100%;
  height: 100vh;
  margin: 0;
  display: flex;
  justify-content: center;
  align-items: center;
  overflow-x: clip;
  color: wheat;
  overflow-y: clip;
  background: rgba(0, 0, 0, 0.7);
  font-family: Inter, BlinkMacSystemFont, Segoe UI, Roboto, Oxygen, Ubuntu, Cantarell, Fira Sans, Droid Sans, Helvetica Neue, sans-serif;
}

body::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

body::-webkit-scrollbar-thumb {
  background: #cacaca;
  border-radius: 20px;
}

body::-webkit-scrollbar-thumb:hover {
  background: #ababab;
}

body::-webkit-scrollbar-track {
  background: transparent;
  border-radius: 0;
  box-shadow: inset 0 0 0 0 #F0F0F0;
  margin: 5px 0;
}

</style>

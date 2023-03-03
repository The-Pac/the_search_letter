<template>
  <ul id="pacman-ghosts">
    <li class="pacman">
      <img :src="pacman_open_mouth.src" loading="eager" class="pacman-img" :alt="pacman_open_mouth.alt"/>
    </li>
    <li class="ghost">
      <img :src="ghost_blue.src" loading="eager" class="ghost-img" :alt="ghost_blue.alt"/>
    </li>
    <li class="ghost">
      <img :src="ghost_purple.src" loading="eager" class="ghost-img" :alt="ghost_purple.alt"/>
    </li>
    <li class="ghost">
      <img :src="ghost_red.src" loading="eager" class="ghost-img" :alt="ghost_red.alt"/>
    </li>
    <li class="ghost">
      <img :src="ghost_orange.src" loading="eager" class="ghost-img" :alt="ghost_orange.alt"/>
    </li>
  </ul>
</template>

<script lang="ts">
import {defineComponent} from "vue";

export default defineComponent({
  name: "Loading-Component",
  data() {
    return {
      pacman_open_mouth: {
        src: "src/assets/img/pacman-open-mouth.svg",
        alt: "pacman"
      },
      ghost_blue: {
        src: "src/assets/img/ghost-red.svg",
        alt: "ghost-red"
      },
      ghost_purple: {
        src: "src/assets/img/ghost-blue.svg",
        alt: "ghost-blue"
      },
      ghost_orange: {
        src: "src/assets/img/ghost-purple.svg",
        alt: "ghost-purple"
      },
      ghost_red: {
        src: "src/assets/img/ghost-orange.svg",
        alt: "ghost-orange"
      }
    }
  }
})
</script>

<style scoped lang="scss">
#pacman-ghosts {
  position: relative;
  padding: 0;
  margin: 0;
  min-height: 120px;
  min-width: 120px;
  display: flex;
  justify-content: center;
  align-items: center;
  animation: pacman-chasing 4s infinite linear;

  .pacman {
    display: flex;
    justify-content: flex-start;
    align-items: center;
    position: absolute;
    list-style: none;
    min-width: 120px;
    transform: rotate(0deg);


    .pacman-img {
      transform: rotate(72deg);
      width: 30px;
      height: 30px;
      animation: pacman-eating .3s infinite;
    }
  }

  .ghost {
    display: flex;
    justify-content: flex-start;
    align-items: center;
    position: absolute;
    list-style: none;
    min-width: 120px;


    .ghost-img {
      width: 30px;
      height: 30px;
    }
  }

  .ghost:nth-child(2) {
    transform: rotate(72deg);

    .ghost-img {
      transform: rotate(72deg);
    }
  }

  .ghost:nth-child(3) {
    transform: rotate(144deg);

    .ghost-img {
      transform: rotate(72deg);
    }
  }

  .ghost:nth-child(4) {
    transform: rotate(216deg);

    .ghost-img {
      transform: rotate(72deg);
    }
  }

  .ghost:nth-child(5) {
    transform: rotate(288deg);

    .ghost-img {
      transform: rotate(72deg);
    }
  }

  @keyframes pacman-chasing {
    0% {
      transform: rotate(360deg);
    }
    100% {
      transform: rotate(0deg);
    }
  }

  @keyframes pacman-eating {
    0% {
      content: url("../assets/img/pacman-close-mouth.svg");
    }
    100% {
      content: url("../assets/img/pacman-open-mouth.svg");
    }
  }
}
</style>

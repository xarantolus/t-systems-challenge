<script setup lang="ts">

import {ref} from "vue";
import Map from "./components/Map.vue";
import InteractiveBoard from "./components/InteractiveBoard.vue";

const rawInput = ref('');
const error = ref('');

function start() {
  const ws = new WebSocket("/ws?scenario_id=" + rawInput.value);
  ws.onerror = (event) => {
    console.log(event);
    error.value = "Websocket failed, see console";
  };
  ws.onmessage = (event) => {
    console.log(event.data);
  };
  ws.onopen = (event) => {
    console.log(event);
  };
}
</script>

<template>
  <div class="main-container">
    <div class="left">
      <InteractiveBoard/>
    </div>

    <div class="right">
      <Map/>
    </div>
  </div>
</template>

<style scoped>
/* Main layout container */
.main-container {
  display: flex; /* Enables horizontal layout */
  width: 100vw; /* Takes full screen width */
  height: 100vh; /* Takes full screen height */
}

/* Left section styling */
.left {
  flex: 50%; /* Left section takes remaining space */
}

/* Right section styling */
.right {
  flex: 50%; /* Takes exactly half of the screen width */
}

/* Error message styling */
#error {
  color: red;
}
</style>

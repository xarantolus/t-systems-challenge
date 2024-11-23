<script setup lang="ts">

import {ref} from "vue";
import FrontPage from "./components/FrontPage.vue";
import Map from "./components/Map.vue";
import InteractiveBoard from "./components/InteractiveBoard.vue";

const error = ref('');

let cars = ref([]);
let customers = ref([]);
let connected = ref(false);
let ws: WebSocket | null;

function start(uuid: string) {
  ws = new WebSocket("/ws?scenario_id=" + uuid);
  ws.onerror = (event) => {
    console.log(event);
    error.value = "Websocket failed, see console";
  };
  ws.onmessage = (event) => {
    const data = JSON.parse(event.data);
    cars.value = data.vehicles;
    customers.value = data.customers;
    connected.value = true;
  };
}
</script>

<template>
  <div id="error" v-show="error">{{ error }}</div>
  <FrontPage :start v-if="!connected"/>
  <div class="main-container" v-else>
    <div class="left">
      <InteractiveBoard/>
    </div>

    <div class="right">
      <Map :cars :customers/>
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

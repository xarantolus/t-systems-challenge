<script setup lang="ts">

import {ref} from "vue";
import FrontPage from "./components/FrontPage.vue";
import Map from "./components/Map.vue";
import InteractiveBoard from "./components/InteractiveBoard.vue";
import CurrentLoad from "./components/graph/CurrentLoad.vue";

const error = ref('');

let cars = ref([
  {coordX: 40.7128, coordY: -74.0060, id: 'car1'}
]);
let customers = ref([]);
let connected = ref(false);
let ws: WebSocket | null;

function start(uuid: string, speed: string) {
  ws = new WebSocket("/ws?scenario_id=" + uuid + "&speed=" + speed);
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

const getError = () => error;
</script>

<template>
  <div id="error" v-if="!connected && error">{{ error }}</div>
  <FrontPage :start :error="getError" v-if="!connected"/>
  <div class="main-container" v-else>
    <div class="left">
      <InteractiveBoard/>
      <CurrentLoad/>
    </div>

    <div class="right">
      <Map :cars :customers/>
    </div>
  </div>
</template>

<style scoped>
.main-container {
  display: flex;
  width: 100vw;
  height: 100vh;
}

.left {
  flex: 50%;
}

.right {
  flex: 50%;
  border-radius: 15px;
  overflow: hidden;
}

#error {
  color: red;
}
</style>

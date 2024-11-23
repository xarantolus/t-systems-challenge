<script setup lang="ts">

import {ref} from "vue";

const rawInput = ref('');
const error = ref('');

function start() {
  const ws = new WebSocket("/ws?scenario_id=" + rawInput.value);
  ws.onerror = (event) => {
    console.log(event)
    error.value = "Websocket failed, see console"
  }
  ws.onmessage = (event) => {
    console.log(event.data)
  }
  ws.onopen = (event) => {
    console.log(event)
  }
}
</script>

<template>
  <h1>RöstiCarl</h1>
  <input type="text">
  <button @click="start">Start</button>
  <div id="error" v-show="error">{{ error }}</div>
</template>

<style scoped>
#error {
  color: red;
}
</style>

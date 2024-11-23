<script setup lang="ts">

import { ref } from "vue";
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
    <!-- Left Side -->
    <div class="left">
      <InteractiveBoard />
    </div>

    <!-- Separator -->
    <div class="separator"></div>

    <!-- Right Side -->
    <div class="right">
      <Map />
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
  flex-basis: 50%; /* Left section takes remaining space */
  overflow: auto; /* Ensures scrollable content if necessary */
  padding: 1rem;
}

/* Separator between sections */
.separator {
  width: 2px; /* Set the thickness of the separator */
  background-color: #ccc; /* Light gray separator */
  height: 100%; /* Full height separator */
}

/* Right section styling */
.right {
  flex-basis: 50%; /* Takes exactly half of the screen width */
  background-color: #f9f9f9; /* Optional background color */
  padding: 1rem;
  box-sizing: border-box; /* Ensures padding doesn't add to the width */
}

/* Error message styling */
#error {
  color: red;
}

@import "https://cdn.jsdelivr.net/npm/bulma@1.0.2/css/bulma.min.css";
</style>

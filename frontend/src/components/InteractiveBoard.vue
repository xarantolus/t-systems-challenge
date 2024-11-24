<script setup lang="ts">
import { ref } from 'vue';


const totalRides = ref(0);
const totalRevenue = ref(0);

const updateFunction = () => {
  totalRides.value++;
  totalRevenue.value += Math.max(10, Math.floor(Math.random() * 50));

  setTimeout(updateFunction, Math.random() * 5000);
};

setTimeout(updateFunction, 2500 * Math.random());
</script>

<template>
  <nav class="panel">
    <p class="panel-heading" style="background-color: #e12885; color: white">Stats And Analytics</p>

    <!-- Show Random stats about a robotaxi fleet using bulma.css -->
    <div class="panel-block">
      <div class="columns">
        <div class="column is-4">
          <div class="box">
            <p class="title is-4">Total Rides</p>
            <p class="subtitle is-6">{{ totalRides }}</p>
          </div>
        </div>
        <div class="column is-4">
          <div class="box">
            <p class="title is-4">Total Revenue</p>
            <p class="subtitle is-6">${{totalRevenue}}</p>
          </div>
        </div>
      </div>
    </div>
  </nav>
</template>

<style scoped>
.panel {
  border-radius: 15px;
}

.panel-heading {
  border-top-left-radius: 15px;
  border-top-right-radius: 15px;
}
</style>

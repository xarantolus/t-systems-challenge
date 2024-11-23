<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { Line } from 'vue-chartjs';
import { Chart as ChartJS, Title, Tooltip, LineElement, PointElement, LinearScale, CategoryScale } from 'chart.js';
import ConditionDD from "../dropdowns/ConditionDD.vue";
import FilterDD from "../dropdowns/FilterDD.vue";

// Register required Chart.js components
ChartJS.register(Title, Tooltip, LineElement, PointElement, LinearScale, CategoryScale);

// Reactive variable to hold data
const loadData = ref<number[]>([0, 1, 2, 3, 4, 5]);
const timeLabels = ref<string[]>(['0s', '1s', '2s', '3s', '4s', '5s']);
const currentLoad = ref(0);

// Chart data configuration
const chartData = ref({
  labels: timeLabels.value,
  datasets: [
    {
      label: 'Current Load',
      data: loadData.value,
      borderColor: 'rgb(75, 192, 192)',
      tension: 0.1,
      fill: false,
    },
  ],
});

// Simulate load updates
const simulateLoad = () => {
  // Simulating load change, in a real application this would be dynamic
  loadData.value.push(currentLoad.value);
  loadData.value.shift(); // Remove the first data point
  currentLoad.value = Math.floor(Math.random() * 100); // Simulate a random load value

  // Update chart data
  chartData.value = {
    ...chartData.value,
    labels: timeLabels.value,
    datasets: [
      {
        ...chartData.value.datasets[0],
        data: loadData.value,
      },
    ],
  };
};

// Start load simulation every second
onMounted(() => {
  const interval = setInterval(simulateLoad, 1000);
  onUnmounted(() => clearInterval(interval)); // Cleanup on unmount
});
</script>

<template>
  <div class="container">
    <div class="columns">
      <div class="column is-12">
        <!-- Panel with rounded corners and padding -->
        <nav class="panel">
          <p class="panel-heading" style="background-color: #e12885; color: white">Chart </p>
            <Line :data="chartData" />
        </nav>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* Optional styling for the panel if needed */

.container {
  margin-top: 30px;
}
</style>

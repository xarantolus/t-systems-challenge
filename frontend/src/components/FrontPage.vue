<script lang="ts">

import {ref} from "vue";

export default {
  props: {
    start: Function,
  },
  setup(props) {
    const uuid = ref('');

    const handleStart = () => {
      if (props.start) {
        props.start(uuid.value);
      }
    };

    return {
      uuid,
      handleStart,
    };
  }
};
</script>

<template>
  <h2 class="title is-1">Y-Router</h2>
  <div id="flex">
    <div>
      <h3 class="title is-3">Create Scenario</h3>
      <form action="/scenario/create">
        <div>
          <label for="numCars">Number of Cars:</label>
          <input type="number" id="numCars" min="1" max="50">
        </div>
        <div>
          <label for="numCustomers">Number of Customers:</label>
          <input type="number" id="numCustomers" min="1" max="200">
        </div>
        <button type="submit">Submit</button>
      </form>
    </div>
    <div>
      <h3 class="title is-3">Load scenario</h3>
      <input type="text" placeholder="UUID" v-model="uuid">
      <button @click="handleStart">Start</button>
    </div>
  </div>
</template>

<style scoped>
#flex {
  display: flex;
  flex: content;
  flex-flow: row wrap;
  gap: 2rem;
  justify-items: center;
}
</style>
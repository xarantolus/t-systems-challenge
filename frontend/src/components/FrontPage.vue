<script lang="ts">
import {ref} from "vue";

export default {
  props: {
    start: Function,
    error: Function,
  },
  setup(props) {
    const uuid = ref('');
    const speed = ref('');
    const creating = ref(false);
    const numCars = ref(1);
    const numCustomers = ref(1);

    const handleStart = () => {
      props.start!(uuid.value, speed.value);
    };

    const handleSubmit = async (_: Event) => {
      creating.value = true;
      const queryString = new URLSearchParams({
        numberOfVehicles: numCars.value.toString(),
        numberOfCustomers: numCustomers.value.toString(),
      }).toString();
      const response = await fetch(`/scenario/create?${queryString}`, {method: 'POST',});
      if (!response.ok) {
        props.error!().value = 'Network response was not ok';
        return;
      }
      const data = await response.json();
      creating.value = false;
      props.start!(data.id, speed.value);
    };

    return {
      uuid,
      speed,
      creating,
      numCars,
      numCustomers,
      handleStart,
      handleSubmit,
    };
  }
};
</script>

<template>

    <h2 class="title is-1">Y-Router</h2>
    <div id="flex">
      <div>
        <div class="box">
          <h3 class="title is-3">Set Speed</h3>
          <div class="input-button-container">
            <input class="input is-rounded" type="number" placeholder="Speed" v-model="speed"/>
            <button class="button" @click="handleStart">Submit</button>
          </div>
        </div>
        <div class="box">
          <h3 class="title is-3">Create Scenario</h3>
          <form @submit.prevent="handleSubmit">
            <div>
              <label for="numCars">Number of Cars:</label>
              <input class="input is-rounded" type="number" id="numCars" v-model="numCars" min="1" max="50">
            </div>
            <div class="input-button-container">
              <div>
                <label for="numCustomers">Number of Customers:</label>
                <input class="input is-rounded" type="number" id="numCustomers" v-model="numCustomers" min="1" max="200">
              </div>
              <button type="submit" :class="{'button':true, 'is-loading': creating}">Start</button>
            </div>
          </form>
        </div>
      </div>
      <div class="box">
        <h3 class="title is-3">Load scenario</h3>
        <div class="input-button-container">
          <input class="input is-rounded" type="text" placeholder="UUID" v-model="uuid"/>
          <button class="button" @click="handleStart">Start</button>
        </div>
      </div>
    </div>
</template>

<style scoped>
#flex {
  display: flex;
  justify-content: space-between; /* Distribute boxes evenly */
  gap: 2rem;
  flex-wrap: nowrap; /* Prevent wrapping */
  align-items: flex-start; /* Align items to the top */
}

.box {
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  padding: 1.5rem;
  height: 100%;
  min-height: 200px;
  border: 1px solid #ddd;
  border-radius: 8px;
  box-shadow: 0px 2px 4px rgba(0, 0, 0, 0.1);
  flex: 1; /* Allow boxes to grow equally */
}

.input-button-container {
  display: flex;
  flex-direction: column;
  gap: 10px;
  margin-top: auto;
}

.button {
  background-color: #e12885;
  color: white;
  margin: 0;
  align-self: flex-end;
}

.input.is-rounded {
  margin-bottom: 10px;
}

h3 {
  margin-bottom: auto;
}

</style>
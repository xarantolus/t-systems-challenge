<script lang="ts">
import {ref} from "vue";

export default {
  props: {
    start: Function,
    error: Function,
  },
  setup(props) {
    const uuid = ref('');
    const creating = ref(false);
    const numCars = ref(1);
    const numCustomers = ref(1);

    const handleStart = () => {
      props.start!(uuid.value);
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
      props.start!(data.id);
    };

    return {
      uuid,
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
          <h3 class="title is-3">Create Scenario</h3>
          <form @submit.prevent="handleSubmit">
            <div>
              <label for="numCars">Number of Cars:</label>
              <input type="number" id="numCars" v-model="numCars" min="1" max="50">
            </div>
            <div class="input-button-container">
              <div>
                <label for="numCustomers">Number of Customers:</label>
                <input type="number" id="numCustomers" v-model="numCustomers" min="1" max="200">
              </div>
              <button type="submit" :class="{'button':true, 'is-loading': creating}">Submit</button>
            </div>
          </form>
        </div>
      </div>
      <div class="box">
        <h3 class="title is-3">Load scenario</h3>
        <div class="input-button-container">
          <input type="text" placeholder="UUID" v-model="uuid">
          <button class="button" @click="handleStart">Start</button>
        </div>
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

.button {
  background-color: #e12885;
  color: white;
  margin: 20px;
}

.input-button-container {
  display: flex;
  flex-direction: column;
  gap: 10px; /* Adjust the gap as needed */
}
</style>
<script lang="ts">
import {ref} from "vue";

export default {
  props: {
    start: Function,
    error: Function,
  },
  setup(props) {
    const uuid = ref('');
    const speed = ref(0.033);
    const creating = ref(false);
    const numCars = ref(10);
    const numCustomers = ref(20);

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
  <div id="container">
    <!-- Set Speed box centered at the top -->
    <div class="top-box">
      <div class="box">
        <h3 class="title is-3">Set Speed</h3>
        <div class="input-button-container">
          <input class="input is-rounded" type="number" placeholder="Speed" v-model="speed" />
        </div>
      </div>
    </div>

    <!-- Two boxes (Load and Create Scenario) below -->
    <div id="flex">
      <div class="box">
        <h3 class="title is-3">Load Scenario</h3>
        <div class="input-button-container">
          <input class="input is-rounded" type="text" placeholder="UUID" v-model="uuid" />
          <button class="button" @click="handleStart">Start</button>
        </div>
      </div>

      <div class="box">
        <h3 class="title is-3">Create Scenario</h3>
        <form @submit.prevent="handleSubmit">
          <div>
            <label for="numCars">Number of Cars:</label>
            <input class="input is-rounded" type="number" id="numCars" v-model="numCars" min="1" max="50" />
          </div>
          <div class="input-button-container">
            <div>
              <label for="numCustomers">Number of Customers:</label>
              <input class="input is-rounded" type="number" id="numCustomers" v-model="numCustomers" min="1" max="200" />
            </div>
            <button type="submit" :class="{'button': true, 'is-loading': creating}">Start</button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>



<style scoped>
/* Main container, to position the speed box on top */
#container {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2rem;
  margin-top: 2rem; /* Adds some space at the top */
}

/* Center the "Set Speed" box */
.top-box {
  display: flex;
  justify-content: center; /* Center horizontally */
  width: 100%;
}

/* Main container, to position the speed box on top */
#container {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2rem;
  margin-top: 2rem; /* Adds some space at the top */
}

/* Center the "Set Speed" box */
.top-box {
  display: flex;
  justify-content: center; /* Center horizontally */
  width: 100%;
}

/* Speed box even smaller size */
.top-box .box {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  text-align: center;
  width: 180px; /* Further reduced width */
  padding: 0.75rem; /* Smaller padding */
  min-height: 120px; /* Reduced height */
}

/* Flex container for the Load and Create Scenario boxes below the speed box */
#flex {
  display: flex;
  justify-content: space-between; /* Distribute boxes evenly */
  gap: 2rem;
  width: 100%; /* Allow it to take the full width */
}

/* The box styles */
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
  flex: 1;
}

/* Input field container */
.input-button-container {
  display: flex;
  flex-direction: column;
  gap: 10px;
  margin-top: auto;
}

/* Button styles */
.button {
  background-color: #e12885;
  color: white;
  margin: 0;
  align-self: flex-end;
}

/* Rounded input field styles */
.input.is-rounded {
  margin-bottom: 10px;
  padding: 8px; /* Smaller padding for the input */
  text-align: center; /* Center the text inside the input */
  width: 100%; /* Ensures the input takes full width of the box */
}

/* Center text within the speed input box */
.top-box .box {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  text-align: center;
}

h3 {
  margin-bottom: auto;
}


</style>

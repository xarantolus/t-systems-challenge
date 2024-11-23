<script lang="ts">

</script>

<template>
  <h2>RöstiCarl</h2>
  <div id="flex">
    <div>
      <h3>Create Scenario</h3>
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
      <h3>Load scenario</h3>
      <input type="text" placeholder="UUID">
      <button @click="start">Start</button>
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
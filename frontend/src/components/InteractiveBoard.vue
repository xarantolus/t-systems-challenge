<script setup lang="ts">
import FilterDD from "./Dropdowns/FilterDD.vue";
import ConditionDD from "./Dropdowns/ConditionDD.vue";
</script>

<template>
  <nav class="panel">
    <p class="panel-heading" style="background-color: #e12885; color: white">Filters and Conditions</p>

    <div class="dropdown-container" style="padding: 20px">
      <FilterDD style="padding: 10px"/>
      <ConditionDD style="padding: 10px"/>
    </div>

    <div class="panel-block">
      <button class="button is-link is-outlined is-fullwidth">
        Reset all filters
      </button>
    </div>
  </nav>
  <!-- Container to center dropdowns -->
</template>

<style scoped>
</style>

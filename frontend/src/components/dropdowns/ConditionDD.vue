<script setup lang="ts">
import { ref } from 'vue';

const isActive = ref(false);

const activeItem = ref<string | null>(null);

function toggleDropdown() {
  isActive.value = !isActive.value;
}

function resetDrops() {
  isActive.value = false;
  activeItem.value = null;
}

defineExpose({
  resetDrops
});

function handleItemClick(text: string) {
  activeItem.value = text;
  isActive.value = false;
}
</script>

<template>
  <div :class="['dropdown', { 'is-active': isActive }]">
    <!-- Dropdown Trigger -->
    <div class="dropdown-trigger">
      <button class="button" aria-haspopup="true" aria-controls="dropdown-menu" @click="toggleDropdown">
        <span>{{ activeItem ? activeItem : 'Dropdown button' }}</span>
        <span class="icon is-small">
          <i class="fas fa-angle-down" aria-hidden="true"></i>
        </span>
      </button>
    </div>

    <!-- Dropdown Menu -->
    <div class="dropdown-menu" id="dropdown-menu" role="menu">
      <div class="dropdown-content">
        <a
            href="#"
            class="dropdown-item"
            :class="{ 'is-active': activeItem === 'Least CO2 impact' }"
            @click.prevent="handleItemClick('Least CO2 impact')">
          Least CO2 impact
        </a>
        <a
            href="#"
            class="dropdown-item"
            :class="{ 'is-active': activeItem === 'Shortest distance' }"
            @click.prevent="handleItemClick('Shortest distance')">
          Shortest distance
        </a>
        <a
            href="#"
            class="dropdown-item"
            :class="{ 'is-active': activeItem === 'Shortest travel time' }"
            @click.prevent="handleItemClick('Shortest travel time')">
          Shortest travel time
        </a>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* Add any custom styles here if needed */
</style>


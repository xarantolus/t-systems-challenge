<script setup lang="ts">
import { ref } from 'vue';


// Declare a reactive variable for dropdown state (active/inactive)
const isActive = ref(false);

// Reactive variable to track the active item text
const activeItem = ref<string | null>(null);

// Function to toggle the dropdown visibility
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

// Function to handle item clicks
function handleItemClick(text: string) {
  // Set the clicked item text as active
  activeItem.value = text;
  // Close the dropdown by setting isActive to false
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
            :class="{ 'is-active': activeItem === 'Dropdown item' }"
            @click.prevent="handleItemClick('Dropdown item')">
          Dropdown item
        </a>
        <a
            href="#"
            class="dropdown-item"
            :class="{ 'is-active': activeItem === 'Other dropdown item' }"
            @click.prevent="handleItemClick('Other dropdown item')">
          Other dropdown item
        </a>
        <a
            href="#"
            class="dropdown-item"
            :class="{ 'is-active': activeItem === 'Active dropdown item' }"
            @click.prevent="handleItemClick('Active dropdown item')">
          Active dropdown item
        </a>
        <a
            href="#"
            class="dropdown-item"
            :class="{ 'is-active': activeItem === 'Other dropdown item 2' }"
            @click.prevent="handleItemClick('Other dropdown item 2')">
          Other dropdown item 2
        </a>
        <hr class="dropdown-divider" />
        <a
            href="#"
            class="dropdown-item"
            :class="{ 'is-active': activeItem === 'With a divider' }"
            @click.prevent="handleItemClick('With a divider')">
          With a divider
        </a>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* Add any custom styles here if needed */
</style>

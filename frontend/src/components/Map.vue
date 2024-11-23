<template>
  <div class="map-container">
    <l-map ref="map" v-model:zoom="zoom"
           :center="(cars.length !== 0 && [cars[0].coordX, cars[0].coordY]) || [48.137154, 11.576124]">
      <l-tile-layer
          url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
          layer-type="base"
          name="OpenStreetMap"
      ></l-tile-layer>
      <l-marker
          v-for="(marker, index) in cars"
          :key="index"
          :lat-lng="[marker.coordX, marker.coordY]"
          :icon="marker.customerId ? fullCarIcon : carIcon"
      >
        <l-popup>{{ marker.customerId }}</l-popup>
      </l-marker>
      <template v-for="(marker, index) in customers">
        <l-marker
            v-if="marker.awaitingService"
            :key="index"
            :lat-lng="[marker.coordX, marker.coordY]"
            :icon="personIcon"
        >
          <l-popup>{{ marker.id }}</l-popup>
        </l-marker>
      </template>
    </l-map>
  </div>
</template>

<script>
import "leaflet";
import "leaflet/dist/leaflet.css";
import {LMap, LMarker, LPopup, LTileLayer} from "@vue-leaflet/vue-leaflet";
import car from "../assets/car.png";
import fullCar from "../assets/full_car.png";
import person from "../assets/person.png";

export default {
  props: {
    cars: Array,
    customers: Array,
  },
  components: {
    LMap,
    LTileLayer,
    LMarker,
    LPopup,
  },
  data() {
    return {
      zoom: 12,
      carIcon: L.icon({
        iconUrl: car,
        iconSize: [40, 27],
        iconAnchor: [20, 13],
        popupAnchor: [0, -13],
      }),
      fullCarIcon: L.icon({
        iconUrl: fullCar,
        iconSize: [50, 33],
        iconAnchor: [25, 16],
        popupAnchor: [0, -16],
      }),
      personIcon: L.icon({
        iconUrl: person,
        iconSize: [27, 40],
        iconAnchor: [13, 40],
        popupAnchor: [0, -40],
      }),
    };
  },
};
</script>

<style scoped>
.map-container {
  width: 100%;
  height: 100%;
}
</style>

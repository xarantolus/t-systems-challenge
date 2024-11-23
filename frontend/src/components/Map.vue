<template>
  <div style="height:600px; width:800px">
    <l-map ref="map" v-model:zoom="zoom" :center="[47.41322, -1.219482]">
      <l-tile-layer
          url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
          layer-type="base"
          name="OpenStreetMap"
      ></l-tile-layer>
      <l-marker v-for="(marker, index) in markers" :key="index" :lat-lng="marker.position" :icon="carIcon">
        <l-popup>{{ marker.popupText }}</l-popup>
      </l-marker>
    </l-map>
  </div>
</template>

<script>
import "leaflet";
import "leaflet/dist/leaflet.css";
import {LMap, LMarker, LPopup, LTileLayer} from "@vue-leaflet/vue-leaflet";
import carIconUrl from "../assets/car.png";

export default {
  components: {
    LMap,
    LTileLayer,
    LMarker,
    LPopup
  },
  data() {
    return {
      zoom: 2,
      markers: [
        {position: [47.41322, -1.219482], popupText: "Marker 1"},
        {position: [48.8566, 2.3522], popupText: "Marker 2"},
        {position: [51.5074, -0.1278], popupText: "Marker 3"}
      ], carIcon: L.icon({
        iconUrl: carIconUrl,
        iconSize: [38, 38],
        iconAnchor: [19, 38],
        popupAnchor: [0, -38]
      })
    };
  },
};
</script>

<style></style>
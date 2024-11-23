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
          v-for="(marker, index) in interpolateCarPositions()"
          :key="index"
          :lat-lng="location(marker)"
          :icon="getIconUrl(marker)"
      >
        <l-popup>{{ marker.id }}</l-popup>
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
  methods: {
    getIconUrl(obj) {
      switch (obj.type) {
        case 'stationary_car':
          return this.carIcon;
        case 'stationary_customer':
          return this.personIcon;
        case 'travelling_to_customer':
          return this.carIcon;
        case 'travelling_to_destination':
          return this.fullCarIcon;
      }
    },
    location(obj) {
      if (!obj.location) {
        throw new Error('Object does not have a location');
      }

      return [obj.location.x, obj.location.y];
    },
    interpolateCarPositions() {
        let outputs = [];

      let movingCustomers = new Set();

        for (let i = 0; i < this.cars.length; i++) {
            const car = this.cars[i];

            if (car.customerId) {
              // If we have a customer, we group the car + customer together
              const customer = this.customers.find(customer => customer.id === car.customerId);
              if (!customer) {
                throw new Error('Customer not found');
              }

              // We calculate the position like this:
              // if distanceTravelled == 0, the car is driving to the customer,
              // otherwise the customer is in the car
              // Calculate as location the progress between the car position and customer position,
              // or otherwise between car position and customer destination

              if (car.remainingTravelTime) {
                const progress = car.remainingTravelTime / car.activeTime;
                if (isNaN(progress)) {
                  throw new Error('Progress is NaN');
                }

                // Here we want the x and y between car and customer destinationX/Y
                outputs.push({
                  'type': 'travelling_to_destination',
                  'car': car,
                  'customer': customer,
                  'location': {
                    x: car.coordX + (car.coordX - customer.destinationX) * progress,
                    y: car.coordY + (car.coordY - customer.destinationY) * progress,
                  }
                })
                console.log('progress', progress);

                movingCustomers.add(customer.id);
              } else {
                const travelProgress = car.remainingTravelTime / car.activeTime;
                if (isNaN(travelProgress)) {
                  throw new Error('Travel progress is NaN');
                }

                // Now essentially take the percentage between the car and the customer
                outputs.push({
                  'type': 'travelling_to_customer',
                  'car': car,
                  'customer': customer,
                  'location': {
                    x: car.coordX + (car.coordX - customer.coordX) * travelProgress,
                    y: car.coordY + (car.coordY - customer.coordY) * travelProgress,
                  },
                });
              }
            } else {
              // If we don't have a customer, we add the car to the cars array
              outputs.push({
                'type': 'stationary_car',
                'car': car,
                'location': {
                  x: car.coordX,
                  y: car.coordY,
                },
              });
            }
        }

        // All non-seen customers are added to the outputs array
        for (let i = 0; i < this.customers.length; i++) {
          const customer = this.customers[i];
          if (!movingCustomers.has(customer.id)) {
            outputs.push({
              'type': 'stationary_customer',
              'customer': customer,
              'location': {
                x: customer.coordX,
                y: customer.coordY,
              },
            });
          }
        }

        return outputs;
    }
  }
};
</script>

<style scoped>
.map-container {
  width: 100%;
  height: 100%;
}
</style>

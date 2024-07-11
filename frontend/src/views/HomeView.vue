<script setup lang="ts">
import { ref } from 'vue'

let text = ref();

const fetchAllCharacters = async () => {
  const response = await fetch('http://localhost:8080/get-all')
  const data = await response.json()
  console.log(data)
  text.value = data
}

</script>

<template>
  <main>
    <button @click="fetchAllCharacters">Read all</button>
    <p>{{ text }}</p>
  </main>
</template>

<script setup lang="ts">
import { ref } from 'vue'

let showAddForm = ref<boolean>(false)
let characterToAdd = ref<Character>({
  name: 'Example Name',
  birthday_season: 'spring',
  birthday_day: 1,
  is_bachelor: true,
  best_gift: 'Example Gift',
})

let lastFetchedCharacter = ref<Character | undefined>(undefined)

let showChangeForm = ref<boolean>(false)

let feedback = ref<string>('')

interface Character {
  name: string
  birthday_season: string
  birthday_day: number
  is_bachelor: boolean
  best_gift: string
}

interface CharacterChange {
  name: string
  change_name: string | undefined
  change_birthday_season: string | undefined
  change_birthday_day: number | undefined
  change_is_bachelor: boolean | undefined
  change_best_gift: string | undefined
}

const characterChangeToAdd = ref<CharacterChange>({
  name: 'Example Name',
  change_name: undefined,
  change_birthday_season: undefined,
  change_birthday_day: undefined,
  change_is_bachelor: undefined,
  change_best_gift: undefined
})

const value_names = ['name', 'birthday_season', 'birthday_day', 'is_bachelor', 'best_gift']
const showValueName = ref<string>('name')

let characters = ref<Character[]>([])

const fetchAllCharacters = async () => {
  const response = await fetch('http://localhost:8080/get-all')
  const data = await response.json()

  characters.value = data
}

const fetchCharacter = async (characterName: string) => {
  const response = await fetch(`http://localhost:8080/get/${characterName}`)
  const data = await response.json()

  lastFetchedCharacter.value = data
}

const submitCharacter = async () => {
  const response = await fetch('http://localhost:8080/add', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(characterToAdd.value)
  })

  feedback.value = await response.text()
  void fetchAllCharacters()
}

const changeCharacter = async () => {
  const response = await fetch('http://localhost:8080/change', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(characterChangeToAdd.value)
  })

  feedback.value = await response.text()

}

</script>

<template>
  <main>

    <p>{{ feedback }}</p>

    <button @click="showAddForm = true">Show character submission form</button>
    <div v-if="showAddForm">
      <div>
        <label>Name:</label>
        <input type="text" name="name" v-model="characterToAdd.name" required>
        <label>Birthday Season:</label>
        <input type="text" name="birthday_season" v-model="characterToAdd.birthday_season" required>
        <label>Birthday Day:</label>
        <input type="number" name="birthday_day" v-model="characterToAdd.birthday_day" required>
        <label>Is Bachelor:</label>
        <input type="text" name="is_bachelor" v-model="characterToAdd.is_bachelor" required>
        <label>Best Gift:</label>
        <input type="text" name="best_gift" v-model="characterToAdd.best_gift" required>
        <button @click.preventDefault="submitCharacter">Submit</button>
      </div>
      <div v-if="lastFetchedCharacter">
        <p>👉 Name: {{ lastFetchedCharacter.name }}</p>
        <p>Birthday Season: {{ lastFetchedCharacter.birthday_season }}</p>
        <p>Birthday Day: {{ lastFetchedCharacter.birthday_day }}</p>
        <p>Is Bachelor: {{ lastFetchedCharacter.is_bachelor }}</p>
        <p>Best Gift: {{ lastFetchedCharacter.best_gift }}</p>
      </div>
    </div>

    <br>

    <button @click="showChangeForm = !showChangeForm">Show character submission form</button>
    <div v-if="showChangeForm">

      <label>Character Name:</label>
      <input type="text" name="name" v-model="characterChangeToAdd.name" required>
      <label>Value to change:</label>
      <select name="value_name" v-model="showValueName">
        <option v-for="value_name in value_names" :key="value_name">{{ value_name }}</option>
      </select>
      <label>New Value:</label>
      <input v-if="showValueName === 'name'" type="text" name="new_value" v-model="characterChangeToAdd.change_name"
        required>
      <input v-if="showValueName === 'best_gift'" type="text" name="new_value"
        v-model="characterChangeToAdd.change_best_gift" required>
      <select v-if="showValueName === 'birthday_day'" name="new_value"
        v-model="characterChangeToAdd.change_birthday_day">
        <option v-for="n in 28">{{ n.toString() }}</option>
      </select>
      <select v-if="showValueName === 'is_bachelor'" name="new_value" v-model="characterChangeToAdd.change_is_bachelor">
        <option>true</option>
        <option>false</option>
      </select>
      <select v-if="showValueName === 'birthday_season'" name="new_value"
        v-model="characterChangeToAdd.change_birthday_season">
        <option>spring</option>
        <option>summer</option>
        <option>fall</option>
        <option>winter</option>
      </select>
      <button @click="changeCharacter">Submit</button>
    </div>

    <br>

    <button @click="fetchAllCharacters">{{ characters.length > 0 ? 'Refresh all' : 'Read all' }}</button>
    <div v-for="character in characters">
      <p>👉 Name: {{ character.name }}</p>
      <p>Birthday Season: {{ character.birthday_season }}</p>
      <p>Birthday Day: {{ character.birthday_day }}</p>
      <p>Is Bachelor: {{ character.is_bachelor }}</p>
      <p>Best Gift: {{ character.best_gift }}</p>
      <p>~~~</p>
    </div>
  </main>
</template>

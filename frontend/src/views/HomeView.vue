<script setup lang="ts">
import { ref, watch, computed } from 'vue'

let showAddForm = ref<boolean>(false)
let characterToAdd = ref<Character>({
  name: 'Example Name',
  birthday_season: 'spring',
  birthday_day: 1,
  is_bachelor: true,
  best_gift: 'Example Gift',
})

let prohibitAdding = computed(() => {
  return characterToAdd.value.name === '' || characterToAdd.value.birthday_day === 0 || characterToAdd.value.birthday_season === '' || characterToAdd.value.best_gift === ''
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

const emptyCharacterChangeToAdd = () => {
  characterChangeToAdd.value.change_name = undefined;
  characterChangeToAdd.value.change_birthday_season = undefined,
    characterChangeToAdd.value.change_birthday_day = undefined,
    characterChangeToAdd.value.change_is_bachelor = undefined,
    characterChangeToAdd.value.change_best_gift = undefined
}

const characterChangeToAdd = ref<CharacterChange>({
  name: 'Example Name',
  change_name: undefined,
  change_birthday_season: undefined,
  change_birthday_day: undefined,
  change_is_bachelor: undefined,
  change_best_gift: undefined
})

const seasons = ['spring', 'summer', 'fall', 'winter']

const value_names = ['name', 'birthday_season', 'birthday_day', 'is_bachelor', 'best_gift']
const showValueName = ref<string>('name')

watch(showValueName, () => {
  emptyCharacterChangeToAdd()
})

let characters = ref<Character[]>([])

let showCharacters = ref<boolean>(true)
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
  console.log(characterChangeToAdd.value)
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
    <v-sheet :max-width="400" class="d-flex flex-column">
      <v-card class="pa-4" width="400" v-if="feedback" variant="tonal" color="orange">{{ feedback }}</v-card>
      <v-btn class="ma-2" prepend-icon="mdi-plus" @click="showAddForm = !showAddForm">
        {{ showAddForm ? 'Hide character submission form' : 'Show character submission form' }}
      </v-btn>
      <v-expand-transition>
        <div v-if="showAddForm">
          <div class="d-flex flex-column">
            <v-text-field label="Name" v-model="characterToAdd.name" required />
            <v-select label="Birthday Season" v-model="characterToAdd.birthday_season" :items="seasons"
              required></v-select>
            <v-select label="Birthday Day" v-model="characterToAdd.birthday_day"
              :items="Array.from({ length: 28 }, (_, index) => index + 1)" required></v-select>
            <v-checkbox :label="`Is Bachelor: ${characterToAdd.is_bachelor.toString()}`"
              v-model="characterToAdd.is_bachelor" />
            <v-text-field label="Best Gift" v-model="characterToAdd.best_gift" required />
            <v-btn block color="green" :disabled="prohibitAdding" @click.preventDefault="submitCharacter">Submit new
              character</v-btn>
          </div>
          <v-card v-if="lastFetchedCharacter">
            <p>👉 Name: {{ lastFetchedCharacter.name }}</p>
            <p>Birthday Season: {{ lastFetchedCharacter.birthday_season }}</p>
            <p>Birthday Day: {{ lastFetchedCharacter.birthday_day }}</p>
            <p>Is Bachelor: {{ lastFetchedCharacter.is_bachelor }}</p>
            <p>Best Gift: {{ lastFetchedCharacter.best_gift }}</p>
          </v-card>
        </div>
      </v-expand-transition>

      <v-btn class="ma-2" prepend-icon="mdi-pencil" @click="showChangeForm = !showChangeForm">
        {{ showChangeForm ? 'Hide change form' : 'Show change form' }}
      </v-btn>
      <v-expand-transition>
        <div v-if="showChangeForm" class="v-expand-x-transition">
          <v-text-field label="Name" v-model="characterChangeToAdd.name" required />
          <v-select label="Value to change" v-model="showValueName" :items="value_names" required></v-select>

          <v-text-field label="New Name" v-if="showValueName === 'name'" v-model="characterChangeToAdd.change_name" />
          <v-text-field label="New Best Gift" v-if="showValueName === 'best_gift'"
            v-model="characterChangeToAdd.change_best_gift" />
          <v-select label="New Birthday Day" v-if="showValueName === 'birthday_day'"
            v-model="characterChangeToAdd.change_birthday_day"
            :items="Array.from({ length: 28 }, (_, index) => index + 1)"></v-select>
          <v-select label="New Birthday Season" v-if="showValueName === 'birthday_season'"
            v-model="characterChangeToAdd.change_birthday_season" :items="seasons" />
          <v-checkbox
            :label="`New Is Bachelor: ${characterChangeToAdd.change_is_bachelor !== undefined ? characterChangeToAdd.change_is_bachelor.toString() : '?'}`"
            v-if="showValueName === 'is_bachelor'" v-model="characterChangeToAdd.change_is_bachelor" />
          <v-btn color="green" block @click="changeCharacter">
            Submit change to character
          </v-btn>
        </div>
      </v-expand-transition>

      <br>

      <v-btn class="ma-2" prepend-icon="mdi-book" @click="fetchAllCharacters">
        {{ characters.length > 0 ? 'Refresh all' : 'Read all' }}

      </v-btn>
      <v-btn class="ma-2" prepend-icon="mdi-eye" v-if="characters.length > 0" @click="showCharacters = !showCharacters">
        {{ showCharacters ? 'Hide characters' : 'Show characters' }}
      </v-btn>
      <v-expand-transition>
        <div v-if="showCharacters">
          <v-card class="ma-4 pa-3" v-for="character in characters">
            <p>👉 Name: {{ character.name }}</p>
            <p>Birthday Season: {{ character.birthday_season }}</p>
            <p>Birthday Day: {{ character.birthday_day }}</p>
            <p>Is Bachelor: {{ character.is_bachelor }}</p>
            <p>Best Gift: {{ character.best_gift }}</p>
            <p>~~~</p>
          </v-card>
        </div>
      </v-expand-transition>
    </v-sheet>
  </main>
</template>

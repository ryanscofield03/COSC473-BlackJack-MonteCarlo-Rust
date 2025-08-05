<script setup lang="ts">
import { emptyProbabilitiesData } from '@/types/ProbabilitiesData.ts';
import { ref, watch, onBeforeUnmount } from 'vue';
import Worker from "../wasm-worker.ts?worker";
import type { UserDataStateHolderJS } from '@/types/UserDataStateHolderJS.ts';
import { formatProbabilityPercentage, formatEstimatedValue } from '@/helper.ts';
import InfoModal from '@/components/InfoModal.vue'

const worker = new Worker();
worker.onmessage = function(msg) {
  probabilities.value = msg.data;
  isLoading.value = false
}

onBeforeUnmount(() => {
  worker.terminate();
})

const generateProbabilities = () => {
  const userDataState: UserDataStateHolderJS = {
    currentCards: [...currentCards.value],
    dealerCard: [dealerCard.value],
    numDecks: numDecks.value,
    betSize: betSize.value,
    numSims: numSims.value
  };

  isLoading.value = true;

  worker.postMessage(userDataState);
}

const probabilities = ref(emptyProbabilitiesData);
const currentCards = ref<number[]>([-1, -1]);
const dealerCard = ref<number>(-1);
const numSims = ref<string>('1000000');
const numDecks = ref<string>('2');
const betSize = ref<string>('50');
const isLoading = ref<boolean>(false);
const infoModalOpen = ref<boolean>(false);

watch(
  [currentCards, dealerCard, numDecks, betSize, numSims],
  async () => {
    clearProbabilities();
  },
  { deep: true },
);

const clearProbabilities = () => {
  probabilities.value = { ...emptyProbabilitiesData }
}

const addNewCardEntry = () => {
  if (canAddNewEntry()) {
    currentCards.value.push(-1)
  }
}

const canAddNewEntry = (): boolean => {
  return currentCards.value.length <= 6
}

const removeLastCardEntry = () => {
  if (canRemoveLastEntry()) {
    currentCards.value.pop()
  }
}

const canRemoveLastEntry = (): boolean => {
  return currentCards.value.length > 2
}

const canSplit = () => {
  return currentCards.value.length == 2 && currentCards.value[0] == currentCards.value[1]
}

const openInfoModal = () => {
  infoModalOpen.value = true;
}

const closeInfoModal = () => {
  infoModalOpen.value = false;
}

const cardOptions = [
  { value: 1, name: 'Ace' },
  { value: 2, name: 'Two' },
  { value: 3, name: 'Three' },
  { value: 4, name: 'Four' },
  { value: 5, name: 'Five' },
  { value: 6, name: 'Six' },
  { value: 7, name: 'Seven' },
  { value: 8, name: 'Eight' },
  { value: 9, name: 'Nine' },
  { value: 10, name: 'Ten' },
  { value: 11, name: 'Jack' },
  { value: 12, name: 'Queen' },
  { value: 13, name: 'King' },
]
</script>

<template>
  <main>
    <div style="display: flex; width: 100%; gap: 3rem">
      <div class="box">
        <div style="display: flex; justify-content: space-between; width: 100%">
          <h1>BlackJack Parameters</h1>

          <InfoModal :isOpen="infoModalOpen" :handleClose="closeInfoModal" />
          <button
            @click="openInfoModal()"
            class="btn btn-primary"
            style="padding: 6px 6px 2px 6px; margin-bottom: 8px; height: auto; align-self: center;"
          >
            <img src="../components/icons/help.svg" alt="help" width="32" height="32" />
          </button>
        </div>

        <div class="scrollable-box">
          <div class="input-box">
            <div style="display: flex; justify-content: space-between; width: 100%">
              <label for="user_cards" class="form-label">Current cards</label>

              <div>
                <button
                  @click="removeLastCardEntry"
                  class="btn"
                  style="padding: 8px; margin-right: 8px"
                  :class="canRemoveLastEntry() ? 'btn-error' : 'btn-disabled'"
                >
                  <img src="../components/icons/remove.svg" alt="remove card" width="16" height="16" />
                </button>
                <button
                  @click="addNewCardEntry"
                  class="btn"
                  style="padding: 8px"
                  :class="canAddNewEntry() ? 'btn-primary' : 'btn-disabled'"
                >
                  <img src="../components/icons/add.svg" alt="add card" width="16" height="16" />
                </button>
              </div>
            </div>

            <select
              id="user_cards"
              v-for="(_, index) in currentCards"
              :key="index"
              v-model="currentCards[index]"
              class="form-select"
            >
              <option disabled value="">Please select a card</option>
              <option v-for="option in cardOptions" :key="option.value" :value="option.value">
                {{ option.name }}
              </option>
            </select>
          </div>

          <div class="input-box">
            <label for="dealer_card" class="form-label">Dealer's Card</label>
            <select id="dealer_card" v-model="dealerCard" class="form-select">
              <option disabled value="">Please select a card</option>
              <option v-for="card in cardOptions" :key="card.value" :value="card.value">
                {{ card.name }}
              </option>
            </select>
          </div>

          <div class="input-box">
            <label for="num_sims" class="form-label">Number of Trials</label>
            <input type="text" id="num_sims" v-model="numSims" class="form-control" />
          </div>

          <div class="input-box">
            <label for="bet_size" class="form-label">Bet Size</label>
            <input type="text" id="bet_size" v-model="betSize" class="form-control" />
          </div>

          <div class="input-box">
            <label for="number_of_decks" class="form-label">Number of Decks</label>
            <input type="text" id="number_of_decks" v-model="numDecks" class="form-control" />
          </div>
        </div>
      </div>

      <div class="box">
        <div style="display: flex; justify-content: space-between; width: 100%">
          <h1>Estimates</h1>
          <div>
            <button
              :disabled="isLoading"
              @click="generateProbabilities"
              class="btn"
              style="padding: 8px 8px 6px 8px;margin-top: 6px; height: auto; align-self: center;"
              :class="!isLoading ? 'btn-primary' : 'btn-disabled'"
            >
              <img src="../components/icons/refresh.svg" alt="refresh" width="24" height="24" />
            </button>
          </div>
        </div>

        <div class="scrollable-box">
          <div class="action-box">
            <div style="display: flex; justify-content: space-between; width: 100%">
              <h2>Stand</h2>
              <h2 :style="{ color: probabilities.standEV >= 0 ? '#c3ffc0' : '#ffc0cc' }">
                {{ formatEstimatedValue(probabilities.standEV) }}
              </h2>
            </div>
            <ul>
              <h3>
                <li>Win: {{ formatProbabilityPercentage(probabilities.standWin) }}</li>
              </h3>
              <h3>
                <li>Lose: {{ formatProbabilityPercentage(probabilities.standLoss) }}</li>
              </h3>
              <h3>
                <li>Tie: {{ formatProbabilityPercentage(probabilities.standTie) }}</li>
              </h3>
            </ul>
          </div>

          <div class="action-box">
            <h2>Hit</h2>

            <ul>
              <div style="display: flex; justify-content: space-between; width: 100%">
                <h3><li>Once</li></h3>
                <h3 :style="{ color: probabilities.hitOnceEV >= 0 ? '#c3ffc0' : '#ffc0cc' }">
                  {{ formatEstimatedValue(probabilities.hitOnceEV) }}
                </h3>
              </div>
              <ul>
                <h4>
                  <li>Win: {{ formatProbabilityPercentage(probabilities.hitOnceWin) }}</li>
                </h4>
                <h4>
                  <li>Lose: {{ formatProbabilityPercentage(probabilities.hitOnceLoss) }}</li>
                </h4>
                <h4>
                  <li>Tie: {{ formatProbabilityPercentage(probabilities.hitOnceTie) }}</li>
                </h4>
              </ul>
            </ul>

            <ul>
              <div style="display: flex; justify-content: space-between; width: 100%">
                <h3><li>Twice</li></h3>
                <h3 :style="{ color: probabilities.hitTwiceEV >= 0 ? '#c3ffc0' : '#ffc0cc' }">
                  {{ formatEstimatedValue(probabilities.hitTwiceEV) }}
                </h3>
              </div>
              <ul>
                <h4>
                  <li>Win: {{ formatProbabilityPercentage(probabilities.hitTwiceWin) }}</li>
                </h4>
                <h4>
                  <li>Lose: {{ formatProbabilityPercentage(probabilities.hitTwiceLoss) }}</li>
                </h4>
                <h4>
                  <li>Tie: {{ formatProbabilityPercentage(probabilities.hitTwiceTie) }}</li>
                </h4>
              </ul>
            </ul>

            <ul>
              <div style="display: flex; justify-content: space-between; width: 100%">
                <h3><li>Thrice</li></h3>
                <h3 :style="{ color: probabilities.hitThriceEV >= 0 ? '#c3ffc0' : '#ffc0cc' }">
                  {{ formatEstimatedValue(probabilities.hitThriceEV) }}
                </h3>
              </div>
              <ul>
                <h4>
                  <li>Win: {{ formatProbabilityPercentage(probabilities.hitThriceWin) }}</li>
                </h4>
                <h4>
                  <li>Lose: {{ formatProbabilityPercentage(probabilities.hitThriceLoss) }}</li>
                </h4>
                <h4>
                  <li>Tie: {{ formatProbabilityPercentage(probabilities.hitThriceTie) }}</li>
                </h4>
              </ul>
            </ul>
          </div>

          <div v-if="canSplit()" class="action-box">
            <h2>Split and Hit</h2>

            <ul>
              <div style="display: flex; justify-content: space-between; width: 100%">
                <h3><li>Once</li></h3>
                <h3 :style="{ color: probabilities.splitHitOnceEV >= 0 ? '#c3ffc0' : '#ffc0cc' }">
                  {{ formatEstimatedValue(probabilities.splitHitOnceEV) }}
                </h3>
              </div>
              <ul>
                <h4>
                  <li>Win: {{ formatProbabilityPercentage(probabilities.splitHitOnceWin) }}</li>
                </h4>
                <h4>
                  <li>Lose: {{ formatProbabilityPercentage(probabilities.splitHitOnceLoss) }}</li>
                </h4>
                <h4>
                  <li>Tie: {{ formatProbabilityPercentage(probabilities.splitHitOnceTie) }}</li>
                </h4>
              </ul>
            </ul>

            <ul>
              <div style="display: flex; justify-content: space-between; width: 100%">
                <h3><li>Twice</li></h3>
                <h3 :style="{ color: probabilities.splitHitTwiceEV >= 0 ? '#c3ffc0' : '#ffc0cc' }">
                  {{ formatEstimatedValue(probabilities.splitHitTwiceEV) }}
                </h3>
              </div>
              <ul>
                <h4>
                  <li>Win: {{ formatProbabilityPercentage(probabilities.splitHitTwiceWin) }}</li>
                </h4>
                <h4>
                  <li>Lose: {{ formatProbabilityPercentage(probabilities.splitHitTwiceLoss) }}</li>
                </h4>
                <h4>
                  <li>Tie: {{ formatProbabilityPercentage(probabilities.splitHitTwiceTie) }}</li>
                </h4>
              </ul>
            </ul>

            <ul>
              <div style="display: flex; justify-content: space-between; width: 100%">
                <h3><li>Thrice</li></h3>
                <h3 :style="{ color: probabilities.splitHitThriceEV >= 0 ? '#c3ffc0' : '#ffc0cc' }">
                  {{ formatEstimatedValue(probabilities.splitHitThriceEV) }}
                </h3>
              </div>
              <ul>
                <h4>
                  <li>Win: {{ formatProbabilityPercentage(probabilities.splitHitThriceWin) }}</li>
                </h4>
                <h4>
                  <li>Lose: {{ formatProbabilityPercentage(probabilities.splitHitThriceLoss) }}</li>
                </h4>
                <h4>
                  <li>Tie: {{ formatProbabilityPercentage(probabilities.splitHitThriceTie) }}</li>
                </h4>
              </ul>
            </ul>
          </div>
        </div>
      </div>
    </div>
  </main>
</template>

<style scoped>
.form-select,
.form-control {
  width: 100%;
  padding: 12px;
  margin-top: 8px;
  margin-bottom: 8px;
  border: 0;
  background-color: #b4b4b4;
  color: #000000;
}

.form-label {
  color: white;
  font-size: 20px;
}

.box {
  width: 400px;
  gap: 20px;
  padding: 20px 20px 0 20px;
  background-color: #505050;
  border-radius: 8px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}

.input-box {
  background-color: #282828;
  padding: 12px 12px 12px 12px;
  border-radius: 12px;
  margin-bottom: 12px;
}

.action-box {
  background-color: #282828;
  padding: 4px 12px 12px 12px;
  border-radius: 12px;
  margin-bottom: 16px;
}

.scrollable-box {
  max-height: 535px;
  overflow-y: auto;
}

.box > div {
  margin-bottom: 10px;
}
</style>

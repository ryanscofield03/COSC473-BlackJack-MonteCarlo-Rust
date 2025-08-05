<script setup lang="ts">
defineProps({
  isOpen: {
    type: Boolean,
    required: true
  },
  handleClose: {
    type: Function,
    required: true
  },
});
</script>

<template>
  <Teleport to="#modal" transition-transform="transition-transform">
    <Transition name="modal">
      <div v-if="isOpen" class="modal-mask" @click="handleClose()">
        <div class="modal-box" @click.stop>
          <h1>FAQs</h1>

          <div class="modal-content scrollable-box">
            <h2>How do I use this app?</h2>
            <h4>
              Enter values in the <b>BlackJack Parameters Box</b>. Then click the <b>refresh icon</b>
              in the Probabilities box to run a Monte Carlo simulation for each possible action.
            </h4>

            <spacer></spacer>

            <h2>Great, but how does it work?</h2>
            <h4>
              This app runs a <b>Monte Carlo simulation</b> on your inputs, which simulates the possible
              different moves (e.g stand, hit, split). The outcomes of the simulations for each move
              is then averaged to find estimate probabilities of wins, losses, and ties.
            </h4>

            <spacer></spacer>

            <h2>Why are the probabilities different with the same inputs?</h2>
            <h4>
              With the simulations <b>relying on randomness</b>, there will be some variation in the
              results. Try increasing the number of trials to decrease variation!
            </h4>

            <spacer></spacer>

            <h2>What about doubling down?</h2>
            <h4>
              Since doubling down affects EV but <b>not probabilities</b>, it has been omitted.
            </h4>

            <spacer></spacer>

            <h2>Why are the probabilities not updating?</h2>
            <h4>
              When there are a significant number of trials, say <b>one million</b>, it can take
              many seconds to compute. Web Workers help to prevent freezing, but your browser
              may not support them.
            </h4>

            <spacer></spacer>

            <h2>What are the limitations of this tool?</h2>
            <h4>
              <ol>
                <li>
                  Assumes the deck(s) is full before your cards and dealers card have been removed.
                  To calculate the EV, we would need to know the cards that have already been removed
                  from the deck.
                </li>
                <li>
                  Split probabilities assume you hit the same number of times on both hands for
                  simplicity reasons. In reality, you may hit twice on one hand and twice on the
                  other hand, leading to a better EV.
                </li>
                <li>
                  This does not account for rules such as insurance or surrendering for simplicity.
                  These actions may have a better EV than what this tool finds with some parameters.
                </li>
              </ol>
            </h4>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.modal-mask {
  position: fixed;
  z-index: 9999;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.75);
  display: flex;
  justify-content: end;
  transition: 0.5s;
}

.modal-box {
  width: 23%;
  background-color: #505050;
  padding: 20px;
  border-radius: 8px;
  box-shadow: 0 4px 10px rgba(0, 0, 0, 0.3);
  transition: 0.5s;
}

.scrollable-box {
  height: 91%;
  overflow-y: auto;
}

.modal-content {
  background-color: #313131;
  padding: 20px;
  border-radius: 8px;
  box-shadow: 0 4px 10px rgba(0, 0, 0, 0.3);
}

h2 {
  line-height: 1.4;
  margin-bottom: 6px;
}

b {
  font-weight: bolder;
}

li {
  margin-bottom: 12px;
}

.modal-enter-from .modal-box {
  opacity: 0;
}

.modal-enter-to .modal-box {
  animation: bounceIn 0.5s;
}

.modal-leave-from .modal-box {
  opacity: 1;
}

.modal-leave-to .modal-box {
  animation: bounceOut 0.5s;
  opacity: 0;
}

@keyframes bounceIn {
  0% {
    transform: translateX(100%);
  }
  80% {
    transform: translateX(-10px);
  }
  100% {
    transform: translateX(0);
  }
}

@keyframes bounceOut {
  0% {
    transform: translateX(0);
  }
  20% {
    transform: translateX(-10px);
  }
  100% {
    transform: translateX(100%);
  }
}
</style>

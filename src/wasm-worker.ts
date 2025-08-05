import init, { ActionOutcomes, Card, UserDataStateHolder } from 'wasm-module-monte-carlo'
import { emptyProbabilitiesData, type ProbabilitiesData } from '@/types/ProbabilitiesData.ts'
import type { UserDataStateHolderJS } from '@/types/UserDataStateHolderJS.ts'
let actionOutcomes: ActionOutcomes | null = null;

const initializeWasm = async () => {
  await init()
  actionOutcomes = ActionOutcomes.new()
}

initializeWasm()

const cardValueMapping = [
  { value: 1, card: Card.Ace },
  { value: 2, card: Card.Two },
  { value: 3, card: Card.Three },
  { value: 4, card: Card.Four },
  { value: 5, card: Card.Five },
  { value: 6, card: Card.Six },
  { value: 7, card: Card.Seven },
  { value: 8, card: Card.Eight },
  { value: 9, card: Card.Nine },
  { value: 10, card: Card.Ten },
  { value: 11, card: Card.Jack },
  { value: 12, card: Card.Queen },
  { value: 13, card: Card.King },
]

const toCards = (cardValueList: number[]): Card[] => {
  return cardValueList
    .filter((value: number) => value !== -1)
    .map((value: number) => cardValueMapping
      .find(mapping => mapping.value == value)!.card);
}

self.onmessage = function (event) {
  let probabilities = emptyProbabilitiesData
  const userDataStateJS: UserDataStateHolderJS = event.data;
  const userDataState = UserDataStateHolder.new(
    toCards(userDataStateJS.currentCards),
    toCards(userDataStateJS.dealerCard),
    userDataStateJS.numDecks,
    userDataStateJS.betSize,
    userDataStateJS.numSims
  )

  probabilities = generateProbabilities(userDataState)
  postMessage(probabilities)
}

const generateProbabilities = (userDataState: UserDataStateHolder): ProbabilitiesData => {
  if (actionOutcomes != null) {
    try {
      const result = actionOutcomes.generate_all_action_outcomes(userDataState);
      if (result == undefined) return emptyProbabilitiesData;

      const probabilities = emptyProbabilitiesData;
      probabilities.standEV = result["stand"]["estimated_value"];
      probabilities.standWin = result["stand"]["win"];
      probabilities.standLoss = result["stand"]["loss"];
      probabilities.standTie = result["stand"]["tie"];

      // hit once
      probabilities.hitOnceEV = result["hit_once"]["estimated_value"];
      probabilities.hitOnceWin = result["hit_once"]["win"];
      probabilities.hitOnceLoss = result["hit_once"]["loss"];
      probabilities.hitOnceTie = result["hit_once"]["tie"];

      // hit twice
      probabilities.hitTwiceEV = result["hit_twice"]["estimated_value"];
      probabilities.hitTwiceWin = result["hit_twice"]["win"];
      probabilities.hitTwiceLoss = result["hit_twice"]["loss"];
      probabilities.hitTwiceTie = result["hit_twice"]["tie"];

      // hit thrice
      probabilities.hitThriceEV = result["hit_thrice"]["estimated_value"];
      probabilities.hitThriceWin = result["hit_thrice"]["win"];
      probabilities.hitThriceLoss = result["hit_thrice"]["loss"];
      probabilities.hitThriceTie = result["hit_thrice"]["tie"];

      // split hit once
      probabilities.splitHitOnceEV = result["split_hit_once"]["estimated_value"];
      probabilities.splitHitOnceWin = result["split_hit_once"]["win"];
      probabilities.splitHitOnceLoss = result["split_hit_once"]["loss"];
      probabilities.splitHitOnceTie = result["split_hit_once"]["tie"];

      // split hit twice
      probabilities.splitHitTwiceEV = result["split_hit_twice"]["estimated_value"];
      probabilities.splitHitTwiceWin = result["split_hit_twice"]["win"];
      probabilities.splitHitTwiceLoss = result["split_hit_twice"]["loss"];
      probabilities.splitHitTwiceTie = result["split_hit_twice"]["tie"];

      // split hit thrice
      probabilities.splitHitThriceEV = result["split_hit_thrice"]["estimated_value"];
      probabilities.splitHitThriceWin = result["split_hit_thrice"]["win"];
      probabilities.splitHitThriceLoss = result["split_hit_thrice"]["loss"];
      probabilities.splitHitThriceTie = result["split_hit_thrice"]["tie"];

      return probabilities;
    } catch {
      return emptyProbabilitiesData;
    }
  }
  return emptyProbabilitiesData;
}
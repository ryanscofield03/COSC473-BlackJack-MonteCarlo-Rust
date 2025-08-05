use std::num::{ParseFloatError, ParseIntError};
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

/// Generalises errors of parsing numbers
enum ParseNumberError {
    Int(ParseIntError),
    Float(ParseFloatError)
}

/// This is used by the JS code to store user inputs and send it to Rust code
#[wasm_bindgen]
pub struct UserDataStateHolder {
    current_cards: Vec<Card>,
    dealer_card: Vec<Card>,
    num_decks: String,
    bet_size: String,
    num_sims: String
}

#[wasm_bindgen]
impl UserDataStateHolder {
    pub fn new(
        current_cards: Vec<Card>,
        dealer_card: Vec<Card>,
        num_decks: String,
        bet_size: String,
        num_sims: String
    ) -> Self {
        UserDataStateHolder {
            current_cards,
            dealer_card,
            num_decks,
            bet_size,
            num_sims,
        }
    }

    /// Converts this to a UserDataState, which is used by Rust
    /// This parses data and handles errors if there are any
    fn to_user_data_state(self) -> Result<UserDataState, ParseNumberError> {
        let current_cards = self.current_cards
            .into_iter()
            .filter(|card| *card != Card::Empty)
            .collect();
        let dealer_card = self.dealer_card
            .into_iter()
            .filter(|card| *card != Card::Empty)
            .collect();

        // parse and return err if we cannot parse it
        let num_decks: u8 = match self.num_decks.parse::<u8>() {
            Ok(value) => value,
            Err(err) => return Err(ParseNumberError::Int(err))
        };
        let bet_size: f64 = match self.bet_size.parse::<f64>() {
            Ok(value) => value,
            Err(err) => return Err(ParseNumberError::Float(err))
        };
        let num_sims: u32 = match self.num_sims.parse::<u32>() {
            Ok(value) => value,
            Err(err) => return Err(ParseNumberError::Int(err))
        };

        Ok(UserDataState {
            current_cards,
            dealer_card,
            num_decks,
            bet_size,
            num_sims
        })
    }
}

/// Struct that stores values for our monte carlo simulation
/// We can be sure that at this stage, values have been sanitised
struct UserDataState {
    current_cards: Vec<Card>,
    dealer_card: Vec<Card>,
    num_decks: u8,
    bet_size: f64,
    num_sims: u32
}

impl UserDataState {
    /// Checks the validity of user inputs (it must have a possible state of a BJ game)
    fn is_valid(&self) -> bool {
        self.current_cards.len() >= 2
            && self.dealer_card.len() == 1
            && self.num_decks >= 1
            && self.num_sims >= 1
    }
}

/// Enum type for BJ cards
#[derive(Clone)]
#[wasm_bindgen]
#[derive(Debug, PartialEq)]
pub enum Card {
    Empty, // would prefer to use an Option if JS could make them
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Card {
    /// Stores a list of values for each Card, this must be a list since Ace equals 1 or 11
    fn get_card_values(&self) -> Vec<u8> {
        match self {
            Card::Empty => vec![],
            Card::Ace => vec![1, 11],
            Card::Two => vec![2],
            Card::Three => vec![3],
            Card::Four => vec![4],
            Card::Five => vec![5],
            Card::Six => vec![6],
            Card::Seven => vec![7],
            Card::Eight => vec![8],
            Card::Nine => vec![9],
            Card::Ten => vec![10],
            Card::Jack => vec![10],
            Card::Queen => vec![10],
            Card::King => vec![10],
        }
    }
}

/// Stores a list of cards for the entire deck
/// This means that the "deck" may actually be 6 decks
#[derive(Clone)]
struct Deck {
    cards: Vec<Card>
}

impl Deck {
    /// Initialises the deck to be num_decks copies of a standard 52-card deck
    pub fn new(
        num_decks: &u8
    ) -> Self {
        // list of each card
        let mut cards = vec![
            Card::Ace, Card::Two, Card::Three, Card::Four,
            Card::Five, Card::Six, Card::Seven, Card::Eight,
            Card::Nine, Card::Ten, Card::Jack, Card::Queen,
            Card::King];

        // duplicate 4 times (to make a deck) and num_deck times to make number of decks
        cards = cards
            .into_iter()
            .flat_map(|x: Card| std::iter::repeat(x).take((4 * num_decks) as usize))
            .collect::<Vec<Card>>();

        Deck {
            cards
        }
    }

    /// Removes a card from the deck, required for removing cards that are known
    /// e.g. player's cards and dealer's card
    pub fn remove_card_from_deck(&mut self, card: &Card) {
        if let Some(index) = self.cards.iter().position(|c| c == card) {
            self.cards.swap_remove(index);
        }
    }

    /// Takes a random card from the deck and returns it, useful for drawing a new card
    /// in our simulation.
    pub fn take_random_card_from_deck(&mut self) -> Card {
        match getrandom::u64() {
            Ok(value) => {
                let random_index = (value % self.cards.len() as u64) as usize;
                self.cards.remove(random_index)
            }
            Err(_) => {
                self.cards.remove(0)
            }
        }
    }
}

/// Holder for the data we want to send to JS
#[derive(Serialize, Deserialize)]
struct ProbabilityValueOutcomes {
    estimated_value: f64,
    win: f64,
    loss: f64,
    tie: f64
}

impl ProbabilityValueOutcomes {
    /// Initialise to a 50/50 chance to win or lose with EV of 0
    pub fn new() -> Self {
        ProbabilityValueOutcomes {
            estimated_value: 0.0,
            win: 0.5,
            loss: 0.5,
            tie: 0.0,
        }
    }
}

/// Enum holder for different game outcomes
#[derive(Debug, PartialEq)]
enum GameOutcome {
    WIN,
    LOSS,
    TIE
}

/// Holder for different BJ actions, HIT and SPLIT have u8s to
/// store the number of times the player will hit (e.g. SPLIT(2) means split and hit twice)
enum BlackJackAction {
    HIT(u8),
    STAND,
    SPLIT(u8)
}

/// Holder for the different actions to send back to JS
#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct ActionOutcomes {
    hit_once: ProbabilityValueOutcomes,
    hit_twice: ProbabilityValueOutcomes,
    hit_thrice: ProbabilityValueOutcomes,
    stand: ProbabilityValueOutcomes,
    split_hit_once: ProbabilityValueOutcomes,
    split_hit_twice: ProbabilityValueOutcomes,
    split_hit_thrice: ProbabilityValueOutcomes,
}

#[wasm_bindgen]
impl ActionOutcomes {
    pub fn new() -> Self {
        ActionOutcomes {
            hit_once: ProbabilityValueOutcomes::new(),
            hit_twice: ProbabilityValueOutcomes::new(),
            hit_thrice: ProbabilityValueOutcomes::new(),
            stand: ProbabilityValueOutcomes::new(),
            split_hit_once: ProbabilityValueOutcomes::new(),
            split_hit_twice: ProbabilityValueOutcomes::new(),
            split_hit_thrice: ProbabilityValueOutcomes::new(),
        }
    }

    /// Sets each action value to default without changing the object
    fn clear(&mut self) {
        self.hit_once = ProbabilityValueOutcomes::new();
        self.hit_twice = ProbabilityValueOutcomes::new();
        self.hit_thrice = ProbabilityValueOutcomes::new();
        self.stand = ProbabilityValueOutcomes::new();
        self.split_hit_once = ProbabilityValueOutcomes::new();
        self.split_hit_twice = ProbabilityValueOutcomes::new();
        self.split_hit_thrice = ProbabilityValueOutcomes::new();
    }

    /// Generates probabilities and EVs for all possible moves given BJ game state
    pub fn generate_all_action_outcomes(&mut self, data: UserDataStateHolder) -> Result<JsValue, JsValue> {
        let data: UserDataState = match data.to_user_data_state() {
            Ok(value) => value,
            Err(_) => return Err(Default::default())
        };

        if !data.is_valid() { return Err(Default::default()); }

        // currently we "hit" 6 times but could bring this down to 3 - unsure if this would
        // make it much faster however.
        self.hit_once = self.generate_outcomes(&data, BlackJackAction::HIT(1));
        self.hit_twice = self.generate_outcomes(&data, BlackJackAction::HIT(2));
        self.hit_thrice = self.generate_outcomes(&data, BlackJackAction::HIT(3));

        self.stand = self.generate_outcomes(&data, BlackJackAction::STAND);

        if can_split_hand(&data.current_cards) {
            self.split_hit_once = self.generate_outcomes(&data, BlackJackAction::SPLIT(1));
            self.split_hit_twice = self.generate_outcomes(&data, BlackJackAction::SPLIT(2));
            self.split_hit_thrice = self.generate_outcomes(&data, BlackJackAction::SPLIT(3));
        }

        let response = Ok(serde_wasm_bindgen::to_value(&self)?);
        self.clear();
        response
    }

    /// Generates probabilities and EVs for a single action
    fn generate_outcomes(&self, data: &UserDataState, action: BlackJackAction) -> ProbabilityValueOutcomes {
        let mut wins = 0;
        let mut losses = 0;
        let mut ties = 0;

        // remove known cards in dealer/player hands from deck
        let mut deck = Deck::new(&data.num_decks);
        data.current_cards.iter().for_each(|card| deck.remove_card_from_deck(card));
        data.dealer_card.iter().for_each(|card| deck.remove_card_from_deck(card));

        for _ in 0..data.num_sims {
            let mut current_deck = deck.clone();
            let draw_card = &mut || current_deck.take_random_card_from_deck();

            let mut player_cards = data.current_cards.clone();
            handle_player_action(
                &mut player_cards,
                &action,
                draw_card
            );

            let mut dealer_cards = data.dealer_card.clone();
            handle_dealer_action(
                &mut dealer_cards,
                draw_card
            );

            let outcome = evaluate_hands(
                &player_cards,
                &dealer_cards
            );

            match outcome {
                GameOutcome::WIN => wins += 1,
                GameOutcome::LOSS => losses += 1,
                GameOutcome::TIE => ties += 1
            }
        }

        let win_probability = wins as f64 / data.num_sims as f64;
        let loss_probability = losses as f64 / data.num_sims as f64;
        let tie_probability = ties as f64 / data.num_sims as f64;
        let estimated_value = (win_probability * data.bet_size)
            - (loss_probability * data.bet_size); // ignore ties as it doesnt change ev

        ProbabilityValueOutcomes {
            estimated_value,
            win: win_probability,
            loss: loss_probability,
            tie: tie_probability
        }
    }
}

/// Makes a move depending on the given player action
fn handle_player_action(
    player_cards: &mut Vec<Card>,
    action: &BlackJackAction,
    draw_card: &mut impl FnMut()->Card
) {
    match action {
        BlackJackAction::HIT(num_hits) => {
            for _ in 0..*num_hits {
                player_cards.push(draw_card());
            }
        }
        BlackJackAction::STAND => {
            // do nothing if we stand
        }
        BlackJackAction::SPLIT(num_hits) => {
            player_cards.remove(1);

            // for the sake of simplicity, we just pretend the outcome of 1
            // split hand is the outcome of both, since this is basically
            // what happens given the law of large numbers
            for _ in 0..*num_hits {
                player_cards.push(draw_card());
            }
        }
    }
}

/// Handles the dealer drawing until they reach 17 or higher
fn handle_dealer_action(
    dealer_cards: &mut Vec<Card>,
    draw_card: &mut impl FnMut()->Card
) {
    // if any iteration of the dealer's hand is >= 17, then they stand
    while evaluate_hand(&dealer_cards).iter().all(|x| *x <= 16) {
        dealer_cards.push(draw_card());
    }
}

/// Evaluates the players and dealers cards after they have both made their actions
/// and then returns an outcome from the player's perspective.
///
/// They should win if their best hand beats the dealer's best hand
/// Tie if their best hand matches the dealer's best hand
/// Lose if their best hand is worse than the dealer's best hand
fn evaluate_hands(players_cards: &Vec<Card>, dealers_cards: &Vec<Card>) -> GameOutcome {
    let player_evaluations = evaluate_hand(players_cards);
    let dealer_evaluations = evaluate_hand(dealers_cards);

    let player_best_option = player_evaluations
        .into_iter()
        .filter(|&value| value <= 21)
        .max();

    let dealer_best_option = dealer_evaluations
        .into_iter()
        .filter(|&value| value <= 21)
        .max();

    match (player_best_option, dealer_best_option) {
        (None, _) => GameOutcome::LOSS,
        (Some(_), None) => GameOutcome::WIN,
        (Some(player_best_value), Some(dealer_best_value)) => {
            if player_best_value == dealer_best_value {
                GameOutcome::TIE
            } else if player_best_value > dealer_best_value {
                GameOutcome::WIN
            } else {
                GameOutcome::LOSS
            }
        }
    }
}

/// Check if the player's hand can be split, if it can, return true
fn can_split_hand(hand: &Vec<Card>) -> bool {
    hand.len() == 2 && hand[0] == hand[1]
}


/// Evaluates a hand and returns a list of possible values
fn evaluate_hand(cards: &[Card]) -> Vec<u8> {
    let value_mapping = cards
        .iter()
        .map(|card| card.get_card_values())
        .collect();
    generate_value_combinations(&value_mapping)
}

/// Generates all combinations of evaluations of a hand
fn generate_value_combinations(card_values: &Vec<Vec<u8>>) -> Vec<u8> {
    let n =  card_values.len();
    let mut results = vec![];
    let mut stack = vec![(0, 0)];

    while let Some((index, current_sum)) = stack.pop() {
        if index == n { // assure that we stop once we have combinations of length n
            results.push(current_sum);
            continue;
        }

        for &value in &card_values[index] {
            stack.push((index + 1, current_sum + value));
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_value_combinations() {
        let first_values = vec![1, 11];
        let second_values = vec![7];
        let third_values = vec![1, 11];
        let input = vec![first_values, second_values, third_values];
        let expected = vec![9, 19, 19, 29];

        let mut result = generate_value_combinations(&input);
        result.sort();

        assert_eq!(expected, result, "Expected {:?} but got {:?}", expected, result);
    }

    #[test]
    fn test_evaluate_hand() {
        let hand = vec![Card::Ace, Card::Five, Card::Three];
        let expected = vec![9, 19];

        let mut result = evaluate_hand(&hand);
        result.sort();

        assert_eq!(expected, result, "Expected {:?} but got {:?}", expected, result);
    }

    #[test]
    fn test_evaluate_hands_player_busts() {
        let player_hand = vec![Card::Jack, Card::Five, Card::Seven];
        let dealers_hand = vec![Card::Jack, Card::Six, Card::Queen];
        let expected_outcome = GameOutcome::LOSS;

        let actual_outcome = evaluate_hands(
            &player_hand,
            &dealers_hand
        );

        assert_eq!(expected_outcome, actual_outcome,
                   "Expected {:?} but got {:?}", expected_outcome, actual_outcome);
    }

    #[test]
    fn test_evaluate_hands_player_dealer_busts_player_does_not_bust() {
        let player_hand = vec![Card::Jack, Card::Five];
        let dealers_hand = vec![Card::Jack, Card::Six, Card::Queen];
        let expected_outcome = GameOutcome::WIN;

        let actual_outcome = evaluate_hands(
            &player_hand,
            &dealers_hand
        );

        assert_eq!(expected_outcome, actual_outcome,
                   "Expected {:?} but got {:?}", expected_outcome, actual_outcome);
    }

    #[test]
    fn test_evaluate_hands_when_equal() {
        let player_hand = vec![Card::Six, Card::Five];
        let dealers_hand = vec![Card::Six, Card::Five];
        let expected_outcome = GameOutcome::TIE;

        let actual_outcome = evaluate_hands(
            &player_hand,
            &dealers_hand
        );

        assert_eq!(expected_outcome, actual_outcome,
                   "Expected {:?} but got {:?}", expected_outcome, actual_outcome);
    }

    #[test]
    fn test_evaluate_hands_when_equal_with_aces() {
        let player_hand = vec![Card::Six, Card::Five, Card::Ace];
        let dealers_hand = vec![Card::Six, Card::Five, Card::Ace];
        let expected_outcome = GameOutcome::TIE;

        let actual_outcome = evaluate_hands(
            &player_hand,
            &dealers_hand
        );

        assert_eq!(expected_outcome, actual_outcome,
                   "Expected {:?} but got {:?}", expected_outcome, actual_outcome);
    }

    #[test]
    fn test_generate_hit_outcomes_ten_thousand_sims() {
        let action_outcomes = ActionOutcomes::new();
        let outcome = action_outcomes.generate_outcomes(
            &UserDataState {
                current_cards: vec![Card::Ace, Card::Jack],
                dealer_card: vec![Card::Six],
                num_decks: 10,
                bet_size: 100.0,
                num_sims: 10_000,
            },
            BlackJackAction::HIT(1)
        );

        assert_ne!(outcome.win, 1.0);
        assert_ne!(outcome.win, 0.0);

        assert_ne!(outcome.loss, 1.0);
        assert_ne!(outcome.loss, 0.0);

        assert_ne!(outcome.tie, 1.0);
        assert_ne!(outcome.tie, 0.0);
    }

    #[test]
    fn test_generate_stand_outcomes_ten_thousand_sims() {
        let action_outcomes = ActionOutcomes::new();
        let outcome = action_outcomes.generate_outcomes(
            &UserDataState {
                current_cards: vec![Card::Ace, Card::Queen],
                dealer_card: vec![Card::Six],
                num_decks: 10,
                bet_size: 100.0,
                num_sims: 10_000,
            },
            BlackJackAction::STAND
        );

        assert!(outcome.win > outcome.loss);

        assert_ne!(outcome.win, 1.0);
        assert_ne!(outcome.win, 0.0);

        assert_eq!(outcome.loss, 0.0);

        assert_ne!(outcome.tie, 1.0);
        assert_ne!(outcome.tie, 0.0);
    }

    #[test]
    fn test_generate_stand_outcomes_expected_loss_ten_thousand_sims() {
        let action_outcomes = ActionOutcomes::new();
        let outcome = action_outcomes.generate_outcomes(
            &UserDataState {
                current_cards: vec![Card::Two],
                dealer_card: vec![Card::Six],
                num_decks: 10,
                bet_size: 100.0,
                num_sims: 10_000,
            },
            BlackJackAction::STAND
        );

        assert!(outcome.loss > outcome.win);
    }

    #[test]
    fn test_new_user_data_does_not_break_when_cannot_parse() {
        let _user_data = UserDataStateHolder::new(
            vec![],
            vec![],
            "hello this is num_decks".to_string(),
            "2".to_string(),
            "1 million!".to_string()
        );
    }

    #[test]
    fn test_holder_to_user_data_conversion_ok() {
        let user_data = UserDataStateHolder::new(
            vec![Card::Ace, Card::Two],
            vec![Card::Jack],
            "1".to_string(),
            "1".to_string(),
            "1".to_string()
        );

        let user_data = user_data.to_user_data_state();
        match user_data {
            Ok(_) => {},
            Err(_) => panic!("Test failed: Should not Err")
        }
    }

    #[test]
    fn test_holder_to_user_data_conversion_err() {
        let user_data = UserDataStateHolder::new(
            vec![Card::Ace, Card::Two],
            vec![Card::Jack],
            "hello this is num_decks".to_string(),
            "2".to_string(),
            "1 million!".to_string()
        );

        let user_data = user_data.to_user_data_state();
        match user_data {
            Ok(_) => panic!("Test failed: Should not Ok"),
            Err(_) => {}
        }
    }

    #[test]
    fn test_holder_empty_cards_are_removed() {
        let user_data = UserDataStateHolder::new(
            vec![Card::Empty, Card::Two],
            vec![Card::Empty],
            "1".to_string(),
            "1".to_string(),
            "1".to_string()
        );

        let user_data = user_data.to_user_data_state();
        match user_data {
            Ok(user_data) => {
                assert_eq!(1, user_data.current_cards.len());
                assert_eq!(0, user_data.dealer_card.len());
            }
            Err(_) => panic!("Test failed: Should not Err"),
        }
    }
}
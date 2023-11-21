use std::collections::HashMap;
use std::str::FromStr;

/// Given a list of poker hands, return a list of those hands which win.
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    // Create a vector of poker hands from given hands, and map to index of original hands
    let mut poker_hands = Vec::new();

    for (i, hand) in hands.iter().enumerate() {
        let poker_hand = PokerHand::from_str(hand).unwrap();
        poker_hands.push((i, poker_hand));
    }

    // Sort the poker hands by hand_rank
    poker_hands.sort_by(|(_, a), (_, b)| b.hand_rank().cmp(&a.hand_rank()));
    // Get the highest hand_rank
    let highest_rank = poker_hands[0].1.hand_rank();

    // Get the winning hands via hand_rank
    let winning_hands = poker_hands
        .iter()
        .filter(|(_, poker_hand)| poker_hand.hand_rank() == highest_rank)
        .map(|&(i, _)| hands[i].clone())
        .collect::<Vec<_>>();
    println!("Winning hands: {:?}", winning_hands);

    // If a winner found via hand_rank, return
    if winning_hands.len() == 1 {
        return winning_hands;
    }

    // Sort the poker hands by rank
    poker_hands.sort_by(|(_, a), (_, b)| b.rank().cmp(&a.rank()));
    // Get the highest rank
    let highest_rank_v2 = poker_hands[0].1.rank();

    // Get the winning hands via rank
    let winning_hands_v2 = poker_hands
        .iter()
        .filter(|(_, poker_hand)| poker_hand.rank() == highest_rank_v2)
        .map(|&(i, _)| hands[i].clone())
        .collect::<Vec<_>>();
    println!("Winning hands_v2: {:?}", winning_hands_v2);

    // Need to handle ties when multiple winning hands, need to find the high cards to compare.
    // if equal high card ranks, pot is split

    if winning_hands_v2.len() == 1 {
        return winning_hands_v2;
    }

    // Get the kicker card for each hand, compare them to find the highest kicker card
    let mut highest_kicker_card = vec![];
    let mut highest_kicker_card_hands = vec![];

    for hand in winning_hands_v2 {
        let poker_hand = PokerHand::from_str(hand).unwrap();
        let hand_kicker_card = poker_hand.kicker_card();

        if let Some(kicker_card) = hand_kicker_card {
            if highest_kicker_card.is_empty() {
                highest_kicker_card = vec![kicker_card];
                highest_kicker_card_hands = vec![hand];
            } else if kicker_card > highest_kicker_card[0] {
                highest_kicker_card = vec![kicker_card];
                highest_kicker_card_hands = vec![hand];
            } else if kicker_card == highest_kicker_card[0] {
                highest_kicker_card.push(kicker_card);
                highest_kicker_card_hands.push(hand);
            }
        }
    }

    // If we have one highest kicker card hands, return it
    if highest_kicker_card_hands.len() == 1 {
        // Return the highest kicker card hands
        return highest_kicker_card_hands;
    }

    // If multiple highest kicker card hands, need to compare the next highest card
    // until we have checked all cards in hand, or have just one winner
    let all_hands = highest_kicker_card_hands.clone();
    let hands_cards = all_hands
        .iter()
        .map(|hand| PokerHand::from_str(hand).unwrap().cards())
        .collect::<Vec<_>>();

    // While our all_hands has more than one hand, keep checking the next highest card, until we have checked all cards
    let mut current_card_index = 0;
    let mut current_hands = all_hands.clone();
    let mut current_hands_cards = hands_cards.clone();

    while current_hands.len() > 1 && current_card_index < 5 {
        // Get the current card rank
        let current_card_rank = current_hands_cards[0][current_card_index].rank;
        // Check each hands_cards hands to see if it has the current card rank
        let mut next_hands = vec![];
        let mut next_hands_cards = vec![];
        for (i, hand) in current_hands.iter().enumerate() {
            if current_hands_cards[i][current_card_index].rank == current_card_rank {
                next_hands.push(hand.clone());
                next_hands_cards.push(current_hands_cards[i].clone());
            } else {
                // keep the hand that has the larger value
                if current_hands_cards[i][current_card_index].rank > current_card_rank {
                    next_hands = vec![hand.clone()];
                    next_hands_cards = vec![current_hands_cards[i].clone()];
                    current_card_index = 5;
                }
            }
        }

        // If we have one next hand, return it
        if next_hands.len() == 1 {
            // Return the next hand
            return next_hands;
        }

        // If we have multiple next hands, keep checking the next highest card
        current_hands = next_hands.clone();
        current_hands_cards = next_hands_cards.clone();
        current_card_index += 1;
    }

    return all_hands;
}

/// A poker hand is a collection of 5 cards with a rank and a suit.
/// The rank is a number from 2 to 14 (Jack, Queen, King, Ace)
/// The suit is a character: 'H', 'S', 'D', 'C' (Hearts, Spades, Diamonds, Clubs)
/// Cards are separated by spaces.
/// Examples:
/// 4S 5S 6S 8D 3C (4 of Spades, 5 of Spades, 6 of Spades, 8 of Diamonds, 3 of Clubs)
/// 2S 4C 7S 9H 10H (2 of Spades, 4 of Clubs, 7 of Spades, 9 of Hearts, 10 of Hearts)
/// 3S 4S 5D 6H JH (3 of Spades, 4 of Spades, 5 of Diamonds, 6 of Hearts, Jack of Hearts)
///
/// A poker hand is one of the following types, in decreasing order of value:
/// High Card: Highest value card.
/// One Pair: Two cards of the same value.
/// Two Pairs: Two different pairs.
/// Three of a Kind: Three cards of the same value.
/// Straight: All cards are consecutive values.
/// Flush: All cards of the same suit.
/// Full House: Three of a kind and a pair.
/// Four of a Kind: Four cards of the same value.
/// Straight Flush: All cards are consecutive values of same suit.
/// Royal Flush: Ten, Jack, Queen, King, Ace, in same suit.
///
/// The cards are valued in the order:
/// 2, 3, 4, 5, 6, 7, 8, 9, 10, Jack, Queen, King, Ace.
///
/// The suit has no value.
///
/// The result of your poker hand is a tuple containing:
/// 1) the type of hand
/// 2) the highest card in the hand
/// 3) the cards, sorted by rank, descending
///
/// Examples:
/// ("High Card", 8, vec![8, 6, 5, 4, 2])
/// ("One Pair", 11, vec![11, 11, 8, 7, 3])
/// ("Two Pairs", 11, vec![11, 11, 3, 3, 8])
///
///

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PokerHand {
    cards: Vec<Card>,
}

impl PokerHand {
    pub fn new(cards: Vec<Card>) -> Self {
        PokerHand { cards }
    }

    pub fn rank(&self) -> PokerHandRank {
        // Return the rank of the hand
        // The rank is one of the following types, in decreasing order of value:
        // High Card: Highest value card.
        // One Pair: Two cards of the same value.
        // Two Pairs: Two different pairs.
        // Three of a Kind: Three cards of the same value.
        // Straight: All cards are consecutive values.
        // Flush: All cards of the same suit.
        // Full House: Three of a kind and a pair.
        // Four of a Kind: Four cards of the same value.
        // Straight Flush: All cards are consecutive values of same suit.
        // Royal Flush: Ten, Jack, Queen, King, Ace, in same suit.

        // Check for a royal flush
        if self.is_royal_flush() {
            return PokerHandRank::RoyalFlush;
        }

        // Check for a straight flush
        if self.is_straight_flush() {
            return PokerHandRank::StraightFlush;
        }

        // Check for a four of a kind
        if self.is_four_of_a_kind().is_some() {
            return PokerHandRank::FourOfAKind(self.quad_rank());
        }

        // Check for a full house
        if self.is_full_house().is_some() {
            return PokerHandRank::FullHouse(self.triplet_rank(), self.pair_rank());
        }

        // Check for a flush
        if self.is_flush() {
            return PokerHandRank::Flush;
        }

        // Check for a straight
        if self.is_straight() {
            return PokerHandRank::Straight;
        }

        // Check for a three of a kind
        if self.is_three_of_a_kind().is_some() {
            return PokerHandRank::ThreeOfAKind(self.triplet_rank());
        }

        // Check for two pairs
        if self.is_two_pairs().is_some() {
            return PokerHandRank::TwoPairs(self.higher_pair_rank(), self.lower_pair_rank());
        }

        // Check for a pair
        if self.is_one_pair().is_some() {
            return PokerHandRank::OnePair(self.pair_rank());
        }

        // Return high card
        PokerHandRank::HighCard
    }

    pub fn highest_card(&self) -> u8 {
        // Return the highest card in the hand
        // If there are multiple cards with the same rank, return either one

        // Sort the cards by rank
        let mut cards = self.cards.clone();
        cards.sort_by(|a, b| b.rank.cmp(&a.rank));

        // Return the highest card
        cards[0].rank
    }

    pub fn cards(&self) -> Vec<Card> {
        // Return the cards in the hand, sorted by rank, descending

        // Sort the cards by rank
        let mut cards = self.cards.clone();
        cards.sort_by(|a, b| b.rank.cmp(&a.rank));

        // Return the cards
        let mut cards_vec = Vec::new();
        for card in cards {
            cards_vec.push(card);
        }
        cards_vec
    }

    pub fn kicker_card(&self) -> Option<u8> {
        match self.rank() {
            PokerHandRank::HighCard => Some(self.highest_card()),
            PokerHandRank::OnePair(_) => self.kicker_card_for_pairs(),
            PokerHandRank::TwoPairs(_, _) => self.kicker_card_for_pairs(),
            PokerHandRank::ThreeOfAKind(_) => self.kicker_card_for_three_of_a_kind(),
            PokerHandRank::Straight => self.highest_card_except_ace(),
            PokerHandRank::Flush => Some(self.highest_card()),
            PokerHandRank::FullHouse(_, _) => None,
            PokerHandRank::FourOfAKind(_) => self.kicker_card_for_four_of_a_kind(),
            PokerHandRank::StraightFlush => self.highest_card_except_ace(),
            PokerHandRank::RoyalFlush => None,
        }
    }

    pub fn hand_rank(&self) -> u8 {
        // Return the rank of the hand as a number, ranked by which hand beats another(reverse so cmp wrks w/o reverse)
        match self.rank() {
            PokerHandRank::HighCard => 1,
            PokerHandRank::OnePair(_) => 2,
            PokerHandRank::TwoPairs(_, _) => 3,
            PokerHandRank::ThreeOfAKind(_) => 4,
            PokerHandRank::Straight => 5,
            PokerHandRank::Flush => 6,
            PokerHandRank::FullHouse(_, _) => 7,
            PokerHandRank::FourOfAKind(_) => 8,
            PokerHandRank::StraightFlush => 9,
            PokerHandRank::RoyalFlush => 10,
        }
    }

    fn quad_rank(&self) -> u8 {
        // Find the rank of the quad in a four of a kind
        // Iterate over the cards and find the rank that appears four times
        let mut rank_counts = HashMap::new();
        for card in &self.cards {
            *rank_counts.entry(card.rank).or_insert(0) += 1;
        }

        for (rank, count) in rank_counts.iter() {
            if *count == 4 {
                return *rank;
            }
        }

        panic!("No triplet found in the hand");
    }

    fn triplet_rank(&self) -> u8 {
        // Find the rank of the triplet in a full house or three of a kind
        // Iterate over the cards and find the rank that appears three times
        let mut rank_counts = HashMap::new();
        for card in &self.cards {
            *rank_counts.entry(card.rank).or_insert(0) += 1;
        }

        for (rank, count) in rank_counts.iter() {
            if *count == 3 {
                return *rank;
            }
        }

        panic!("No triplet found in the hand");
    }

    fn pair_rank(&self) -> u8 {
        // Find the rank of the pair in a full house or one pair
        // Iterate over the cards and find the rank that appears twice
        let mut rank_counts = HashMap::new();
        for card in &self.cards {
            *rank_counts.entry(card.rank).or_insert(0) += 1;
        }

        for (rank, count) in rank_counts.iter() {
            if *count == 2 {
                return *rank;
            }
        }

        panic!("No pair found in the hand");
    }

    fn higher_pair_rank(&self) -> u8 {
        // Find the rank of the higher pair in two pairs
        // Iterate over the cards and find the ranks that appear twice
        // Return the higher rank of the two
        let mut rank_counts = HashMap::new();
        for card in &self.cards {
            *rank_counts.entry(card.rank).or_insert(0) += 1;
        }

        let mut higher_rank = 0;
        for (rank, count) in rank_counts.iter() {
            if *count == 2 && *rank > higher_rank {
                higher_rank = *rank;
            }
        }

        higher_rank
    }

    fn lower_pair_rank(&self) -> u8 {
        // Find the rank of the lower pair in two pairs
        // Iterate over the cards and find the ranks that appear twice
        // Return the lower rank of the two
        let mut rank_counts = HashMap::new();
        for card in &self.cards {
            *rank_counts.entry(card.rank).or_insert(0) += 1;
        }

        let mut lower_rank = u8::MAX;
        for (rank, count) in rank_counts.iter() {
            if *count == 2 && *rank < lower_rank {
                lower_rank = *rank;
            }
        }

        lower_rank
    }

    fn kicker_card_for_four_of_a_kind(&self) -> Option<u8> {
        // Return the kicker card for a four of a kind
        // The kicker card is the highest card that is not part of the four of a kind

        // Iterate over the cards and count the occurrences of each rank
        let mut rank_counts = HashMap::new();
        for card in &self.cards {
            *rank_counts.entry(card.rank).or_insert(0) += 1;
        }

        // Return the highest card that is not part of the four of a kind
        for card in &self.cards {
            if rank_counts[&card.rank] == 1 {
                return Some(card.rank);
            }
        }

        None
    }

    fn highest_card_except_ace(&self) -> Option<u8> {
        let mut ranks: Vec<u8> = self.cards.iter().map(|card| card.rank).collect();
        ranks.sort_by(|a, b| b.cmp(a));

        for rank in ranks {
            if rank != 14 {
                return Some(rank);
            }
        }

        None
    }

    fn kicker_card_for_three_of_a_kind(&self) -> Option<u8> {
        // Return the kicker card for a three of a kind
        // The kicker card is the highest card that is not part of the three of a kind

        // Iterate over the cards and count the occurrences of each rank
        let mut rank_counts = HashMap::new();
        for card in &self.cards {
            *rank_counts.entry(card.rank).or_insert(0) += 1;
        }

        // Return the highest card that is not part of the three of a kind
        for card in &self.cards {
            if rank_counts[&card.rank] == 1 {
                return Some(card.rank);
            }
        }

        None
    }

    fn kicker_card_for_pairs(&self) -> Option<u8> {
        // Return the kicker card for a pair or two pairs
        // The kicker card is the highest card that is not part of the pair(s)

        // Iterate over the cards and count the occurrences of each rank
        let mut rank_counts = HashMap::new();
        for card in &self.cards {
            *rank_counts.entry(card.rank).or_insert(0) += 1;
        }

        // Return the highest card that is not part of the pair(s)
        for card in &self.cards {
            if rank_counts[&card.rank] == 1 {
                return Some(card.rank);
            }
        }

        None
    }

    // Helper functions for hands
    fn is_royal_flush(&self) -> bool {
        // Check if the hand is a royal flush
        // Royal Flush: Ten, Jack, Queen, King, Ace, in same suit.

        // Check if the hand contains a 10, Jack, Queen, King, and Ace
        let mut contains_10 = false;
        let mut contains_jack = false;
        let mut contains_queen = false;
        let mut contains_king = false;
        let mut contains_ace = false;
        for card in &self.cards {
            match card.rank {
                10 => contains_10 = true,
                11 => contains_jack = true,
                12 => contains_queen = true,
                13 => contains_king = true,
                14 => contains_ace = true,
                _ => (),
            }
        }

        // Return true if the hand contains a 10, Jack, Queen, King, and Ace and is a flush
        contains_10
            && contains_jack
            && contains_queen
            && contains_king
            && contains_ace
            && self.is_flush()
    }

    fn is_straight_flush(&self) -> bool {
        // Check if the hand is a straight flush
        // Straight Flush: All cards are consecutive values of same suit.

        // Return true if the hand is a straight and a flush
        self.is_straight() && self.is_flush()
    }

    fn is_flush(&self) -> bool {
        // Check if the hand is a flush
        // Flush: All cards of the same suit.

        // Check if all the cards are the same suit
        let suit = self.cards[0].suit;
        for card in &self.cards {
            if card.suit != suit {
                return false;
            }
        }

        true
    }

    fn is_straight(&self) -> bool {
        // Check if the hand is a straight
        // Straight: All cards are consecutive values.

        // Check if the cards are consecutive
        let mut cards = self.cards.clone();
        cards.sort_by(|a, b| b.rank.cmp(&a.rank));

        if cards[0].rank == 14 && cards[1].rank == 5 {
            // Ace-low straight: Ace, 2, 3, 4, 5
            for i in 2..cards.len() {
                if cards[i - 1].rank - cards[i].rank != 1 {
                    return false;
                }
            }
            return true;
        }

        for i in 1..cards.len() {
            if cards[i - 1].rank - cards[i].rank != 1 {
                return false;
            }
        }

        true
    }

    fn is_full_house(&self) -> Option<(u8, u8)> {
        // Check if the hand is a full house
        // Full House: Three of a kind and a pair.

        // Check if the hand has three of a kind and a pair
        // self.is_kinds(3, 1) && self.is_kinds(2, 1)
        // Check if the hand has three of a kind and a pair
        if let Some(triplet) = self.is_kinds(3) {
            if let Some(pair) = self.is_kinds(2) {
                return Some((triplet, pair));
            }
        }

        None
    }

    fn is_kinds(&self, count: usize) -> Option<u8> {
        // Check if the hand has "count" cards of the same value in "amount" groups

        // Iterate over the cards and count the occurrences of each rank
        let mut rank_counts = HashMap::new();
        for card in &self.cards {
            *rank_counts.entry(card.rank).or_insert(0) += 1;
        }

        // Find the rank that has "count" occurrences
        let mut rank = None;
        for (&r, &c) in &rank_counts {
            if c == count {
                rank = Some(r);
                break;
            }
        }

        // Return the rank if it exists in the specified amount
        rank.and_then(|r| {
            if let Some(&c) = rank_counts.get(&r) {
                if c == count {
                    Some(r)
                } else {
                    None
                }
            } else {
                None
            }
        })
    }

    fn is_kinds_except(&self, excluded_rank: u8, count: usize) -> Option<u8> {
        // Check if the hand has "count" cards of the same value in "amount" groups,
        // excluding the specified rank

        // Iterate over the cards and count the occurrences of each rank
        let mut rank_counts = HashMap::new();
        for card in &self.cards {
            *rank_counts.entry(card.rank).or_insert(0) += 1;
        }

        // Find the rank that has "count" occurrences (excluding the specified rank)
        let mut rank = None;
        for (&r, &c) in &rank_counts {
            if r != excluded_rank && c == count {
                rank = Some(r);
                break;
            }
        }

        // Return the rank if it exists in the specified amount
        rank.and_then(|r| {
            if let Some(&c) = rank_counts.get(&r) {
                if c == count {
                    Some(r)
                } else {
                    None
                }
            } else {
                None
            }
        })
    }

    fn is_two_pairs(&self) -> Option<(u8, u8)> {
        // Check if the hand is two pairs
        // Two Pairs: Two different pairs.

        // Iterate over the cards and count the occurrences of each rank
        // Check if the hand has two pairs
        if let Some(pair1) = self.is_kinds(2) {
            if let Some(pair2) = self.is_kinds_except(pair1, 2) {
                return Some((pair1, pair2));
            }
        }

        None
    }

    fn is_four_of_a_kind(&self) -> Option<u8> {
        // Check if the hand is a four of a kind
        // Four of a Kind: Four cards of the same value.

        self.is_kinds(4)
    }

    fn is_three_of_a_kind(&self) -> Option<u8> {
        // Check if the hand is a three of a kind
        // Three of a Kind: Three cards of the same value.

        self.is_kinds(3)
    }

    fn is_one_pair(&self) -> Option<u8> {
        // Check if the hand is a two of a kind (pair)
        // Two of a Kind: Two cards of the same value.

        self.is_kinds(2)
    }
}

impl FromStr for PokerHand {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Parse the cards from the string
        // Return an error if the string is not a valid poker hand

        // Split the string into cards
        let cards_str = s.split_whitespace();

        // Parse each card
        let mut cards = Vec::new();
        for card_str in cards_str {
            let card = Card::from_str(card_str)?;
            cards.push(card);
        }

        Ok(PokerHand::new(cards))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Card {
    rank: u8,
    suit: Suit,
}

impl Card {
    pub fn new(rank: u8, suit: Suit) -> Self {
        Card { rank, suit }
    }
}

impl FromStr for Card {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Parse the rank and suit from the string
        // Return an error if the string is not a valid card

        // Split the string in 2, first is the rank, second is the suit
        let (rank_str, suit_str) = s.split_at(s.len() - 1);

        // Parse the rank
        let rank = match rank_str {
            "2" => 2,
            "3" => 3,
            "4" => 4,
            "5" => 5,
            "6" => 6,
            "7" => 7,
            "8" => 8,
            "9" => 9,
            "10" => 10,
            "J" => 11,
            "Q" => 12,
            "K" => 13,
            "A" => 14,
            _ => return Err("Invalid rank"),
        };

        // Parse the suit
        let suit = match suit_str {
            "H" => Suit::Hearts,
            "S" => Suit::Spades,
            "D" => Suit::Diamonds,
            "C" => Suit::Clubs,
            _ => return Err("Invalid suit"),
        };

        Ok(Card::new(rank, suit))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Suit {
    Hearts,
    Spades,
    Diamonds,
    Clubs,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PokerHandRank {
    HighCard,
    OnePair(u8),
    TwoPairs(u8, u8),
    ThreeOfAKind(u8),
    Straight,
    Flush,
    FullHouse(u8, u8),
    FourOfAKind(u8),
    StraightFlush,
    RoyalFlush,
}

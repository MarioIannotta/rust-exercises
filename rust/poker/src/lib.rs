#![allow(dead_code)]

use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Ordering;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    
    // assert_eq!(
    //     Hand::from_string("4H 6H 7H 8H 5H").get_score(), 
    //     HandScore::StraightFlush(Rank::Number(8))
    // );
    // assert_eq!(
    //     Hand::from_string("AH 3H 4H 2H 5H").get_score(), 
    //     HandScore::StraightFlush(Rank::Number(5))
    // );
    // assert_eq!(
    //     Hand::from_string("AH KH JH 10H QH").get_score(), 
    //     HandScore::StraightFlush(Rank::Ace)
    // );
    // assert_eq!(
    //     Hand::from_string("9H KH JH 10H QH").get_score(), 
    //     HandScore::StraightFlush(Rank::King)
    // );
    // assert_eq!(
    //     Hand::from_string("8H 2H 5H 10H QH").get_score(), 
    //     HandScore::Flush(Rank::Queen)
    // );
    // assert_eq!(
    //     Hand::from_string("QS QD QC 10H QH").get_score(), 
    //     HandScore::FourOfAKind(Rank::Queen)
    // );
    // assert_eq!(
    //     Hand::from_string("QS QD 10C 10H QH").get_score(), 
    //     HandScore::FullHouse(Rank::Queen)
    // );
    // assert_eq!(
    //     Hand::from_string("QS 10D 10C 10H QH").get_score(), 
    //     HandScore::FullHouse(Rank::Number(10))
    // );
    // assert_eq!(
    //     Hand::from_string("QS 10D 10C 10H KH").get_score(), 
    //     HandScore::ThreeOfAKind(Rank::Number(10))
    // );
    // assert_eq!(
    //     Hand::from_string("QS 10D 10C QH KH").get_score(), 
    //     HandScore::TwoPairs(Rank::Queen)
    // );
    // assert_eq!(
    //     Hand::from_string("QS 10D 10C AH KH").get_score(), 
    //     HandScore::OnePair(Rank::Number(10))
    // );
    // assert_eq!(
    //     Hand::from_string("QS 10D 4C AH KH").get_score(), 
    //     HandScore::HighCard(Rank::Ace)
    // );
    
    let _hands: Vec<Hand> = hands.iter().map(|hand_string| Hand::from_string(hand_string)).collect();
    let mut winning_hand_indexes: Vec<usize> = vec!(0);
    let mut winning_score = _hands[0].get_score();
    let mut i = 1;
    while i < _hands.len() {
        let current_score = _hands[i].get_score();
        if current_score == winning_score {
            winning_hand_indexes.push(i);
        } else if current_score > winning_score {
            winning_score = current_score;
            winning_hand_indexes = vec!(i);
        }
        i += 1;
    }

    // 1. iterate over the hands cards to get the highest.
    // eg: five of kind, straight flush --> five of kind
    // 1. iterate over the highest score of all the ends to get the best 
    let mut winning_hands: Vec<& str> = vec!();
    for winning_hand_indexe in winning_hand_indexes {
        winning_hands.push(hands[winning_hand_indexe]);
    }
    return winning_hands;
    
}

#[derive(Debug, Eq, Ord)]
enum HandScore {
    // associated value: the highest rank in the straight
    StraightFlush(Rank), 
    // associated value: (the four of a kind rank value, the kicker)
    FourOfAKind(Rank, Rank),
    // associated value: (the rank repeted three times, the pair rank)
    FullHouse(Rank, Rank),
    // associated value: the highest rank 
    Flush(Rank),
    // associated value: the highest rank 
    Straight(Rank),
    // associated value: (the triplet value, the kicker)
    ThreeOfAKind(Rank, Rank),
    // associated value: (the pairs value (sorted), the kicker)
    TwoPairs([Rank; 2], Rank),
    // associated value: the repeted rank value
    OnePair(Rank),
    // associated value: all the cards, sorted
    HighCard([Rank; 5]),
    // none
    None
    
}

impl PartialEq for HandScore {
    
    fn eq(&self, other: &HandScore) -> bool {
        if self.cardinal_value() == other.cardinal_value() {
            return match (self, other) {
                (HandScore::StraightFlush(rank_a), HandScore::StraightFlush(rank_b)) | 
                (HandScore::Flush(rank_a), HandScore::Flush(rank_b)) |
                (HandScore::Straight(rank_a), HandScore::Straight(rank_b)) |
                (HandScore::OnePair(rank_a), HandScore::OnePair(rank_b))
                => rank_a == rank_b,
                (HandScore::FullHouse(rank_a, kicker_a), HandScore::FullHouse(rank_b, kicker_b)) |
                (HandScore::FourOfAKind(rank_a, kicker_a), HandScore::FourOfAKind(rank_b, kicker_b)) |
                (HandScore::ThreeOfAKind(rank_a, kicker_a), HandScore::ThreeOfAKind(rank_b, kicker_b)) 
                => rank_a == rank_b && kicker_a == kicker_b,
                (HandScore::TwoPairs(pairs_a, kicker_a), HandScore::TwoPairs(pairs_b, kicker_b))
                => pairs_a == pairs_b && kicker_a == kicker_b,
                (HandScore::HighCard(ranks_a), HandScore::HighCard(ranks_b)) 
                => ranks_a == ranks_b,
                _ => { return false }
            }
        } else {
            return self.cardinal_value().eq(&other.cardinal_value());
        }
    }    
}

impl PartialOrd for HandScore {
    fn partial_cmp(&self, other: &HandScore) -> Option<Ordering> {
        if self.cardinal_value() == other.cardinal_value() {
            return match (self, other) {
                (HandScore::StraightFlush(rank_a), HandScore::StraightFlush(rank_b)) | 
                (HandScore::Flush(rank_a), HandScore::Flush(rank_b)) |
                (HandScore::Straight(rank_a), HandScore::Straight(rank_b)) |
                (HandScore::OnePair(rank_a), HandScore::OnePair(rank_b))
                => rank_a.partial_cmp(rank_b),
                (HandScore::FullHouse(rank_a, kicker_a), HandScore::FullHouse(rank_b, kicker_b)) |
                (HandScore::FourOfAKind(rank_a, kicker_a), HandScore::FourOfAKind(rank_b, kicker_b)) |
                (HandScore::ThreeOfAKind(rank_a, kicker_a), HandScore::ThreeOfAKind(rank_b, kicker_b)) 
                => {
                    if rank_a == rank_b {
                        return kicker_a.partial_cmp(kicker_b);
                    } else {
                        return rank_a.partial_cmp(rank_b);
                    }
                },
                (HandScore::TwoPairs(pairs_a, kicker_a), HandScore::TwoPairs(pairs_b, kicker_b)) => {
                    let ranks_a = pairs_a.to_owned();
                    let ranks_b = pairs_b.to_owned();
                    let same_pairs = ranks_a == ranks_b;
                    if same_pairs {
                        return kicker_a.partial_cmp(kicker_b);
                    } else {
                        if ranks_a[1] == ranks_b[1] {
                            // both hands have two pairs, with the same highest ranked pair,
                            return kicker_b.partial_cmp(kicker_a);
                        } else {
                            // both hands have two pairs, highest ranked pair wins
                            return ranks_a[1].partial_cmp(&ranks_b[1]);
                        }
                    }
                },
                (HandScore::HighCard(rank_a), HandScore::HighCard(rank_b)) => {
                    let mut i = 0;
                    while i < rank_a.len() {
                        if rank_a[i] < rank_b[i] {
                            return Option::Some(Ordering::Less);
                        } else if rank_a[i] > rank_b[i] {
                            return Option::Some(Ordering::Greater);
                        }
                        i += 1;
                    } 
                    return Option::None;
                }
                _ => {
                    return Option::None;
                }
            }
        } else {
            return self.cardinal_value().partial_cmp(&other.cardinal_value());
        }
    }
}

impl HandScore {
    
    fn cardinal_value(& self) -> u8 {
        return match self {
            HandScore::StraightFlush(_) => 9,
            HandScore::FourOfAKind(_, _) => 8,
            HandScore::FullHouse(_, _) => 7,
            HandScore::Flush(_) => 6,
            HandScore::Straight(_) => 5,
            HandScore::ThreeOfAKind(_, _) => 4,
            HandScore::TwoPairs(_, _) => 3,
            HandScore::OnePair(_) => 2,
            HandScore::HighCard(_) => 1,
            HandScore::None => 0,
        }
    }
}

#[derive(Debug)]
struct Hand {
    pub cards: Vec<Card>
}

impl Hand {
    fn from_string(string: & str) -> Hand {
        let cards: Vec<Card> = string.split(" ").map(|card_string| Card::from_string(card_string)).collect();
        assert_eq!(cards.len(), 5, "Unexpected hand size");
        return Hand{cards: cards};
    }
    
    fn get_score(& self) -> HandScore {
        let suits_distribution = self.get_suits_distribution();
        let ranks_distribution = self.get_rank_distribution();
        let highest_rank = self.highest_rank();
        if suits_distribution.len() == 1 {
            let straight_type = self.straight_type();
            if straight_type == StraightType::None {
                return HandScore::Flush(highest_rank);
            } else {
                return HandScore::StraightFlush(highest_rank);
            }
        } else if ranks_distribution.len() == 2 {
            let mut four_of_a_kind: Option<Rank> = None;
            let mut four_of_a_kind_kicker: Option<Rank> = None;
            let mut triplet: Option<Rank> = None;
            let mut pair: Option<Rank> = None;
            
            for (rank, occurrences) in ranks_distribution {
                if occurrences == 4 {
                    four_of_a_kind = Option::Some(rank);
                } else if occurrences == 3 {
                    triplet = Option::Some(rank);
                } else if occurrences == 1 {
                    four_of_a_kind_kicker = Option::Some(rank);
                } else if occurrences == 2 {
                    pair = Option::Some(rank);
                }
            }
            if let (Some(four_of_a_kind), Some(four_of_a_kind_kicker)) = (four_of_a_kind, four_of_a_kind_kicker) {
                return HandScore::FourOfAKind(four_of_a_kind, four_of_a_kind_kicker);
            }
            if let (Some(triplet), Some(pair)) = (triplet, pair) {
                return HandScore::FullHouse(triplet, pair);
            }
        } else if ranks_distribution.len() == 3 {
            let mut triplet: Option<Rank> = None;
            let mut kicker: Option<Rank> = None;
            let mut pairs: Vec<Rank> = Vec::new();
            for (rank, occurrences) in ranks_distribution {
                if occurrences == 3 {
                    triplet = Option::Some(rank);
                } else if occurrences == 2 {
                    pairs.push(rank);
                } else if occurrences == 1 {
                    if let Some(current_kicker) = kicker {
                        if rank.cardinal_value() > current_kicker.cardinal_value() {
                            kicker = Option::Some(rank);
                        }
                    } else {
                        kicker = Option::Some(rank);
                    }
                } 
            }
            if let (Some(triplet), Some(kicker)) = (triplet, kicker) {
                return HandScore::ThreeOfAKind(triplet, kicker);
            } else if pairs.len() == 2 {
                pairs.sort();
                return HandScore::TwoPairs([pairs[0], pairs[1]], kicker.expect("Kicker not found"))
            } else {
                return HandScore::None;
            }
        } else if ranks_distribution.len() == 4 {
            let mut pair: Option<Rank> = None;
            for (rank, occurrences) in ranks_distribution {
                if occurrences == 2 {
                    pair = Option::Some(rank);
                }
            }
            return HandScore::OnePair(pair.expect("Pair not found"));
        } else if ranks_distribution.len() == 5 {
            let straight_type = self.straight_type();
            if straight_type == StraightType::None {
                let mut ranks: Vec<Rank> = self.cards.iter().map(|x| x.rank).collect();
                ranks.sort();
                return HandScore::HighCard([ranks[0], ranks[1], ranks[2], ranks[3], ranks[4]]);
            } else {
                return HandScore::Straight(highest_rank);
            }
        }
        return HandScore::None;
    }
    
    fn get_suits_distribution(& self) -> HashMap<Suit, u8> {
        let suits: Vec<Suit> = self.cards.iter().map(|x| x.suit).collect();
        return suits.get_distribution();
    }
    
    fn get_rank_distribution(& self) -> HashMap<Rank, u8> {
        let ranks: Vec<Rank> = self.cards.iter().map(|x| x.rank).collect();
        return ranks.get_distribution();
    }
    
    fn straight_type(& self) -> StraightType {
        let ranks: Vec<Rank> = self.cards.iter().map(|x| x.rank).collect();
        let mut ranks_values: Vec<u8> = ranks.iter().map(|x| x.cardinal_value()).collect();
        ranks_values.sort();
        
        if ranks.contains(& Rank::Ace) {
            // if the ace is contained, we need to check both for low and high straights
            if ranks_values.is_consecutive() {
                return StraightType::High;
            } else {
                let mut potential_low_straight = ranks.to_vec();
                potential_low_straight.retain(|x| x != &Rank::Ace);
                let mut potential_low_straight_values: Vec<u8> = potential_low_straight.iter().map(|x| x.cardinal_value()).collect();
                let low_ace_cardinal_value = Rank::low_ace_cardinal_value();
                potential_low_straight_values.insert(0, low_ace_cardinal_value);
                potential_low_straight_values.sort();
                if potential_low_straight_values.is_consecutive() {
                    return StraightType::Low;
                } else {
                    return StraightType::None;
                }
            }
        } else {
            // if the ace is not contained, the straight is always "High"
            if ranks_values.is_consecutive() {
                return StraightType::High;
            } else {
                return StraightType::None;
            }
        }
    }
    
    fn highest_rank(& self) -> Rank {
        let straight_type = self.straight_type();
        
        return match straight_type {
            // if the straight is low, 5 is always the biggest rank
            StraightType::Low => Rank::Number(5),
            _ => {
                let mut ranks_cardinal_values: Vec<u8> = self.cards.iter().map(|x| x.rank.cardinal_value()).collect();
                ranks_cardinal_values.sort();
                let highest_rank_cardinal_value = ranks_cardinal_values.last().expect("Hand is empty");
                return Rank::from_cardinal_value(* highest_rank_cardinal_value);
            }   
        }
    }
}

// i'm pretty sure this is done somewhere in std:: but it was fun to do!
pub trait GroupableVector<K: Eq + Hash + Clone> {
    fn get_distribution(&self) -> HashMap<K, u8>;
}

impl<K: Eq + Hash + Clone> GroupableVector<K> for Vec<K> {
    fn get_distribution(&self) -> HashMap<K, u8> {
        let mut distribution: HashMap<K, u8> = HashMap::new();
        let copy = self.to_vec();
        for item in copy {
            let count = distribution.entry(item).or_insert(0);
            *count += 1;
        }
        return distribution;
    }
}

pub trait ConsecutiveVector {
    fn is_consecutive(& self) -> bool;
}

impl ConsecutiveVector for Vec<u8> {
    fn is_consecutive(& self) -> bool {
        let mut i = 0;
        let len = self.len();
        while i < len - 1 {
            if self[i] + 1 != self[i + 1] {
                return false;
            }
            i += 1;
        }
        return true;
    }
}

#[derive(Debug)]
struct Card {   
    pub suit: Suit,
    pub rank: Rank
}

impl Card {
    
    fn from_string(string: & str) -> Card {
        let mut rank_chars = string.chars();
        rank_chars.next_back();
        let rank_string = rank_chars.as_str();
        let suit_char = string.chars().last().expect("Cannot find suit char");
        return Card { 
            rank: Rank::from_string(&rank_string),
            suit: Suit::from_char(&suit_char)
        };
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
enum Suit {
    Spade,
    Club,
    Heart,
    Diamond
}

impl Suit {
    fn from_char(char: & char) -> Suit {
        return match char {
            'S' => Suit::Spade,
            'C' => Suit::Club,
            'H' => Suit::Heart,
            'D' => Suit::Diamond,
            _ => { panic!("Unexpected char {}", char); }
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone, Ord)]
enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8)
}

impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Rank) -> Option<Ordering> {
        return self.cardinal_value().partial_cmp(&other.cardinal_value());
    }
}

impl Rank {
    fn from_string(string: & str) -> Rank {
        return match string {
            "A" => Rank::Ace,
            "K" => Rank::King,
            "Q" => Rank::Queen,
            "J" => Rank::Jack,
            _ => { 
                let value: u8 = (* string).parse().unwrap();
                return Rank::Number(value);
            }
        }
    }
    
    fn low_ace_cardinal_value() -> u8 {
        return 1;
    }
    
    fn cardinal_value(& self) -> u8 {
        return match self {
            Rank::Ace => 14,
            Rank::King => 13,
            Rank::Queen => 12,
            Rank::Jack => 11,
            Rank::Number(number) => *number // from 2 to 10
        }
    }
    
    fn from_cardinal_value(cardinal_value: u8) -> Rank {
        return match cardinal_value {
            14 => Rank::Ace,
            13 => Rank::King,
            12 => Rank::Queen,
            11 => Rank::Jack,
            1 => Rank::Ace,
            value => { 
                let is_value_valid = value >= 2 && value <= 10;
                assert_eq!(is_value_valid, true, "Unexpected value: {}", value);
                Rank::Number(value) 
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum StraightType {
    // A, 2, 3, 4, 5
    Low,
    // any other straight where A is not = 1
    High,
    None
}
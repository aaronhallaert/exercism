use std::{cmp::Ordering, fmt};

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
///

#[derive(Eq, PartialEq, Debug)]
pub enum HandType {
    // high card
    StraightFlush(u8),
    // rank of quadruplet + high card
    FourOfAKind(u8, u8),
    // rank of triplet, rank of pair
    FullHouse(u8, u8),
    // high card
    Flush(u8),
    // highest card
    Straight(u8),
    // number of 3 + high card
    ThreeOfAKind(u8, u8),
    // pair numbers (2) + high card
    TwoPair(u8, u8, u8),
    // pair number + high card
    OnePair(u8, u8),
    // high card
    HighCard(u8),
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            HandType::StraightFlush(a) => match other {
                HandType::StraightFlush(b) => Some(a.cmp(b)),
                _ => Some(Ordering::Greater),
            },
            HandType::FourOfAKind(a, b) => match other {
                HandType::StraightFlush(_) => Some(Ordering::Less),
                HandType::FourOfAKind(c, d) => Some((a, b).cmp(&(c, d))),
                _ => Some(Ordering::Greater),
            },
            HandType::FullHouse(a, b) => match other {
                HandType::StraightFlush(_) | HandType::FourOfAKind(_, _) => Some(Ordering::Less),
                HandType::FullHouse(c, d) => Some((a, b).cmp(&(c, d))),
                _ => Some(Ordering::Greater),
            },
            HandType::Flush(a) => match other {
                HandType::StraightFlush(_)
                | HandType::FourOfAKind(_, _)
                | HandType::FullHouse(_, _) => Some(Ordering::Less),
                HandType::Flush(b) => Some(a.cmp(b)),
                _ => Some(Ordering::Greater),
            },
            HandType::Straight(a) => match other {
                HandType::StraightFlush(_)
                | HandType::FourOfAKind(_, _)
                | HandType::FullHouse(_, _)
                | HandType::Flush(_) => Some(Ordering::Less),
                HandType::Straight(b) => Some(a.cmp(b)),
                _ => Some(Ordering::Greater),
            },
            HandType::ThreeOfAKind(a, b) => match other {
                HandType::StraightFlush(_)
                | HandType::FourOfAKind(_, _)
                | HandType::FullHouse(_, _)
                | HandType::Flush(_)
                | HandType::Straight(_) => Some(Ordering::Less),
                HandType::ThreeOfAKind(c, d) => Some((a, b).cmp(&(c, d))),
                _ => Some(Ordering::Greater),
            },
            HandType::TwoPair(a, b, c) => match other {
                HandType::TwoPair(d, e, f) => Some((a, b, c).cmp(&(d, e, f))),
                HandType::OnePair(_, _) | HandType::HighCard(_) => Some(Ordering::Greater),
                _ => Some(Ordering::Less),
            },
            HandType::OnePair(a, b) => match other {
                HandType::OnePair(c, d) => Some((a, b).cmp(&(c, d))),
                HandType::HighCard(_) => Some(Ordering::Greater),
                _ => Some(Ordering::Less),
            },
            HandType::HighCard(a) => match other {
                HandType::HighCard(b) => Some(a.cmp(b)),
                _ => Some(Ordering::Less),
            },
        }
    }
}

impl fmt::Display for HandType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HandType::StraightFlush(a) => write!(f, "StraightFlush (top card: {})", a),
            HandType::FourOfAKind(a, b) => {
                write!(f, "FourOfAKind (4 cards: {}, top card: {})", a, b)
            }
            HandType::FullHouse(a, b) => write!(f, "FullHouse (3 cards: {}, 2 cards: {})", a, b),
            HandType::Flush(a) => write!(f, "Flush (high card: {})", a),
            HandType::Straight(a) => write!(f, "Straight (high card: {})", a),
            HandType::ThreeOfAKind(a, b) => {
                write!(f, "ThreeOfAKind (3 cards: {}, high card: {})", a, b)
            }
            HandType::TwoPair(a, b, c) => write!(
                f,
                "TwoPair (first pair: {}, second pair: {}, other: {})",
                a, b, c
            ),
            HandType::OnePair(a, b) => write!(f, "OnePair (pair: {}, top card: {})", a, b),
            HandType::HighCard(a) => write!(f, "HighCard (high: {})", a),
        }
    }
}

#[derive(PartialEq, Clone)]
enum CardSymbol {
    Club,
    Spade,
    Heart,
    Diamond,
}

impl fmt::Display for CardSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CardSymbol::Club => write!(f, "Clubs ♣️ "),
            CardSymbol::Spade => write!(f, "Spades ♠️ "),
            CardSymbol::Heart => write!(f, "Hearts ♥️ "),
            CardSymbol::Diamond => write!(f, "Diamonds ♦️ "),
        }
    }
}

pub struct PokerHand {
    pub cards: Vec<Card>,
    pub result: HandType,
    pub origin: String,
}

impl fmt::Display for PokerHand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Cards: {}", self.cards.len())?;
        for v in &self.cards {
            writeln!(f, "\t{}", v)?;
        }
        write!(f, "RESULT: {}", self.result)
    }
}

#[derive(Clone)]
pub struct Card {
    number: u8,
    symbol: CardSymbol,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} of {}", self.number, self.symbol)
    }
}

impl PokerHand {
    pub fn new(input_cards: &[&str]) -> PokerHand {
        let mut cards: Vec<Card> = Vec::new();

        for card in input_cards {
            let (num_string, symbol_char) = match card.len() {
                2 => (card[0..1].to_string(), card.chars().next_back().unwrap()),
                3 => (card[0..2].to_string(), card.chars().next_back().unwrap()),
                _ => panic!("Card should be 2 or 3 characters"),
            };

            let number = match &*num_string {
                "J" => 11,
                "Q" => 12,
                "K" => 13,
                "A" => 1,
                _ => (num_string.to_string()).parse::<u8>().unwrap_or(0),
            };

            let symbol: CardSymbol = match symbol_char {
                'C' => CardSymbol::Club,
                'S' => CardSymbol::Spade,
                'H' => CardSymbol::Heart,
                'D' => CardSymbol::Diamond,
                _ => CardSymbol::Heart,
            };

            cards.push(Card { number, symbol })
        }

        for v in &cards {
            println!("\t{}", v);
        }

        let result = Self::determine_hand(&cards);
        println!("{}", result);

        Self {
            cards,
            result,
            origin: input_cards.join(" "),
        }
    }

    fn determine_hand(cards: &[Card]) -> HandType {
        let symbols = cards.iter().map(|c| &c.symbol).collect::<Vec<_>>();
        let first_symbol = &symbols[0];
        let all_equal_symbols = symbols.iter().all(|symbol| symbol == first_symbol);

        let mut numbers = cards.iter().map(|c| c.number).collect::<Vec<_>>();

        // sort numbers descending
        numbers.sort_by(|a, b| b.cmp(a));

        let mut highest_number = numbers[0];

        let diff_numbers_ace_low = numbers
            .iter()
            .enumerate()
            .map(|(i, &number)| (number as i16 - highest_number as i16 + i as i16) as i16)
            .collect::<Vec<_>>();

        // replace all 1's with 14
        numbers = numbers
            .iter()
            .map(|n| if *n == 1 { 14_u8 } else { *n })
            .collect::<Vec<u8>>();

        numbers.sort_by(|a, b| b.cmp(a));
        highest_number = numbers[0];
        let second_highest_number = numbers[1];

        let diff_numbers_ace_high = numbers
            .iter()
            .enumerate()
            .map(|(i, &number)| (number as i16 - highest_number as i16 + i as i16) as i16)
            .collect::<Vec<_>>();

        let is_straight = diff_numbers_ace_high.iter().all(|&n| n == 0)
            || diff_numbers_ace_low.iter().all(|&n| n == 0);

        // all card symbols are same, sorted without interruption => StraightFlush,
        if all_equal_symbols && is_straight {
            match highest_number {
                14 => match second_highest_number {
                    13 => return HandType::StraightFlush(highest_number),
                    _ => return HandType::StraightFlush(second_highest_number),
                },
                _ => return HandType::StraightFlush(highest_number),
            }
        };

        // FOUR OF A KIND
        // 4 same numbers => FourOfAKind,
        let quadruplet_window_index = numbers
            .windows(4)
            .position(|w| w.iter().all(|&n| n == w[0]));

        match quadruplet_window_index {
            None => {}
            Some(quadruplet_window_index) => {
                let quadruplet_number = numbers.windows(4).nth(quadruplet_window_index).unwrap()[0];
                let other_number = numbers.into_iter().find(|&n| n != quadruplet_number);

                return HandType::FourOfAKind(quadruplet_number, other_number.unwrap());
            }
        }

        // FULL HOUSE
        // 3 same numbers + 2 same numbers => FullHouse
        let triplet_window = numbers.windows(3).find(|w| w.iter().all(|&n| n == w[0]));

        if let Some(triplet_window) = triplet_window {
            let triplet_number = triplet_window[0];

            let pair_window = numbers
                .windows(2)
                .find(|w| w.iter().all(|&n| n == w[0] && n != triplet_number));

            match pair_window {
                None => {}
                Some(pair_window) => {
                    let pair_number = pair_window[0];

                    return HandType::FullHouse(triplet_number, pair_number);
                }
            }
        }

        // FLUSH
        if symbols.iter().all(|s| *s == &CardSymbol::Heart)
            || symbols.iter().all(|s| *s == &CardSymbol::Diamond)
            || symbols.iter().all(|s| *s == &CardSymbol::Spade)
            || symbols.iter().all(|s| *s == &CardSymbol::Club)
        {
            return HandType::Flush(highest_number);
        };

        // STRAIGHT
        // sorted without interruption => Straight,
        if is_straight {
            match highest_number {
                14 => match second_highest_number {
                    13 => return HandType::Straight(highest_number),
                    _ => return HandType::Straight(second_highest_number),
                },
                _ => return HandType::Straight(highest_number),
            }
        };

        // THREE OF A KIND
        // 3 same numbers => ThreeOfAKind,
        let triplet_window_tok = numbers.windows(3).find(|w| w.iter().all(|&n| n == w[0]));
        match triplet_window_tok {
            None => {}
            Some(triplet_window_tok) => {
                let tok_number = triplet_window_tok[0];
                let others = numbers
                    .into_iter()
                    .filter(|&n| n != tok_number)
                    .collect::<Vec<_>>();
                let max_tok = others.iter().max().unwrap();

                return HandType::ThreeOfAKind(tok_number, *max_tok);
            }
        };

        // TWO PAIR
        let first_pair_window = numbers.windows(2).find(|w| w.iter().all(|&n| n == w[0]));
        if let Some(first_pair_window) = first_pair_window {
            let first_pair_number = first_pair_window[0];

            let after_first = numbers
                .iter()
                .filter(|&&n| n != first_pair_number)
                .collect::<Vec<_>>();

            let second_pair_window = after_first
                .windows(2)
                .find(|w| w.iter().all(|&n| n == w[0]));
            if let Some(second_pair_window) = second_pair_window {
                let second_pair_number = second_pair_window[0];

                let others = after_first
                    .into_iter()
                    .filter(|&n| n != second_pair_number)
                    .collect::<Vec<_>>();

                let max_one_pair = others.iter().max().unwrap();

                if first_pair_number > *second_pair_number {
                    return HandType::TwoPair(
                        first_pair_number,
                        *second_pair_number,
                        **max_one_pair,
                    );
                } else {
                    return HandType::TwoPair(
                        *second_pair_number,
                        first_pair_number,
                        **max_one_pair,
                    );
                }
            }
        }

        // ONE PAIR
        // 2 same numbers => OnePair,
        let one_pair_window = numbers.windows(2).find(|w| w.iter().all(|&n| n == w[0]));
        if let Some(one_pair_window) = one_pair_window {
            let one_pair_number = one_pair_window[0];

            let others = numbers
                .into_iter()
                .filter(|&n| n != one_pair_number)
                .collect::<Vec<_>>();

            let max_one_pair = others.iter().max().unwrap();

            return HandType::OnePair(one_pair_number, *max_one_pair);
        };

        // HIGH CARD
        // _ => High Card
        HandType::HighCard(highest_number)
    }
}

impl PartialOrd for PokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let result_cmp = self.result.partial_cmp(&other.result);
        if result_cmp == Some(Ordering::Equal) {
            let mut numbers_self = self.cards.iter().map(|c| c.number).collect::<Vec<u8>>();
            numbers_self.sort_by(|a, b| b.cmp(a));
            let mut numbers_other = other.cards.iter().map(|c| c.number).collect::<Vec<u8>>();
            numbers_other.sort_by(|a, b| b.cmp(a));

            Some(numbers_self.cmp(&numbers_other))
        } else {
            result_cmp
        }
    }
}
impl PartialEq for PokerHand {
    fn eq(&self, other: &Self) -> bool {
        self.result == other.result
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    // split hands on spaces
    let mut poker_hands: Vec<(PokerHand, &str)> = Vec::new();
    hands.iter().for_each(|hand_str| {
        let hand_arr = hand_str.split(' ').collect::<Vec<&str>>();
        poker_hands.push((PokerHand::new(&hand_arr), *hand_str));
    });

    poker_hands.sort_by(|(pk_a, _a), (pk_b, _b)| pk_b.partial_cmp(pk_a).unwrap());

    let (first_winning_hand, _) = &poker_hands[0];

    // Can't return these because poker_hands is borrowed here
    poker_hands
        .iter()
        .filter_map(|(poker_hand, origin)| {
            if poker_hand.partial_cmp(first_winning_hand) == Some(Ordering::Equal) {
                Some(*origin)
            } else {
                None
            }
        })
        .collect::<Vec<&str>>()
}

use poker::{HandType, PokerHand};

#[test]
fn test_single_pair() {
    let single_pair_hand = PokerHand::new(&["4S", "4H", "7H", "8D", "JC"]);
    println!("{}", single_pair_hand);
    assert_eq!(single_pair_hand.result, HandType::OnePair(4, 11))
}

#[test]
fn test_double_pair() {
    let hand = PokerHand::new(&["4S", "4H", "7H", "JC", "7C"]);
    assert_eq!(hand.result, HandType::TwoPair(7, 4, 11))
}

#[test]
fn test_flush() {
    let hand = PokerHand::new(&["1H", "2H", "JH", "9H", "8H"]);
    assert_eq!(hand.result, HandType::Flush(14))
}

#[test]
fn test_straight_flush() {
    let hand = PokerHand::new(&["QH", "10H", "JH", "9H", "8H"]);
    assert_eq!(hand.result, HandType::StraightFlush(12))
}

#[test]
fn test_straight() {
    let hand = PokerHand::new(&["QS", "10H", "JH", "9C", "8C"]);
    assert_eq!(hand.result, HandType::Straight(12))
}

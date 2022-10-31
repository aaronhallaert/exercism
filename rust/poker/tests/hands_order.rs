use poker::HandType;

#[test]
fn test_single_pair_better() {
    let better_hand = HandType::OnePair(3, 11);
    let hand = HandType::OnePair(1, 11);

    assert!(better_hand > hand)
}

#[test]
fn test_single_vs_two_pair() {
    let better_hand = HandType::TwoPair(1, 2, 11);
    let hand = HandType::OnePair(3, 11);

    assert!(better_hand > hand)
}

#[test]
fn test_straight_flush_vs_straight() {
    let better_hand = HandType::StraightFlush(11);
    let hand = HandType::Straight(9);

    assert!(better_hand > hand)
}

 //  left: `{"AS AC KS KC 6S"}`,
 // right: `{"2H 2D 2C 8H 5H"}`', tests/poker.rs:18:5
#[test]
fn test_two_pair_three_of_a_kind() {
    let better_hand = HandType::ThreeOfAKind(2, 8);
    let hand = HandType::TwoPair(14, 13, 6);

    assert!(better_hand > hand)
}

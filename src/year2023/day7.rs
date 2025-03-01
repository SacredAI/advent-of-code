use itertools::Itertools;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Card(u8);

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '2' => Card(2),
            '3' => Card(3),
            '4' => Card(4),
            '5' => Card(5),
            '6' => Card(6),
            '7' => Card(7),
            '8' => Card(8),
            '9' => Card(9),
            'T' => Card(10),
            'J' => Card(11),
            'Q' => Card(12),
            'K' => Card(13),
            'A' => Card(14),
            c => panic!("invalid card {c}"),
        }
    }
}

impl Card {
    fn from_j(value: char) -> Self {
        match value {
            'J' => Card(1),
            '2' => Card(2),
            '3' => Card(3),
            '4' => Card(4),
            '5' => Card(5),
            '6' => Card(6),
            '7' => Card(7),
            '8' => Card(8),
            '9' => Card(9),
            'T' => Card(10),
            'Q' => Card(12),
            'K' => Card(13),
            'A' => Card(14),
            c => panic!("invalid card {c}"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Kind {
    High,
    OnePair,
    TwoPair,
    ThreeOf,
    FullHouse,
    FourOf,
    FiveOf,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    kind: Kind,
    cards: Vec<Card>,
}

impl Hand {
    fn new(input: &str) -> Self {
        let cards: Vec<Card> = input.chars().take(5).map(Card::from).collect();
        let mut counts = [0; 15];
        for card in cards.clone() {
            counts[card.0 as usize] += 1;
        }
        counts.sort();
        let kind = match counts[11..15] {
            [0, 0, 0, 5] => Kind::FiveOf,
            [0, 0, 1, 4] => Kind::FourOf,
            [0, 0, 2, 3] => Kind::FullHouse,
            [0, 1, 1, 3] => Kind::ThreeOf,
            [0, 1, 2, 2] => Kind::TwoPair,
            [1, 1, 1, 2] => Kind::OnePair,
            _ => Kind::High,
        };
        Self { cards, kind }
    }

    fn new_j(input: &str) -> Self {
        let cards: Vec<Card> = input.chars().take(5).map(Card::from_j).collect();
        let mut counts = [0; 15];
        for card in cards.clone() {
            counts[card.0 as usize] += 1;
        }
        let jokers = counts[1];
        counts[1] = 0;
        counts.sort();
        counts[14] += jokers;
        let kind = match counts[11..15] {
            [0, 0, 0, 5] => Kind::FiveOf,
            [0, 0, 1, 4] => Kind::FourOf,
            [0, 0, 2, 3] => Kind::FullHouse,
            [0, 1, 1, 3] => Kind::ThreeOf,
            [0, 1, 2, 2] => Kind::TwoPair,
            [1, 1, 1, 2] => Kind::OnePair,
            _ => Kind::High,
        };
        Self { cards, kind }
    }
}

#[derive(Debug)]
pub struct Play {
    hand: Hand,
    bid: u32,
}

impl PartialEq for Play {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl Eq for Play {}

impl PartialOrd for Play {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Play {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand.cmp(&other.hand)
    }
}

impl Play {
    fn from_str(line: &str) -> Self {
        let (hand, bid) = line.split_once(' ').unwrap();
        let hand = Hand::new(hand);
        let bid = bid.parse().unwrap();
        Self { hand, bid }
    }
    fn from_str_j(line: &str) -> Self {
        let (hand, bid) = line.split_once(' ').unwrap();
        let hand = Hand::new_j(hand);
        let bid = bid.parse().unwrap();
        Self { hand, bid }
    }
}

pub fn parse(input: &str) -> &str {
    input
}

pub fn part1(input: &str) -> u32 {
    let mut plays: Vec<Play> = input.lines().map(Play::from_str).collect();
    plays.sort();
    plays
        .iter()
        .enumerate()
        .map(|(i, play)| play.bid * (i as u32 + 1))
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let mut plays: Vec<Play> = input.lines().map(Play::from_str_j).collect();
    plays.sort();
    plays
        .iter()
        .enumerate()
        .map(|(i, play)| play.bid * (i as u32 + 1))
        .sum()
}

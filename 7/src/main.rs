use env_logger;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::str::FromStr;
use std::usize;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    hand_type: HandType,
    cards: [Label; NUM_CARDS_IN_HAND],
}

impl Hand {
    fn with_joker_rule(self) -> Hand {
        let cards: [Label; NUM_CARDS_IN_HAND] = self
            .cards
            .iter()
            .cloned()
            .map(|label| match label {
                Label::Jack => Label::Joker,
                rest => rest,
            })
            .collect::<Vec<Label>>()
            .try_into()
            .unwrap();

        let hand_type = determine_hand_type(&cards).unwrap();

        Hand { hand_type, cards }
    }
}

impl FromStr for Hand {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards: [Label; NUM_CARDS_IN_HAND] = s
            .chars()
            .map(|c| c.try_into().map_err(|v| ParseHandError::ParseLabelError(v)))
            .collect::<Result<Vec<Label>, Self::Err>>()?
            .try_into()
            .map_err(|_| ParseHandError::NotFiveCards)?;

        let hand_type = determine_hand_type(&cards)?;

        Ok(Hand { hand_type, cards })
    }
}

fn determine_hand_type(cards: &[Label; NUM_CARDS_IN_HAND]) -> Result<HandType, ParseHandError> {
    let mut card_counts: HashMap<Label, usize> = HashMap::new();

    for card in cards {
        *card_counts.entry(*card).or_default() += 1;
    }

    if let Some(num_jokers) = card_counts.remove(&Label::Joker) {
        let (max_label, _): (&Label, &usize) = card_counts
            .iter()
            .max_by(|(_, a), (_, b)| a.cmp(&b))
            .unwrap_or_else(|| (&Label::Joker, &0));

        log::trace!("Max Label: {:?}", max_label);

        card_counts
            .entry(*max_label)
            .and_modify(|value| *value += num_jokers)
            .or_insert(num_jokers);
    }

    log::trace!("{:?}", card_counts);

    let mut counts: Vec<usize> = card_counts.values().cloned().collect();
    counts.sort();

    match counts[..] {
        [5] => Ok(HandType::FiveOfAKind),
        [1, 4] => Ok(HandType::FourOfAKind),
        [2, 3] => Ok(HandType::FullHouse),
        [1, 1, 3] => Ok(HandType::ThreeOfAKind),
        [1, 2, 2] => Ok(HandType::TwoPair),
        [1, 1, 1, 2] => Ok(HandType::OnePair),
        [1, 1, 1, 1, 1] => Ok(HandType::HighCard),
        _ => Err(ParseHandError::UnknownHandType),
    }
}

#[derive(Debug)]
enum ParseHandError {
    ParseLabelError(ParseLabelError),
    NotFiveCards,
    UnknownHandType,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Label {
    Joker,
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
    Ace,
}

#[derive(Debug)]
enum ParseLabelError {
    UnknownLabel(char),
}

impl TryFrom<char> for Label {
    type Error = ParseLabelError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '2' => Ok(Label::Two),
            '3' => Ok(Label::Three),
            '4' => Ok(Label::Four),
            '5' => Ok(Label::Five),
            '6' => Ok(Label::Six),
            '7' => Ok(Label::Seven),
            '8' => Ok(Label::Eight),
            '9' => Ok(Label::Nine),
            'T' => Ok(Label::Ten),
            'J' => Ok(Label::Jack),
            'Q' => Ok(Label::Queen),
            'K' => Ok(Label::King),
            'A' => Ok(Label::Ace),
            _ => Err(ParseLabelError::UnknownLabel(value)),
        }
    }
}

const NUM_CARDS_IN_HAND: usize = 5;

type Bid = u64;

fn main() {
    env_logger::init();

    let input = load_input();
    let lines: Vec<String> = input
        .split('\n')
        .map(|line| line.to_string())
        .filter(|line| line != &"")
        .collect();

    let mut hands: Vec<(Hand, Bid)> = lines
        .iter()
        .map(|s| {
            let mut split = s.split_whitespace();
            let hand = split.next().unwrap().parse().unwrap();
            let bid = split.next().unwrap().parse().unwrap();
            (hand, bid)
        })
        .collect();

    log::debug!("Hands: {:?}", hands);

    hands.sort_by(|a, b| a.0.cmp(&b.0));

    log::debug!("Ranked hands: {:#?}", hands);

    let total_winnings: Bid = calculate_total_winnings(&hands);
    println!("{}", total_winnings);

    let mut joker_hands: Vec<(Hand, Bid)> = hands
        .into_iter()
        .map(|(hand, bid)| (hand.with_joker_rule(), bid))
        .collect();

    joker_hands.sort_by(|a, b| a.0.cmp(&b.0));

    let joker_total_winnings = calculate_total_winnings(&joker_hands);
    println!("{}", joker_total_winnings);
}

fn calculate_total_winnings(sorted_hands: &[(Hand, Bid)]) -> Bid {
    sorted_hands
        .iter()
        .enumerate()
        .map(|(i, hand)| ((i + 1) as Bid, hand))
        .inspect(|a| log::trace!("{:?}", a))
        .map(|(rank, (_, bid))| bid * rank)
        .sum()
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

#[cfg(test)]
mod test {
    use crate::Hand;

    #[test]
    fn card_order() {
        assert!("33332".parse::<Hand>().unwrap() > "2AAAA".parse::<Hand>().unwrap());
        assert!("77888".parse::<Hand>().unwrap() > "77788".parse::<Hand>().unwrap());
    }
}

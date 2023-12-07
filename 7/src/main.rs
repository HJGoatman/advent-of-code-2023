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

    log::trace!("{:?}", card_counts);

    let mut counts: Vec<usize> = card_counts.values().cloned().collect();
    counts.sort();

    if counts == vec![5] {
        return Ok(HandType::FiveOfAKind);
    }

    if counts == vec![1, 4] {
        return Ok(HandType::FourOfAKind);
    }

    if counts == vec![2, 3] {
        return Ok(HandType::FullHouse);
    }

    if counts == vec![1, 1, 3] {
        return Ok(HandType::ThreeOfAKind);
    }

    if counts == vec![1, 2, 2] {
        return Ok(HandType::TwoPair);
    }

    if counts == vec![1, 1, 1, 2] {
        return Ok(HandType::OnePair);
    }

    if counts == vec![1, 1, 1, 1, 1] {
        return Ok(HandType::HighCard);
    }

    return Err(ParseHandError::UnknownHandType);
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

    let total_winnings: Bid = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| ((i + 1) as Bid, hand))
        .inspect(|a| log::trace!("{:?}", a))
        .map(|(rank, (_, bid))| bid * rank)
        .sum();

    println!("{}", total_winnings);
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

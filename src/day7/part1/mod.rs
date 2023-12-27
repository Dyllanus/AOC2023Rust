use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
#[derive(PartialEq, PartialOrd)]
enum DeckType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1
}

#[derive(PartialEq)]
struct Deck {
    cards : String,
    card_strengths : Vec<u32>,
    bet : u32,
    deck_type : DeckType
}

impl Deck {
    pub fn make_deck(s: &str) -> Deck {
        let split_s = s.trim().split(" ").collect::<Vec<&str>>();
        let cards = String::from(split_s[0]);
        let card_strengths = Deck::determine_card_strengths(&cards);
        let bet = split_s[1].parse::<u32>().unwrap();
        let deck_type = Deck::determine_deck_type(cards.clone());
        Deck{
            cards,card_strengths,bet,deck_type
        }
    }
    fn determine_card_strengths(cards : &String) -> Vec<u32> {
        let mut strengths : Vec<u32> = Vec::new();
        for char in cards.chars(){
            if char.is_numeric() {
                strengths.push(char.to_digit(10).unwrap());
            } else {
                let val_char: u32 = match char {
                    'T' => 10,
                    'J' => 11,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => 10
                };
                strengths.push(val_char);
            }
        }
        strengths
    }
    fn determine_deck_type(s:String) -> DeckType {
        //todo fix deze lelijke map meuk
        let mut test_map:HashMap<char, u32> = HashMap::new();
        for char in s.chars() {
            if test_map.contains_key(&char) {
                test_map.insert(char, *test_map.get(&char).unwrap()+1);
            }else{
                test_map.insert(char, 1);
            }
        }
        let max_vals = test_map.values().filter(|val| **val != 1).map(|val| *val).collect::<Vec<u32>>();
        if max_vals.is_empty() { return DeckType::HighCard }
        let max_val = max_vals.iter().max().unwrap();
        let sum_val = max_vals.iter().sum::<u32>();
        let deck_type : DeckType = match (max_val, sum_val) {
            (5, 5) => DeckType::FiveOfAKind,
            (4,4) => DeckType::FourOfAKind,
            (3,5) => DeckType::FullHouse,
            (3,3) => DeckType::ThreeOfAKind,
            (2,4) => DeckType::TwoPair,
            (2,2) => DeckType::OnePair,
            _ => DeckType::HighCard
        };
        return deck_type
    }

    pub fn is_deck_worse(&self, deck: &Deck) -> bool {
        return if self.deck_type > deck.deck_type {
            false
        } else if self.deck_type < deck.deck_type {
            true
        } else {
            for i in 0..self.card_strengths.len() {
                let c_strength_self = self.card_strengths[i];
                let c_strength_deck = deck.card_strengths[i];
                if c_strength_self > c_strength_deck {
                    return false;
                } else if c_strength_self < c_strength_deck {
                    return true;
                }
            }
            true
        }
    }

}

pub fn main(){
    let binding = fs::read_to_string("inputs/day7/input").expect("Should have been able to read the file");
    let binding = binding.split("\n");
    let mut decks : Vec<Deck> = Vec::new();
    for line in binding {
        decks.push(Deck::make_deck(line));
    }
    bubble_sort(&mut decks);
    let mut answer: u64 = 0;
    for i in 0..decks.len() {
        answer += ((i+1) as u64) * (decks[i].bet as u64);
    }
    println!("{answer}")
}


fn bubble_sort(decks : &mut Vec<Deck>) -> &Vec<Deck> {
    let length = decks.len();
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 1..length {
            let previous_element = &decks[i - 1];
            let current_element = &decks[i];

            if current_element.is_deck_worse(previous_element) {
                decks.swap(i - 1, i);
                swapped = true;
            }
        }
    }
    return decks;
}

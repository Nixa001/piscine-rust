extern crate rand;  
use rand::Rng; 

#[derive(Debug, PartialEq)]
pub enum Suit {
    Heart,   
    Diamond, 
    Spade,   
    Club,    
}
impl Suit {
    
    pub fn random() -> Suit {
        let mut rng = rand::thread_rng();
        match rng.gen_range(1..=4) {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            _ => Suit::Club,
        }
    }
    
    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,   
            2 => Suit::Diamond, 
            3 => Suit::Spade,   
            _ => Suit::Club,    
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Rank {
    Ace,        
    King,       
    Queen,      
    Jack,       
    Number(u8), 
}
impl Rank {
    pub fn random() -> Rank {
        let mut rng = rand::thread_rng();
        match rng.gen_range(1..=13) {
            1 => Rank::Ace,
            2..=10 => Rank::Number(rng.gen_range(2..=10)),
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => panic!("Erreur : valeur de rang invalide"),
        }
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            2..=10 => Rank::Number(value),
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => panic!("Erreur : valeur de rang invalide"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    let winner = Card {
        suit: Suit::Spade, 
        rank: Rank::Ace,   
    };
    *card == winner 
} 

 

// Main function for testing
// fn main() {
// 	let your_card = Card {
// 		rank: Rank::random(),
// 		suit: Suit::random(),
// 	};

// 	println!("Your card is {:?}", your_card);

// 	// Now if the card is an Ace of Spades print "You are the winner"
// 	if card_deck::winner_card(&your_card) {
// 		println!("You are the winner!");
// 	}
// }
// And its output

// $ cargo run
// Your card is Card { suit: Club, rank: Ace }
// $
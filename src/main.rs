use rand::{rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {
  cards: Vec<String>,
}

impl Deck {
  fn new() -> Self {
    let suits = [
      "Spades",
      "Clubs",
      // "Hearts",
      // "Diamonds",
    ];
    let values = [
      "Ace",
      "Two",
      "Three",
      // "Four",
      // "Five",
      // "Six",
      // "Seven",
      // "Eight",
      // "Nine",
      // "Ten",
      // "Jack",
      // "Queen",
      // "King",
    ];

    let mut cards = vec![];

    for suit in suits {
      for value in values {
        let card = format!("{} of {}", value, suit);
        cards.push(card)
      }
    }

    // let deck = Deck { cards: Vec::new() };
    // let deck = Deck { cards: vec![] };
    // let deck = Deck { cards: cards };
    // let deck = Deck { cards };
    // return deck;

    // Deck { cards }

    return Deck { cards };
  }

  fn shuffle(&mut self) {
    let mut salt = rng();
    self.cards.shuffle(&mut salt)
  }

  fn deal(&mut self, num_cards: usize) -> Vec<String> {
    return self.cards.split_off(self.cards.len() - num_cards);
  }
}

fn main() {
  // let rng = rand::rng();

  let mut deck = Deck::new();
  deck.shuffle();

  let hand = deck.deal(3);

  // println!("Here's your deck: {deck?}");
  println!("Here's your deck: {:#?}", deck);
  println!("Here's your hand: {:#?}", hand);
}

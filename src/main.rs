#[derive(Debug)]
struct Deck {
  cards: Vec<String>,
}

impl Deck {
  fn new() -> Self {
    let suits = [
      "Spades",
      "Clubs",
      "Hearts",
      "Diamonds",
    ];
    let values = [
      "Ace",
      "Two",
      "Three",
      "Four",
      "Five",
      "Six",
      "Seven",
      "Eight",
      "Nine",
      "Ten",
      "Jack",
      "Queen",
      "King",
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
    let deck = Deck { cards };

    return deck;
  }
}

fn main() {

  let deck = Deck::new();

  // println!("Here's your deck: {deck?}");
  println!("Here's your deck: {:#?}", deck);
}

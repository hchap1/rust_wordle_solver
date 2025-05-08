mod wordle;
mod application;

use wordle::WordleGame;

fn main() {
    let mut game: WordleGame = WordleGame::default();
    let _ = game.cli_information("hello", "bbbbb");
    let _ = game.cli_information("world", "bbybb");
    let _ = game.cli_information("crane", "bggbb");
    let _ = game.cli_information("cloth", "bbbbb");
    game.calculate().into_iter().for_each(|word| println!("{word}"));
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameSession {
    pub id: u32,
    pub p1: (String, u16),
    pub p2: (String, u16),
    pub nb_games: u16,
}

impl GameSession {
    pub fn new(id: u32, p1_name: String, p2_name: String, nb_games: u16) -> Box<GameSession> {
        let game_session = GameSession {
            id,
            p1: (p1_name, 0),
            p2: (p2_name, 0),
            nb_games,
        };
        Box::new(game_session)
    }
    pub fn read_winner(&self) -> (String, u16) {
        if self.p1.1 > self.p2.1 {
            self.p1.clone()
        } else if self.p1.1 < self.p2.1 {
            self.p2.clone()
        } else {
            ("Same score! tied".to_string(), self.p1.1)
        }
    }
    pub fn update_score(&mut self, user_name: String) {
        if self.p1.1 + self.p2.1 < self.nb_games {
            if self.p1.0 == user_name && self.p1.1 < (self.nb_games / 2 + 1) {
                self.p1.1 += 1;
            } else if self.p2.0 == user_name && self.p2.1 < (self.nb_games / 2 + 1) {
                self.p2.1 += 1;
            }
        }
    }
    pub fn delete(self) -> String {
        format!("game deleted: id -> {}", self.id)
    }
}

// fn main() {
//     let mut game = GameSession::new(0, String::from("Joao"), String::from("Susana"), 5);
//     println!("{:?}", game.read_winner());
//     // output : ("Same score! tied", 0)

//     game.update_score(String::from("Joao"));
//     game.update_score(String::from("Joao"));
//     game.update_score(String::from("Susana"));
//     game.update_score(String::from("Susana"));
//     println!("{:?}", game.read_winner());
//     // output : ("Same score! tied", 2)

//     game.update_score(String::from("Joao"));
//     // this one will not count because it already 5 games played, the nb_games
//     game.update_score(String::from("Susana"));

//     println!("{:?}", game.read_winner());
//     // output : ("Joao", 3)

//     println!("{:?}", game.delete());
//     // output : "game deleted: id -> 0"

//     // game.read_winner();
//     // this will give error
//     // because the game was dropped, no longer exists on the heap
// }
// And its output:

// $ cargo run
// ("Same score! tied", 0)
// ("Same score! tied", 2)
// ("Joao", 3)
// "game deleted: id -> 0"
// $

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
pub enum Player {
    White,
    Black,
}

#[derive(Debug)]
pub enum GameResult {
    Win(Player),
    Draw,
    Undecided,
}

pub trait Node<A> {
    fn get_possible_actions(&self) -> Vec<A>;
    fn take_action(&self, action: &A) -> Self;
    fn is_terminal(&self) -> bool;
    fn get_result(&self) -> GameResult;
    fn to_play(&self) -> Player;
    fn ply(&self) -> u32;
    fn bitboards(&self) -> (u64, u64);
}

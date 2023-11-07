use std::cmp::Ordering;

use arrayvec::ArrayVec;

use crate::board::Board;
use crate::card::Card;
use crate::level::Level;
use crate::position::{CardPile, Position};
use crate::solution::Solution;
use crate::square::Col;
use crate::yaku::process_yaku_chain;
use crate::{Frame, Money};

/// 探索中の状態。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {
    frame: Frame,
    money: Money,
    board: Board,
    solution: Solution,
}

impl State {
    pub fn new(frame: Frame, money: Money, board: Board, solution: Solution) -> Self {
        Self {
            frame,
            money,
            board,
            solution,
        }
    }

    /// (レベル開始時の状態, 初期化後の山札) を返す。
    pub fn new_initial(level: Level, money: Money, pile: CardPile) -> (Self, CardPile) {
        let (board, pile) = Position::with_level(level, pile).destructure();
        let state = State::new(0, money, board, Solution::new());
        (state, pile)
    }

    pub fn frame(&self) -> Frame {
        self.frame
    }

    pub fn money(&self) -> Money {
        self.money
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn solution(&self) -> &Solution {
        &self.solution
    }

    pub fn card_count(&self) -> u8 {
        self.board.card_count() as u8
    }

    /// 現在の状態を `ply` 手目 (0-based)、ツモを `card` としたときの近傍状態を列挙する。
    pub fn neighbors(&self, ply: usize, card: Card) -> ArrayVec<Self, 5> {
        let mut res = ArrayVec::<Self, 5>::new();

        for col in Col::all() {
            let Some((mut board, frame_put)) = self.board.put(col, card) else {
                continue;
            };
            let (frame_yaku, prize) = process_yaku_chain(&mut board);
            let state = Self::new(
                self.frame + frame_put + frame_yaku,
                self.money + prize,
                board,
                self.solution.add_move(ply, col),
            );
            res.push(state);
        }

        res
    }

    /// 指定した着手を行った後の状態を返す。着手は有効であることを仮定している。
    pub fn do_move(&self, ply: usize, card: Card, col: Col) -> Self {
        let (mut board, frame_put) = self.board.put(col, card).unwrap();
        let (frame_yaku, prize) = process_yaku_chain(&mut board);
        Self::new(
            self.frame + frame_put + frame_yaku,
            self.money + prize,
            board,
            self.solution.add_move(ply, col),
        )
    }

    /// 手順前後を無視して等しいかどうかを返す。
    pub fn eq_ignore_solution(&self, other: &Self) -> bool {
        (self.frame, self.money, &self.board) == (other.frame, other.money, &other.board)
    }

    /// 手順前後を無視して大小比較する。
    pub fn cmp_ignore_solution(&self, other: &Self) -> Ordering {
        (self.frame, self.money, &self.board).cmp(&(other.frame, other.money, &other.board))
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.board.fmt(f)?;
        writeln!(f, "frame={}", self.frame)?;
        writeln!(f, "money={}", self.money)?;
        writeln!(f, "solution={}", self.solution)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::card::*;

    use super::*;

    fn parse_board(s: impl AsRef<str>) -> Board {
        s.as_ref().parse().unwrap()
    }

    #[test]
    fn test_state_neighbors() {
        let board = parse_board(indoc! {"
            ..........
            ........C3
            ......C7H5
            ....CJH9D7
            ..C2S2DJS9
        "});

        let state = State::new(0, 0, board, Solution::new());
        let neighbors = state.neighbors(0, CARD_H2);

        assert_eq!(neighbors[0].frame(), 101 + 72 + 24 + 8);
        assert_eq!(neighbors[1].frame(), 85);
        assert_eq!(neighbors[2].frame(), 69);
        assert_eq!(neighbors[3].frame(), 53);
        assert_eq!(neighbors[4].frame(), 37);
    }
}

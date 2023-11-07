use std::borrow::Borrow;

use anyhow::{anyhow, ensure, Context as _};
use ascii::{AsciiStr, AsciiString};
use itertools::Itertools as _;

use crate::board::Board;
use crate::card::Card;
use crate::level::*;
use crate::square::*;

/// 山札。
///
/// 内部配列はゲーム内の山札配列と逆順であることに注意。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CardPile(Vec<Card>);

impl CardPile {
    fn new(inner: impl Into<Vec<Card>>) -> Self {
        Self(inner.into())
    }

    /// 初期山札を作る。`inner` 内に重複があってはならない。
    pub fn new_initial(inner: impl Borrow<[Card; 52]>) -> Self {
        Self::_new_initial(inner.borrow())
    }

    fn _new_initial(inner: &[Card; 52]) -> Self {
        {
            let uniq_count = inner.iter().unique().count();
            assert_eq!(uniq_count, 52, "52 枚のカード中に重複がある: {inner:?}");
        }

        Self::new(*inner)
    }

    /// 山札内のカードの枚数を返す。
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// 山札が空かどうかを返す。
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// 山札に 1 枚カードを戻す。
    pub fn push(&mut self, card: Card) {
        self.0.push(card);
    }

    /// 山札から 1 枚カードを取り出す。
    pub fn pop(&mut self) -> Option<Card> {
        self.0.pop()
    }

    /// `self[i]` と `self[j]` を swap する。
    fn swap(&mut self, i: usize, j: usize) {
        let tmp = self[i];
        self[i] = self[j];
        self[j] = tmp;
    }

    /// ゲーム内の山札配列メモリダンプ (例: "01 0A 3D ...") を山札としてパースする。
    ///
    /// 文字列内の ASCII 空白文字は無視される。
    pub fn parse_memory(s: impl AsRef<str>) -> anyhow::Result<Self> {
        Self::_parse_memory(s.as_ref())
    }

    fn _parse_memory(s: &str) -> anyhow::Result<Self> {
        let s = AsciiStr::from_ascii(s)
            .with_context(|| format!("無効な山札配列メモリダンプ: '{s}'"))?;
        let s: AsciiString = s.chars().filter(|ch| !ch.is_ascii_whitespace()).collect();
        ensure!(
            s.len() % 2 == 0,
            "空白除去後の山札配列メモリダンプの文字数が偶数でない: '{s}'"
        );

        Self::_parse_memory_helper(&s)
    }

    fn _parse_memory_helper(s: &AsciiStr) -> anyhow::Result<Self> {
        let mut inner = Vec::<Card>::with_capacity(s.len() / 2);

        for (i, chunk) in s.as_bytes().chunks_exact(2).enumerate() {
            let token = unsafe { std::str::from_utf8_unchecked(chunk) };
            let value = u8::from_str_radix(token, 16)
                .with_context(|| format!("山札配列メモリダンプ[{i}] のパースに失敗: '{token}'"))?;
            let card = Card::from_cadillac_value(value)
                .ok_or_else(|| anyhow!("山札配列メモリダンプ[{i}] の値が無効: 0x{value:02X}"))?;
            inner.push(card);
        }

        inner.reverse();

        Ok(Self::new(inner))
    }

    /// ゲーム内の山札配列メモリダンプ (例: "01 0A 3D ...") を初期山札としてパースする。
    ///
    /// 文字列内の ASCII 空白文字は無視される。
    pub fn parse_memory_initial(s: impl AsRef<str>) -> anyhow::Result<Self> {
        Self::_parse_memory_initial(s.as_ref())
    }

    fn _parse_memory_initial(s: &str) -> anyhow::Result<Self> {
        let s = AsciiStr::from_ascii(s)
            .with_context(|| format!("無効な初期山札配列メモリダンプ: '{s}'"))?;
        let s: AsciiString = s.chars().filter(|ch| !ch.is_ascii_whitespace()).collect();
        ensure!(
            s.len() == 2 * 52,
            "空白除去後の初期山札配列メモリダンプが 2 * 52 文字でない"
        );

        Self::_parse_memory_helper(&s)
    }

    /// 山札をゲーム内の山札配列メモリダンプとしてフォーマットする。
    pub fn display_memory(&self) -> CardPileDisplayMemory {
        CardPileDisplayMemory(self)
    }
}

impl std::ops::Index<usize> for CardPile {
    type Output = Card;

    /// 0-indexed で `idx` 番目に取り出されるカードへの不変参照を返す。
    fn index(&self, idx: usize) -> &Self::Output {
        let len = self.0.len();
        &self.0[len - 1 - idx]
    }
}

impl std::ops::IndexMut<usize> for CardPile {
    /// 0-indexed で `idx` 番目に取り出されるカードへの可変参照を返す。
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        let len = self.0.len();
        &mut self.0[len - 1 - idx]
    }
}

#[derive(Debug)]
pub struct CardPileDisplayMemory<'a>(&'a CardPile);

impl std::fmt::Display for CardPileDisplayMemory<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, card) in self.0 .0.iter().copied().rev().enumerate() {
            if i != 0 {
                f.write_str(" ")?;
            }
            write!(f, "{:02X}", card.to_cadillac_value())?;
        }

        Ok(())
    }
}

/// 局面。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Position {
    board: Board,
    pile: CardPile,
}

impl Position {
    pub fn new(board: Board, pile: CardPile) -> Self {
        Self { board, pile }
    }

    /// レベルと初期山札を与えて局面を初期化する。
    pub fn with_level(level: Level, pile: CardPile) -> Self {
        assert_eq!(pile.len(), 52, "初期山札は 52 枚でなければならない");

        match level {
            LEVEL_1 | LEVEL_2 | LEVEL_3 | LEVEL_4 => Self::with_level_1_to_4(pile),
            LEVEL_5 | LEVEL_6 | LEVEL_7 => Self::with_level_5_to_7(level, pile),
            LEVEL_8 | LEVEL_9 | LEVEL_10 => Self::with_level_8_to_10(level, pile),
        }
    }

    fn with_level_1_to_4(pile: CardPile) -> Self {
        // レベル 1..=4 では初期盤面は空。
        Self::new(Board::new(), pile)
    }

    fn with_level_5_to_7(level: Level, mut pile: CardPile) -> Self {
        // レベル 5 では 2 枚、レベル 6..=7 では 3 枚初期配置される。
        // これらの場合、初期配置で役ができることはない。

        let mut board = Board::new();

        board[SQ_A1] = Some(pile.pop().unwrap());
        board[SQ_E1] = Some(pile.pop().unwrap());

        if level >= LEVEL_6 {
            board[SQ_C1] = Some(pile.pop().unwrap());
        }

        Self::new(board, pile)
    }

    fn with_level_8_to_10(level: Level, mut pile: CardPile) -> Self {
        // レベル 8 では 5 枚、レベル 9..=10 では 7 枚初期配置される。
        // これらの場合、初期配置で役ができないような調整が行われる。

        /// 2 つのカードが「繋がっている」かどうかを返す。
        fn is_connected(card1: Card, card2: Card) -> bool {
            let connected_suit = card1.suit() == card2.suit();
            let connected_rank = card1.rank().prev() == card2.rank()
                || card1.rank() == card2.rank()
                || card1.rank().next() == card2.rank();
            connected_suit || connected_rank
        }

        /// `pile[i]` と `pile[j]` が繋がらなくなるよう調整する。
        fn disconnect(pile: &mut CardPile, i: usize, j: usize) {
            let card1 = pile[i];
            if !is_connected(card1, pile[j]) {
                return;
            }
            let mut k = 10;
            while is_connected(card1, pile[k]) {
                k += 1;
            }
            pile.swap(j, k);
        }

        // 原作準拠の disconnect 処理。なお、論理的には (4, 5) は disconnect する必要がない。
        for i in 0..5 {
            disconnect(&mut pile, i, i + 1);
        }
        if level >= LEVEL_9 {
            disconnect(&mut pile, 1, 5);
            disconnect(&mut pile, 3, 6);
        }

        // 初期配置処理。

        let mut board = Board::new();

        for col in Col::all() {
            let sq = Square::new(col, ROW_1);
            board[sq] = Some(pile.pop().unwrap());
        }
        if level >= LEVEL_9 {
            board[SQ_B2] = Some(pile.pop().unwrap());
            board[SQ_D2] = Some(pile.pop().unwrap());
        }

        Self::new(board, pile)
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn pile(&self) -> &CardPile {
        &self.pile
    }

    pub fn destructure(self) -> (Board, CardPile) {
        (self.board, self.pile)
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    fn parse_board(s: impl AsRef<str>) -> Board {
        s.as_ref().parse().unwrap()
    }

    fn parse_pile(s: impl AsRef<str>) -> CardPile {
        CardPile::parse_memory(s.as_ref()).unwrap()
    }

    fn parse_pile_initial(s: impl AsRef<str>) -> CardPile {
        CardPile::parse_memory_initial(s.as_ref()).unwrap()
    }

    /// 配牌固定の裏技を使った場合の調整前初期山札配列メモリダンプ (`$0505-$0538`)。
    ///
    /// 裏技: https://cah4e3.shedevr.org.ru/cheatsbase_c.php#237
    const CHEAT_PILE_MEMORY: &str = "1A 2B 3B 2A 0A 19 2C 3C 29 09 17 16 0D 1D 2D 3D 11 01 21 31 28 08 18 15 04 3A 1C 0C 14 05 37 1B 0B 32 33 35 36 23 06 13 03 22 07 12 02 34 27 26 25 24 23 22";

    fn cheat_pile() -> CardPile {
        parse_pile_initial(CHEAT_PILE_MEMORY)
    }

    #[test]
    fn test_card_pile_io() {
        assert_eq!(cheat_pile().display_memory().to_string(), CHEAT_PILE_MEMORY);

        {
            let mut s = CHEAT_PILE_MEMORY.to_owned();
            s.retain(|c| !c.is_ascii_whitespace());
            assert_eq!(parse_pile_initial(s), cheat_pile());
        }
    }

    #[test]
    fn test_position_with_level() {
        // レベル 1
        {
            let pos = Position::with_level(LEVEL_1, cheat_pile());
            assert_eq!(*pos.board(), Board::new());
            assert_eq!(*pos.pile(), cheat_pile());
        }

        // レベル 5
        {
            let board_expect = parse_board(indoc! {"
                ..........
                ..........
                ..........
                ..........
                CT......HJ
            "});
            let pile_expect = parse_pile(indoc! {"
                3B 2A 0A 19 2C 3C 29 09 17 16 0D 1D 2D 3D 11 01 21 31 28 08 18 15 04 3A 1C 0C 14 05 37 1B 0B 32 33 35 36 23 06 13 03 22 07 12 02 34 27 26 25 24 23 22
            "});
            let pos = Position::with_level(LEVEL_5, cheat_pile());
            assert_eq!(*pos.board(), board_expect);
            assert_eq!(*pos.pile(), pile_expect);
        }

        // レベル 6
        {
            let board_expect = parse_board(indoc! {"
                ..........
                ..........
                ..........
                ..........
                CT..DJ..HJ
            "});
            let pile_expect = parse_pile(indoc! {"
                2A 0A 19 2C 3C 29 09 17 16 0D 1D 2D 3D 11 01 21 31 28 08 18 15 04 3A 1C 0C 14 05 37 1B 0B 32 33 35 36 23 06 13 03 22 07 12 02 34 27 26 25 24 23 22
            "});
            let pos = Position::with_level(LEVEL_6, cheat_pile());
            assert_eq!(*pos.board(), board_expect);
            assert_eq!(*pos.pile(), pile_expect);
        }

        // レベル 8
        {
            let board_expect = parse_board(indoc! {"
                ..........
                ..........
                ..........
                ..........
                CTSKDJC7ST
            "});
            let pile_expect = parse_pile(indoc! {"
                16 2C 3C 29 09 2A 19 2B 1D 2D 3D 11 01 21 31 28 08 18 15 04 3A 1C 0C 14 05 37 1B 0B 32 33 35 36 23 06 13 03 22 07 12 02 34 27 26 25 24 23 22
            "});
            let pos = Position::with_level(LEVEL_8, cheat_pile());
            assert_eq!(*pos.board(), board_expect);
            assert_eq!(*pos.pile(), pile_expect);
        }

        // レベル 9
        {
            let board_expect = parse_board(indoc! {"
                ..........
                ..........
                ..........
                ..C6..HQ..
                CTSKDJC7ST
            "});
            let pile_expect = parse_pile(indoc! {"
                3C 29 09 2A 19 2B 1D 2D 3D 11 01 21 31 28 08 18 15 04 3A 1C 0C 14 05 37 1B 0B 32 33 35 36 23 06 13 03 22 07 12 02 34 27 26 25 24 23 22
            "});
            let pos = Position::with_level(LEVEL_9, cheat_pile());
            assert_eq!(*pos.board(), board_expect);
            assert_eq!(*pos.pile(), pile_expect);
        }
    }
}

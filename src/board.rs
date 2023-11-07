use anyhow::{ensure, Context as _};

use crate::card::Card;
use crate::square::{Col, Row, Square};
use crate::Frame;

/// 盤面。
#[repr(transparent)]
#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Board([Option<Card>; Col::NUM * Row::NUM]);

impl Board {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn card_count(&self) -> usize {
        self.0.iter().flatten().count()
    }

    pub fn col(&self, col: Col) -> [Option<Card>; 5] {
        *self.col_ref(col)
    }

    pub fn row(&self, row: Row) -> [Option<Card>; 5] {
        std::array::from_fn(|col| self.0[5 * col + row.to_index()])
    }

    fn col_ref(&self, col: Col) -> &[Option<Card>; 5] {
        unsafe {
            self.0[5 * col.to_index()..][..5]
                .try_into()
                .unwrap_unchecked()
        }
    }

    fn col_mut(&mut self, col: Col) -> &mut [Option<Card>; 5] {
        unsafe {
            (&mut self.0[5 * col.to_index()..][..5])
                .try_into()
                .unwrap_unchecked()
        }
    }

    /// 指定した列に指定したカードを落下させる。役判定/処理までは行わない。
    /// (結果の盤面, フレームコスト) を返す。
    pub fn put(&self, col: Col, card: Card) -> Option<(Self, Frame)> {
        let i = self.col_ref(col).iter().position(Option::is_none)?;

        let mut after = self.clone();
        after.col_mut(col)[i] = Some(card);

        let frame = 37 + 16 * (4 - i as Frame);

        Some((after, frame))
    }

    /// 空中にある全てのカードを落下完了させる。in-place 処理。
    /// フレームコストを返す。
    pub fn fall(&mut self) -> Frame {
        // 1 マスの落下に 8F かかるとする(概算)。
        fn fall_col(ary: &mut [Option<Card>; 5]) -> Frame {
            let mut frame = 0;
            let mut i = 0;
            for j in 0..5 {
                if let Some(card) = ary[j].take() {
                    ary[i] = Some(card);
                    frame += 8 * (j - i) as Frame;
                    i += 1;
                }
            }
            frame
        }

        let mut frame = 0;

        for col in Col::all() {
            let ary = self.col_mut(col);
            frame += fall_col(ary);
        }

        frame
    }

    fn parse(s: &str) -> anyhow::Result<Self> {
        let mut board = Board::new();

        let lines: Vec<_> = s.lines().collect();
        ensure!(lines.len() == 5, "盤面は 5 行でなければならない:\n{s}",);

        for (row, line) in std::iter::zip(Row::all().into_iter().rev(), lines) {
            let ary = Self::parse_row(line, row)?;
            for col in Col::all() {
                let sq = Square::new(col, row);
                board[sq] = ary[col.to_index()];
            }
        }

        Ok(board)
    }

    fn parse_row(line: &str, row: Row) -> anyhow::Result<[Option<Card>; 5]> {
        let chars: Vec<_> = line.chars().collect();
        ensure!(
            chars.len() == 10,
            "盤面の行は 10 文字でなければならない ({row:?}): '{line}'"
        );

        let mut ary = [None; 5];
        for (col, cs) in std::iter::zip(Col::all(), chars.chunks_exact(2)) {
            let s = String::from_iter(cs);
            ary[col.to_index()] = match s.as_str() {
                ".." => None,
                s => {
                    let card: Card = s.parse().with_context(|| {
                        let sq = Square::new(col, row);
                        format!("マス {sq:?} のカード文字列が無効: {s}")
                    })?;
                    Some(card)
                }
            };
        }

        Ok(ary)
    }
}

impl std::ops::Index<Square> for Board {
    type Output = Option<Card>;

    fn index(&self, sq: Square) -> &Self::Output {
        unsafe { self.0.get_unchecked(sq.to_index()) }
    }
}

impl std::ops::IndexMut<Square> for Board {
    fn index_mut(&mut self, sq: Square) -> &mut Self::Output {
        unsafe { self.0.get_unchecked_mut(sq.to_index()) }
    }
}

impl std::str::FromStr for Board {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in Row::all().into_iter().rev() {
            for card in self.row(row) {
                match card {
                    Some(card) => card.fmt(f)?,
                    None => f.write_str("..")?,
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::card::*;
    use crate::square::*;

    use super::*;

    fn parse_board(s: impl AsRef<str>) -> Board {
        s.as_ref().parse().unwrap()
    }

    #[test]
    fn test_board_io() {
        assert_eq!(parse_board(Board::new().to_string()), Board::new());

        let case = indoc! {"
            ....SA....
            S2..C9..HT
            CJCQS5DKDA
            D2D5HAH4C3
            S3CAH3D6D7
        "};
        let board = parse_board(case);
        assert_eq!(board.to_string(), case);
    }

    #[test]
    fn test_board_count() {
        assert_eq!(Board::new().card_count(), 0);

        let case = (
            indoc! {"
                ....SA....
                S2..C9..HT
                CJCQS5DKDA
                D2D5HAH4C3
                S3CAH3D6D7
            "},
            19,
        );
        let board = parse_board(case.0);
        assert_eq!(board.card_count(), case.1);
    }

    #[test]
    fn test_board_put() {
        let cases = [
            (
                indoc! {"
                    ..SA......
                    CAC2..C3C4
                    HAH2..H4H3
                    DKDT..DAD5
                    S5S7..S9SK
                "},
                COL_A,
                CARD_S8,
                Some(indoc! {"
                    S8SA......
                    CAC2..C3C4
                    HAH2..H4H3
                    DKDT..DAD5
                    S5S7..S9SK
                "}),
            ),
            (
                indoc! {"
                    ..SA......
                    CAC2..C3C4
                    HAH2..H4H3
                    DKDT..DAD5
                    S5S7..S9SK
                "},
                COL_B,
                CARD_S8,
                None,
            ),
            (
                indoc! {"
                    ..SA......
                    CAC2..C3C4
                    HAH2..H4H3
                    DKDT..DAD5
                    S5S7..S9SK
                "},
                COL_C,
                CARD_S8,
                Some(indoc! {"
                    ..SA......
                    CAC2..C3C4
                    HAH2..H4H3
                    DKDT..DAD5
                    S5S7S8S9SK
                "}),
            ),
        ];

        for (before, col, card, after) in cases {
            let before = parse_board(before);
            let after = after.map(parse_board);
            assert_eq!(before.put(col, card).map(|x| x.0), after);
        }
    }

    #[test]
    fn test_board_fall() {
        let case = (
            indoc! {"
                SA..S2S3..
                ..CA..C2C3
                HA..H2..H3
                ..DA..D2D3
                SK..SQ..SJ
            "},
            indoc! {"
                ..........
                ........C3
                SA..S2S3H3
                HACAH2C2D3
                SKDASQD2SJ
            "},
        );

        let mut board = parse_board(case.0);
        let after = parse_board(case.1);
        board.fall();
        assert_eq!(board, after);
    }
}

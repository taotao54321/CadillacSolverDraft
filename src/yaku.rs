//! 役検出および賞金計算。

use crate::board::Board;
use crate::card::{Card, CardRank, RANK_A, RANK_J, RANK_K, RANK_Q, RANK_T};
use crate::square::{Col, Row, Square};
use crate::{Frame, Money};

/// 与えられた盤面に対して役検出/処理を行い、(フレームコスト, 得られた賞金) を返す (連鎖処理あり)。
/// 盤面は全ての連鎖後の消去/落下処理が完了した後の状態となる。
///
/// 与えられた盤面は全てのカードが落下済みと仮定している。
///
/// この関数が 0 を返すことと役が一切成立しないことは同値。
pub fn process_yaku_chain(board: &mut Board) -> (Frame, Money) {
    let mut frame = 0;
    let mut prize = 0;

    loop {
        let (frame_cur, prize_cur) = process_yaku_step(board);
        if prize_cur == 0 {
            break;
        }
        frame += frame_cur;
        prize += prize_cur;
    }

    (frame, prize)
}

/// 与えられた盤面に対して役検出/処理を行い、(フレームコスト, 得られた賞金) を返す (連鎖処理なし)。
/// 盤面は 1 ステップ後の消去/落下処理が完了した後の状態となる。
///
/// 与えられた盤面は全てのカードが落下済みと仮定している。
///
/// この関数が 0 を返すことと役が一切成立しないことは同値。
fn process_yaku_step(board: &mut Board) -> (Frame, Money) {
    // 役検出と賞金加算処理は分離されている。挙動が非自明なので愚直にシミュレートする。

    let yaku_board = detect_yaku(board);

    let prize = calc_prize(board, &yaku_board);

    // 役成立演出に 72F かかるとする (概算)。
    let mut frame = 72;

    // カード 1 枚の消去に 8F かかるとする (概算)。
    for sq in yaku_board.squares_nonzero() {
        board[sq] = None;
        frame += 8;
    }
    frame += board.fall();

    (frame, prize)
}

/// 与えられた盤面に対して役検出を行い、`YakuBoard` を返す。
fn detect_yaku(board: &Board) -> YakuBoard {
    let mut yaku_board = YakuBoard::new();

    detect_straight(board, &mut yaku_board);
    detect_flush(board, &mut yaku_board);
    detect_n_of_kind(board, &mut yaku_board);

    yaku_board
}

/// 盤面の全ての行/列についてストレートを検出する。
fn detect_straight(board: &Board, yaku_board: &mut YakuBoard) {
    for row in Row::all() {
        detect_straight_row(board, yaku_board, row);
    }

    for col in Col::all() {
        detect_straight_col(board, yaku_board, col);
    }
}

/// 盤面の 1 つの行についてストレートを検出する。
fn detect_straight_row(board: &Board, yaku_board: &mut YakuBoard, row: Row) {
    // NOTE: 2 3 4 3 2 のようなケースでは前半の 2 3 4 のみが検出される。

    let ary = board.row(row);

    for col in Col::all().into_iter().take(3) {
        let len = straight_len(&ary[col.to_index()..]);
        if len >= 3 {
            for col in cols_from(col).take(len) {
                let sq = Square::new(col, row);
                yaku_board[sq].set_straight();
            }
            return;
        }
    }
}

/// 盤面の 1 つの列についてストレートを検出する。
fn detect_straight_col(board: &Board, yaku_board: &mut YakuBoard, col: Col) {
    // NOTE: 5 枚ストレートや 2 3 4 3 2 のようなケースはそもそも出現しえない。

    let ary = board.col(col);

    for row in Row::all().into_iter().take(3) {
        // 全てのカードは落下済みだから、列については先頭が None になった時点で打ち切ってよい。
        if ary[row.to_index()].is_none() {
            break;
        }
        let len = straight_len(&ary[row.to_index()..]);
        if len >= 3 {
            for row in rows_from(row).take(len) {
                let sq = Square::new(col, row);
                yaku_board[sq].set_straight();
            }
            return;
        }
    }
}

/// 与えられた手札スライスの先頭から成立しているストレートの枚数を返す。
fn straight_len(ary: &[Option<Card>]) -> usize {
    // 先頭にカードがないなら 0 枚。
    let Some(first) = ary[0] else {
        return 0;
    };

    // まず昇順ストレートの枚数を調べる。
    // 2 以上ならそれを返せばよい(降順ストレートは調べる必要がない)。
    let len_ascend = straight_len_ascend(first, &ary[1..]);
    if len_ascend >= 2 {
        return len_ascend;
    }

    straight_len_descend(first, &ary[1..])
}

/// 先頭と後続のカードが与えられたときに先頭から成立している昇順ストレートの枚数を返す。
fn straight_len_ascend(mut card: Card, card_nxts: &[Option<Card>]) -> usize {
    let mut len = 1;

    for card_nxt in card_nxts.iter().copied() {
        let Some(card_nxt) = card_nxt else {
            break;
        };
        if card.rank().next() != card_nxt.rank() {
            break;
        }
        card = card_nxt;
        len += 1;
    }

    len
}

/// 先頭と後続のカードが与えられたときに先頭から成立している降順ストレートの枚数を返す。
fn straight_len_descend(mut card: Card, card_nxts: &[Option<Card>]) -> usize {
    let mut len = 1;

    for card_nxt in card_nxts.iter().copied() {
        let Some(card_nxt) = card_nxt else {
            break;
        };
        if card.rank().prev() != card_nxt.rank() {
            break;
        }
        card = card_nxt;
        len += 1;
    }

    len
}

/// 盤面の全ての行/列についてフラッシュを検出する。
fn detect_flush(board: &Board, yaku_board: &mut YakuBoard) {
    for row in Row::all() {
        detect_flush_row(board, yaku_board, row);
    }

    for col in Col::all() {
        detect_flush_col(board, yaku_board, col);
    }
}

/// 盤面の 1 つの行についてフラッシュを検出する。
fn detect_flush_row(board: &Board, yaku_board: &mut YakuBoard, row: Row) {
    let ary = board.row(row);

    for col in Col::all().into_iter().take(3) {
        let len = flush_len(&ary[col.to_index()..]);
        if len >= 3 {
            for col in cols_from(col).take(len) {
                let sq = Square::new(col, row);
                yaku_board[sq].set_flush();
            }
            return;
        }
    }
}

/// 盤面の 1 つの列についてフラッシュを検出する。
fn detect_flush_col(board: &Board, yaku_board: &mut YakuBoard, col: Col) {
    let ary = board.col(col);

    for row in Row::all().into_iter().take(3) {
        // 全てのカードは落下済みだから、列については先頭が None になった時点で打ち切ってよい。
        if ary[row.to_index()].is_none() {
            break;
        }
        let len = flush_len(&ary[row.to_index()..]);
        if len >= 3 {
            for row in rows_from(row).take(len) {
                let sq = Square::new(col, row);
                yaku_board[sq].set_flush();
            }
            return;
        }
    }
}

/// 与えられた手札スライスの先頭から成立しているフラッシュの枚数を返す。
fn flush_len(ary: &[Option<Card>]) -> usize {
    // 先頭にカードがないなら 0 枚。
    let Some(first) = ary[0] else {
        return 0;
    };

    ary.iter()
        .position(|card| card.map_or(true, |card| card.suit() != first.suit()))
        .unwrap_or(ary.len())
}

/// 盤面の全ての行/列についてスリーカード/フォーカードを検出する。
fn detect_n_of_kind(board: &Board, yaku_board: &mut YakuBoard) {
    for row in Row::all() {
        detect_n_of_kind_row(board, yaku_board, row);
    }

    for col in Col::all() {
        detect_n_of_kind_col(board, yaku_board, col);
    }
}

/// 盤面の 1 つの行についてスリーカード/フォーカードを検出する。
fn detect_n_of_kind_row(board: &Board, yaku_board: &mut YakuBoard, row: Row) {
    let ary = board.row(row);

    for col in Col::all().into_iter().take(3) {
        let len = n_of_kind_len(&ary[col.to_index()..]);
        if len >= 3 {
            for col in cols_from(col).take(len) {
                let sq = Square::new(col, row);
                yaku_board[sq].set_n_of_kind();
            }
            return;
        }
    }
}

/// 盤面の 1 つの列についてスリーカード/フォーカードを検出する。
fn detect_n_of_kind_col(board: &Board, yaku_board: &mut YakuBoard, col: Col) {
    let ary = board.col(col);

    for row in Row::all().into_iter().take(3) {
        // 全てのカードは落下済みだから、列については先頭が None になった時点で打ち切ってよい。
        if ary[row.to_index()].is_none() {
            break;
        }
        let len = n_of_kind_len(&ary[row.to_index()..]);
        if len >= 3 {
            for row in rows_from(row).take(len) {
                let sq = Square::new(col, row);
                yaku_board[sq].set_n_of_kind();
            }
            return;
        }
    }
}

/// 与えられた手札スライスの先頭から成立している n of a kind の枚数を返す。
fn n_of_kind_len(ary: &[Option<Card>]) -> usize {
    // 先頭にカードがないなら 0 枚。
    let Some(first) = ary[0] else {
        return 0;
    };

    ary.iter()
        .position(|card| card.map_or(true, |card| card.rank() != first.rank()))
        .unwrap_or(ary.len())
}

// 賞金テーブル。
//
// ロイヤルフラッシュは単独で成立したとき 5 枚ストレートフラッシュ、5 枚ストレート、5 枚フラッシュと複合する。
// ストレートフラッシュは単独で成立したときストレートおよびフラッシュと複合する。
const PRIZE_ROYAL_FLUSH: Money = 200;
const PRIZE_STRAIGHT_FLUSH_5: Money = 120;
const PRIZE_STRAIGHT_FLUSH_4: Money = 40;
const PRIZE_STRAIGHT_FLUSH_3: Money = 39;
const PRIZE_STRAIGHT_5: Money = 50;
const PRIZE_STRAIGHT_4: Money = 20;
const PRIZE_STRAIGHT_3: Money = 10;
const PRIZE_FLUSH_5: Money = 30;
const PRIZE_FLUSH_4: Money = 10;
const PRIZE_FLUSH_3: Money = 1;
const PRIZE_FOUR_OF_KIND: Money = 100;
const PRIZE_THREE_OF_KIND: Money = 30;

fn prize_straight_flush(len: usize) -> Money {
    match len {
        3 => PRIZE_STRAIGHT_FLUSH_3,
        4 => PRIZE_STRAIGHT_FLUSH_4,
        5 => PRIZE_STRAIGHT_FLUSH_5,
        _ => unreachable!(),
    }
}

fn prize_straight(len: usize) -> Money {
    match len {
        3 => PRIZE_STRAIGHT_3,
        4 => PRIZE_STRAIGHT_4,
        5 => PRIZE_STRAIGHT_5,
        _ => unreachable!(),
    }
}

fn prize_flush(len: usize) -> Money {
    match len {
        3 => PRIZE_FLUSH_3,
        4 => PRIZE_FLUSH_4,
        5 => PRIZE_FLUSH_5,
        _ => unreachable!(),
    }
}

fn prize_n_of_kind(len: usize) -> Money {
    // NOTE: n of a kind フラグが 5 つ並ぶことはありえないと思うが、万一発生したらフォーカードと同等に扱う。

    match len {
        3 => PRIZE_THREE_OF_KIND,
        4..=5 => PRIZE_FOUR_OF_KIND,
        _ => unreachable!(),
    }
}

/// 役検出結果から賞金総額を求める。
fn calc_prize(board: &Board, yaku_board: &YakuBoard) -> Money {
    let mut prize = 0;

    prize += calc_prize_straight_flush(board, yaku_board);
    prize += calc_prize_straight(yaku_board);
    prize += calc_prize_flush(yaku_board);
    prize += calc_prize_n_of_kind(yaku_board);

    if prize == 0 {
        return 0;
    }

    // 役に絡んだカードの枚数により倍率が掛かる。
    prize *= match yaku_board.count_nonzero() {
        0..=5 => 1,
        6 => 2,
        7 => 3,
        8 => 5,
        9 => 6,
        10 => 7,
        11 => 8,
        _ => 10,
    };

    prize
}

/// 検出された全てのストレートフラッシュおよびロイヤルフラッシュの賞金総額を返す。
fn calc_prize_straight_flush(board: &Board, yaku_board: &YakuBoard) -> Money {
    let mut prize = 0;

    prize += Row::all()
        .into_iter()
        .map(|row| calc_prize_straight_flush_row(board, yaku_board, row))
        .sum::<Money>();

    prize += Col::all()
        .into_iter()
        .map(|col| calc_prize_straight_flush_col(yaku_board, col))
        .sum::<Money>();

    prize
}

/// 盤面の 1 つの行についてストレートフラッシュおよびロイヤルフラッシュの賞金総額を返す。
fn calc_prize_straight_flush_row(board: &Board, yaku_board: &YakuBoard, row: Row) -> Money {
    let ary = yaku_board.row(row);

    for col in Col::all().into_iter().take(3) {
        let len = yaku_len(&ary[col.to_index()..], YakuMask::has_straight_flush);
        if len >= 3 {
            let mut prize = prize_straight_flush(len);
            if len == 5 {
                let ranks = board.row(row).map(|card| card.unwrap().rank());
                if ranks_is_royal(&ranks) {
                    prize += PRIZE_ROYAL_FLUSH;
                }
            }
            return prize;
        }
    }

    0
}

/// 盤面の 1 つの列についてストレートフラッシュの賞金総額を返す。
fn calc_prize_straight_flush_col(yaku_board: &YakuBoard, col: Col) -> Money {
    // NOTE: 列については 5 枚ストレートフラッシュは出現しえない。よってロイヤルフラッシュもありえない。

    let ary = yaku_board.col(col);

    for row in Row::all().into_iter().take(3) {
        let len = yaku_len(&ary[row.to_index()..], YakuMask::has_straight_flush);
        if len >= 3 {
            return prize_straight_flush(len);
        }
    }

    0
}

/// ランク配列がロイヤルフラッシュの条件を満たすかどうかを返す。
fn ranks_is_royal(ranks: &[CardRank; 5]) -> bool {
    matches!(
        ranks,
        [RANK_T, RANK_J, RANK_Q, RANK_K, RANK_A] | [RANK_A, RANK_K, RANK_Q, RANK_J, RANK_T]
    )
}

/// 検出された全てのストレートの賞金総額を返す。
fn calc_prize_straight(yaku_board: &YakuBoard) -> Money {
    let mut prize = 0;

    prize += Row::all()
        .into_iter()
        .map(|row| calc_prize_straight_row(yaku_board, row))
        .sum::<Money>();

    prize += Col::all()
        .into_iter()
        .map(|col| calc_prize_straight_col(yaku_board, col))
        .sum::<Money>();

    prize
}

/// 盤面の 1 つの行についてストレートの賞金を返す。
fn calc_prize_straight_row(yaku_board: &YakuBoard, row: Row) -> Money {
    let ary = yaku_board.row(row);

    for col in Col::all().into_iter().take(3) {
        let len = yaku_len(&ary[col.to_index()..], YakuMask::has_straight);
        if len >= 3 {
            return prize_straight(len);
        }
    }

    0
}

/// 盤面の 1 つの列についてストレートの賞金を返す。
fn calc_prize_straight_col(yaku_board: &YakuBoard, col: Col) -> Money {
    let ary = yaku_board.col(col);

    for row in Row::all().into_iter().take(3) {
        let len = yaku_len(&ary[row.to_index()..], YakuMask::has_straight);
        if len >= 3 {
            return prize_straight(len);
        }
    }

    0
}

/// 検出された全てのフラッシュの賞金総額を返す。
fn calc_prize_flush(yaku_board: &YakuBoard) -> Money {
    let mut prize = 0;

    prize += Row::all()
        .into_iter()
        .map(|row| calc_prize_flush_row(yaku_board, row))
        .sum::<Money>();

    prize += Col::all()
        .into_iter()
        .map(|col| calc_prize_flush_col(yaku_board, col))
        .sum::<Money>();

    prize
}

/// 盤面の 1 つの行についてフラッシュの賞金を返す。
fn calc_prize_flush_row(yaku_board: &YakuBoard, row: Row) -> Money {
    let ary = yaku_board.row(row);

    for col in Col::all().into_iter().take(3) {
        let len = yaku_len(&ary[col.to_index()..], YakuMask::has_flush);
        if len >= 3 {
            return prize_flush(len);
        }
    }

    0
}

/// 盤面の 1 つの列についてフラッシュの賞金を返す。
fn calc_prize_flush_col(yaku_board: &YakuBoard, col: Col) -> Money {
    let ary = yaku_board.col(col);

    for row in Row::all().into_iter().take(3) {
        let len = yaku_len(&ary[row.to_index()..], YakuMask::has_flush);
        if len >= 3 {
            return prize_flush(len);
        }
    }

    0
}

/// 検出された全てのスリーカード/フォーカードの賞金総額を返す。
fn calc_prize_n_of_kind(yaku_board: &YakuBoard) -> Money {
    let mut prize = 0;

    prize += Row::all()
        .into_iter()
        .map(|row| calc_prize_n_of_kind_row(yaku_board, row))
        .sum::<Money>();

    prize += Col::all()
        .into_iter()
        .map(|col| calc_prize_n_of_kind_col(yaku_board, col))
        .sum::<Money>();

    prize
}

/// 盤面の 1 つの行についてスリーカード/フォーカードの賞金を返す。
fn calc_prize_n_of_kind_row(yaku_board: &YakuBoard, row: Row) -> Money {
    let ary = yaku_board.row(row);

    for col in Col::all().into_iter().take(3) {
        let len = yaku_len(&ary[col.to_index()..], YakuMask::has_n_of_kind);
        if len >= 3 {
            return prize_n_of_kind(len);
        }
    }

    0
}

/// 盤面の 1 つの列についてスリーカード/フォーカードの賞金を返す。
fn calc_prize_n_of_kind_col(yaku_board: &YakuBoard, col: Col) -> Money {
    let ary = yaku_board.col(col);

    for row in Row::all().into_iter().take(3) {
        let len = yaku_len(&ary[row.to_index()..], YakuMask::has_n_of_kind);
        if len >= 3 {
            return prize_n_of_kind(len);
        }
    }

    0
}

/// 与えられた役検出結果スライスの先頭から条件を満たすものの個数を返す。
fn yaku_len(ary: &[YakuMask], cond: impl Fn(YakuMask) -> bool) -> usize {
    ary.iter()
        .copied()
        .position(|mask| !cond(mask))
        .unwrap_or(ary.len())
}

/// `col` 以降の全ての列を列挙する。
fn cols_from(col: Col) -> impl Iterator<Item = Col> {
    std::iter::successors(Some(col), |col| col.next())
}

/// `row` 以降の全ての行を列挙する。
fn rows_from(row: Row) -> impl Iterator<Item = Row> {
    std::iter::successors(Some(row), |row| row.next())
}

/// 役検出結果を要素とする盤面。
#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct YakuBoard([YakuMask; Col::NUM * Row::NUM]);

impl YakuBoard {
    fn new() -> Self {
        Self::default()
    }

    fn col(&self, col: Col) -> [YakuMask; 5] {
        unsafe {
            self.0[5 * col.to_index()..][..5]
                .try_into()
                .unwrap_unchecked()
        }
    }

    fn row(&self, row: Row) -> [YakuMask; 5] {
        std::array::from_fn(|col| self.0[5 * col + row.to_index()])
    }

    fn count_nonzero(&self) -> usize {
        self.0.iter().filter(|mask| !mask.is_zero()).count()
    }

    fn squares_nonzero(&self) -> impl Iterator<Item = Square> + '_ {
        Square::all().into_iter().filter(|&sq| !self[sq].is_zero())
    }
}

impl std::ops::Index<Square> for YakuBoard {
    type Output = YakuMask;

    fn index(&self, sq: Square) -> &Self::Output {
        unsafe { self.0.get_unchecked(sq.to_index()) }
    }
}

impl std::ops::IndexMut<Square> for YakuBoard {
    fn index_mut(&mut self, sq: Square) -> &mut Self::Output {
        unsafe { self.0.get_unchecked_mut(sq.to_index()) }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct YakuMask(u8);

impl YakuMask {
    const BIT_STRAIGHT: u8 = 1 << 0;
    const BIT_FLUSH: u8 = 1 << 1;
    const BIT_N_OF_KIND: u8 = 1 << 2;

    #[allow(dead_code)]
    fn new() -> Self {
        Self::default()
    }

    fn is_zero(self) -> bool {
        self.0 == 0
    }

    fn has_straight(self) -> bool {
        (self.0 & Self::BIT_STRAIGHT) != 0
    }

    fn set_straight(&mut self) {
        self.0 |= Self::BIT_STRAIGHT;
    }

    fn has_flush(self) -> bool {
        (self.0 & Self::BIT_FLUSH) != 0
    }

    fn set_flush(&mut self) {
        self.0 |= Self::BIT_FLUSH;
    }

    fn has_n_of_kind(self) -> bool {
        (self.0 & Self::BIT_N_OF_KIND) != 0
    }

    fn set_n_of_kind(&mut self) {
        self.0 |= Self::BIT_N_OF_KIND
    }

    fn has_straight_flush(self) -> bool {
        self.has_straight() && self.has_flush()
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use indoc::indoc;

    use super::*;

    fn parse_board(s: impl AsRef<str>) -> Board {
        s.as_ref().parse().unwrap()
    }

    fn yaku_step(board: impl Borrow<Board>) -> (Board, Money) {
        let mut after = board.borrow().clone();
        let (_frame, prize) = process_yaku_step(&mut after);
        (after, prize)
    }

    #[test]
    fn test_process_yaku_step() {
        assert_eq!(yaku_step(Board::new()), (Board::new(), 0));

        let cases = [
            (
                // 昇順ロイヤルフラッシュ
                indoc! {"
                    ..........
                    ..........
                    ..........
                    CAH3..H7C9
                    STSJSQSKSA
                "},
                indoc! {"
                    ..........
                    ..........
                    ..........
                    ..........
                    CAH3..H7C9
                "},
                PRIZE_ROYAL_FLUSH + PRIZE_STRAIGHT_FLUSH_5 + PRIZE_STRAIGHT_5 + PRIZE_FLUSH_5,
            ),
            (
                // 降順ロイヤルフラッシュ
                indoc! {"
                    ..........
                    ..........
                    ..........
                    CAH3..H7C9
                    SASKSQSJST
                "},
                indoc! {"
                    ..........
                    ..........
                    ..........
                    ..........
                    CAH3..H7C9
                "},
                PRIZE_ROYAL_FLUSH + PRIZE_STRAIGHT_FLUSH_5 + PRIZE_STRAIGHT_5 + PRIZE_FLUSH_5,
            ),
            (
                // 昇順 5 枚ストレートフラッシュ (wrap あり)
                indoc! {"
                    ..........
                    ..........
                    ..........
                    CAH3..H7C9
                    SQSKSAS2S3
                "},
                indoc! {"
                    ..........
                    ..........
                    ..........
                    ..........
                    CAH3..H7C9
                "},
                PRIZE_STRAIGHT_FLUSH_5 + PRIZE_STRAIGHT_5 + PRIZE_FLUSH_5,
            ),
            (
                // 降順 5 枚ストレートフラッシュ (wrap あり)
                indoc! {"
                    ..........
                    ..........
                    ..........
                    CAH3..H7C9
                    S3S2SASKSQ
                "},
                indoc! {"
                    ..........
                    ..........
                    ..........
                    ..........
                    CAH3..H7C9
                "},
                PRIZE_STRAIGHT_FLUSH_5 + PRIZE_STRAIGHT_5 + PRIZE_FLUSH_5,
            ),
            (
                // 昇順 4 枚ストレートフラッシュ (wrap あり)
                indoc! {"
                    ..........
                    ..........
                    ..........
                    CAH3..H7..
                    SQSKSAS2..
                "},
                indoc! {"
                    ..........
                    ..........
                    ..........
                    ..........
                    CAH3..H7..
                "},
                PRIZE_STRAIGHT_FLUSH_4 + PRIZE_STRAIGHT_4 + PRIZE_FLUSH_4,
            ),
            (
                // 降順 4 枚ストレートフラッシュ (wrap あり)
                indoc! {"
                    ..........
                    ..........
                    ..........
                    CAH3..H7..
                    S2SASKSQ..
                "},
                indoc! {"
                    ..........
                    ..........
                    ..........
                    ..........
                    CAH3..H7..
                "},
                PRIZE_STRAIGHT_FLUSH_4 + PRIZE_STRAIGHT_4 + PRIZE_FLUSH_4,
            ),
            (
                // 昇順 3 枚ストレートフラッシュ (wrap あり)
                indoc! {"
                    ..........
                    ..........
                    ..........
                    ....H3..H7
                    ....SKSAS2
                "},
                indoc! {"
                    ..........
                    ..........
                    ..........
                    ..........
                    ....H3..H7
                "},
                PRIZE_STRAIGHT_FLUSH_3 + PRIZE_STRAIGHT_3 + PRIZE_FLUSH_3,
            ),
            (
                // 降順 3 枚ストレートフラッシュ (wrap あり)
                indoc! {"
                    ..........
                    ..........
                    ..........
                    ....H3..H7
                    ....S2SASK
                "},
                indoc! {"
                    ..........
                    ..........
                    ..........
                    ..........
                    ....H3..H7
                "},
                PRIZE_STRAIGHT_FLUSH_3 + PRIZE_STRAIGHT_3 + PRIZE_FLUSH_3,
            ),
            (
                // 昇順 5 枚ストレート (wrap あり)
                indoc! {"
                    ..........
                    ..........
                    ..........
                    CAH3..H7C9
                    SQCKHAD2S3
                "},
                indoc! {"
                    ..........
                    ..........
                    ..........
                    ..........
                    CAH3..H7C9
                "},
                PRIZE_STRAIGHT_5,
            ),
            (
                // 降順 5 枚ストレート (wrap あり)
                indoc! {"
                    ..........
                    ..........
                    ..........
                    CAH3..H7C9
                    S3C2HADKSQ
                "},
                indoc! {"
                    ..........
                    ..........
                    ..........
                    ..........
                    CAH3..H7C9
                "},
                PRIZE_STRAIGHT_5,
            ),
            (
                // 昇順 4 枚ストレート (wrap あり)
                indoc! {"
                    ..........
                    ..........
                    ..........
                    CAH3..H7..
                    SQCKHAD2..
                "},
                indoc! {"
                    ..........
                    ..........
                    ..........
                    ..........
                    CAH3..H7..
                "},
                PRIZE_STRAIGHT_4,
            ),
            (
                // 降順 4 枚ストレート (wrap あり)
                indoc! {"
                    ..........
                    ..........
                    ..........
                    CAH3..H7..
                    S2CAHKDQ..
                "},
                indoc! {"
                    ..........
                    ..........
                    ..........
                    ..........
                    CAH3..H7..
                "},
                PRIZE_STRAIGHT_4,
            ),
            (
                // 昇順 3 枚ストレート (wrap あり)
                indoc! {"
                    ..........
                    ..........
                    ..........
                    ....H3..H7
                    ....SKCAH2
                "},
                indoc! {"
                    ..........
                    ..........
                    ..........
                    ..........
                    ....H3..H7
                "},
                PRIZE_STRAIGHT_3,
            ),
            (
                // 降順 3 枚ストレート (wrap あり)
                indoc! {"
                    ..........
                    ..........
                    ..........
                    ....H3..H7
                    ....S2CAHK
                "},
                indoc! {"
                    ..........
                    ..........
                    ..........
                    ..........
                    ....H3..H7
                "},
                PRIZE_STRAIGHT_3,
            ),
            (
                // 5 枚フラッシュ
                indoc! {"
                    ..........
                    ..........
                    ..........
                    CAH3..H7C9
                    SAS3S5S7S9
                "},
                indoc! {"
                    ..........
                    ..........
                    ..........
                    ..........
                    CAH3..H7C9
                "},
                PRIZE_FLUSH_5,
            ),
            (
                // 4 枚フラッシュ
                indoc! {"
                    ..........
                    ..........
                    ..........
                    CAH3..H7..
                    SAS3S5S7..
                "},
                indoc! {"
                    ..........
                    ..........
                    ..........
                    ..........
                    CAH3..H7..
                "},
                PRIZE_FLUSH_4,
            ),
            (
                // 3 枚フラッシュ
                indoc! {"
                    ..........
                    ..........
                    ..........
                    ....H3..H7
                    ....SAS3S5
                "},
                indoc! {"
                    ..........
                    ..........
                    ..........
                    ..........
                    ....H3..H7
                "},
                PRIZE_FLUSH_3,
            ),
            (
                // フォーカード
                indoc! {"
                    ..........
                    ..........
                    ..........
                    CAH3..H7..
                    SACAHADA..
                "},
                indoc! {"
                    ..........
                    ..........
                    ..........
                    ..........
                    CAH3..H7..
                "},
                PRIZE_FOUR_OF_KIND,
            ),
            (
                // スリーカード
                indoc! {"
                    ..........
                    ..........
                    ..........
                    ....H3..H7
                    ....SACAHA
                "},
                indoc! {"
                    ..........
                    ..........
                    ..........
                    ..........
                    ....H3..H7
                "},
                PRIZE_THREE_OF_KIND,
            ),
        ];

        for (before, after, prize) in cases {
            let before = parse_board(before);
            let after = parse_board(after);
            assert_eq!(yaku_step(before), (after, prize));
        }
    }
}

//! 盤面のマス関連。
//!
//! マスの表記はチェス風にする:
//!
//! ```text
//! 5 .....
//! 4 .....
//! 3 .....
//! 2 .....
//! 1 .....
//!   ABCDE
//! ```
//!
//! マスの内部値は column-major で割り当てる。

use std::fmt::Write as _;

use crate::macros::assert_unchecked;

/// 盤面の列。
#[repr(u8)]
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub enum Col {
    ColA = 1,
    ColB,
    ColC,
    ColD,
    ColE,
}

pub const COL_A: Col = Col::ColA;
pub const COL_B: Col = Col::ColB;
pub const COL_C: Col = Col::ColC;
pub const COL_D: Col = Col::ColD;
pub const COL_E: Col = Col::ColE;

impl Col {
    pub const NUM: usize = 5;

    pub const MIN_VALUE: u8 = 1;
    pub const MAX_VALUE: u8 = 5;

    pub const fn from_inner(inner: u8) -> Option<Self> {
        if Self::is_valid(inner) {
            Some(unsafe { Self::from_inner_unchecked(inner) })
        } else {
            None
        }
    }

    /// # Safety
    ///
    /// `inner` は有効値でなければならない。
    pub const unsafe fn from_inner_unchecked(inner: u8) -> Self {
        assert_unchecked!(Self::is_valid(inner));

        std::mem::transmute(inner)
    }

    pub const fn to_inner(self) -> u8 {
        self as u8
    }

    pub const fn to_index(self) -> usize {
        (self.to_inner() - Self::MIN_VALUE) as usize
    }

    pub const fn next(self) -> Option<Self> {
        match self {
            COL_A => Some(COL_B),
            COL_B => Some(COL_C),
            COL_C => Some(COL_D),
            COL_D => Some(COL_E),
            COL_E => None,
        }
    }

    pub const fn prev(self) -> Option<Self> {
        match self {
            COL_A => None,
            COL_B => Some(COL_A),
            COL_C => Some(COL_B),
            COL_D => Some(COL_C),
            COL_E => Some(COL_D),
        }
    }

    pub const fn all() -> [Self; Self::NUM] {
        [COL_A, COL_B, COL_C, COL_D, COL_E]
    }

    const fn is_valid(inner: u8) -> bool {
        matches!(inner, Self::MIN_VALUE..=Self::MAX_VALUE)
    }
}

impl std::fmt::Debug for Col {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            COL_A => "COL_A",
            COL_B => "COL_B",
            COL_C => "COL_C",
            COL_D => "COL_D",
            COL_E => "COL_E",
        };
        f.write_str(s)
    }
}

impl std::fmt::Display for Col {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match *self {
            COL_A => 'A',
            COL_B => 'B',
            COL_C => 'C',
            COL_D => 'D',
            COL_E => 'E',
        };
        f.write_char(c)
    }
}

/// 盤面の行。
#[repr(u8)]
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub enum Row {
    Row1 = 1,
    Row2,
    Row3,
    Row4,
    Row5,
}

pub const ROW_1: Row = Row::Row1;
pub const ROW_2: Row = Row::Row2;
pub const ROW_3: Row = Row::Row3;
pub const ROW_4: Row = Row::Row4;
pub const ROW_5: Row = Row::Row5;

impl Row {
    pub const NUM: usize = 5;

    pub const MIN_VALUE: u8 = 1;
    pub const MAX_VALUE: u8 = 5;

    pub const fn from_inner(inner: u8) -> Option<Self> {
        if Self::is_valid(inner) {
            Some(unsafe { Self::from_inner_unchecked(inner) })
        } else {
            None
        }
    }

    /// # Safety
    ///
    /// `inner` は有効値でなければならない。
    pub const unsafe fn from_inner_unchecked(inner: u8) -> Self {
        assert_unchecked!(Self::is_valid(inner));

        std::mem::transmute(inner)
    }

    pub const fn to_inner(self) -> u8 {
        self as u8
    }

    pub const fn to_index(self) -> usize {
        (self.to_inner() - Self::MIN_VALUE) as usize
    }

    pub const fn next(self) -> Option<Self> {
        match self {
            ROW_1 => Some(ROW_2),
            ROW_2 => Some(ROW_3),
            ROW_3 => Some(ROW_4),
            ROW_4 => Some(ROW_5),
            ROW_5 => None,
        }
    }

    pub const fn prev(self) -> Option<Self> {
        match self {
            ROW_1 => None,
            ROW_2 => Some(ROW_1),
            ROW_3 => Some(ROW_2),
            ROW_4 => Some(ROW_3),
            ROW_5 => Some(ROW_4),
        }
    }

    pub const fn all() -> [Self; Self::NUM] {
        [ROW_1, ROW_2, ROW_3, ROW_4, ROW_5]
    }

    const fn is_valid(inner: u8) -> bool {
        matches!(inner, Self::MIN_VALUE..=Self::MAX_VALUE)
    }
}

impl std::fmt::Debug for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            ROW_1 => "ROW_1",
            ROW_2 => "ROW_2",
            ROW_3 => "ROW_3",
            ROW_4 => "ROW_4",
            ROW_5 => "ROW_5",
        };
        f.write_str(s)
    }
}

impl std::fmt::Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match *self {
            ROW_1 => '1',
            ROW_2 => '2',
            ROW_3 => '3',
            ROW_4 => '4',
            ROW_5 => '5',
        };
        f.write_char(c)
    }
}

/// 盤面のマス。
#[repr(u8)]
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
#[rustfmt::skip]
pub enum Square {
    SqA1 = 1, SqA2, SqA3, SqA4, SqA5,
    SqB1,     SqB2, SqB3, SqB4, SqB5,
    SqC1,     SqC2, SqC3, SqC4, SqC5,
    SqD1,     SqD2, SqD3, SqD4, SqD5,
    SqE1,     SqE2, SqE3, SqE4, SqE5,
}

pub const SQ_A1: Square = Square::SqA1;
pub const SQ_A2: Square = Square::SqA2;
pub const SQ_A3: Square = Square::SqA3;
pub const SQ_A4: Square = Square::SqA4;
pub const SQ_A5: Square = Square::SqA5;
pub const SQ_B1: Square = Square::SqB1;
pub const SQ_B2: Square = Square::SqB2;
pub const SQ_B3: Square = Square::SqB3;
pub const SQ_B4: Square = Square::SqB4;
pub const SQ_B5: Square = Square::SqB5;
pub const SQ_C1: Square = Square::SqC1;
pub const SQ_C2: Square = Square::SqC2;
pub const SQ_C3: Square = Square::SqC3;
pub const SQ_C4: Square = Square::SqC4;
pub const SQ_C5: Square = Square::SqC5;
pub const SQ_D1: Square = Square::SqD1;
pub const SQ_D2: Square = Square::SqD2;
pub const SQ_D3: Square = Square::SqD3;
pub const SQ_D4: Square = Square::SqD4;
pub const SQ_D5: Square = Square::SqD5;
pub const SQ_E1: Square = Square::SqE1;
pub const SQ_E2: Square = Square::SqE2;
pub const SQ_E3: Square = Square::SqE3;
pub const SQ_E4: Square = Square::SqE4;
pub const SQ_E5: Square = Square::SqE5;

impl Square {
    pub const NUM: usize = Col::NUM * Row::NUM;

    pub const MIN_VALUE: u8 = 1;
    pub const MAX_VALUE: u8 = 25;

    pub const fn from_inner(inner: u8) -> Option<Self> {
        if Self::is_valid(inner) {
            Some(unsafe { Self::from_inner_unchecked(inner) })
        } else {
            None
        }
    }

    /// # Safety
    ///
    /// `inner` は有効値でなければならない。
    pub const unsafe fn from_inner_unchecked(inner: u8) -> Self {
        assert_unchecked!(Self::is_valid(inner));

        std::mem::transmute(inner)
    }

    pub const fn to_inner(self) -> u8 {
        self as u8
    }

    pub const fn to_index(self) -> usize {
        (self.to_inner() - Self::MIN_VALUE) as usize
    }

    pub const fn new(col: Col, row: Row) -> Self {
        let inner = 5 * (col.to_inner() - 1) + row.to_inner();
        unsafe { Self::from_inner_unchecked(inner) }
    }

    pub const fn col(self) -> Col {
        let inner = (self.to_inner() - 1) / 5 + 1;
        unsafe { Col::from_inner_unchecked(inner) }
    }

    pub const fn row(self) -> Row {
        let inner = (self.to_inner() - 1) % 5 + 1;
        unsafe { Row::from_inner_unchecked(inner) }
    }

    pub const fn all() -> [Self; Self::NUM] {
        #[rustfmt::skip]
        const ALL: [Square; Square::NUM] = [
            SQ_A1, SQ_A2, SQ_A3, SQ_A4, SQ_A5,
            SQ_B1, SQ_B2, SQ_B3, SQ_B4, SQ_B5,
            SQ_C1, SQ_C2, SQ_C3, SQ_C4, SQ_C5,
            SQ_D1, SQ_D2, SQ_D3, SQ_D4, SQ_D5,
            SQ_E1, SQ_E2, SQ_E3, SQ_E4, SQ_E5,
        ];

        ALL
    }

    const fn is_valid(inner: u8) -> bool {
        matches!(inner, Self::MIN_VALUE..=Self::MAX_VALUE)
    }
}

impl std::fmt::Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            SQ_A1 => "SQ_A1",
            SQ_A2 => "SQ_A2",
            SQ_A3 => "SQ_A3",
            SQ_A4 => "SQ_A4",
            SQ_A5 => "SQ_A5",
            SQ_B1 => "SQ_B1",
            SQ_B2 => "SQ_B2",
            SQ_B3 => "SQ_B3",
            SQ_B4 => "SQ_B4",
            SQ_B5 => "SQ_B5",
            SQ_C1 => "SQ_C1",
            SQ_C2 => "SQ_C2",
            SQ_C3 => "SQ_C3",
            SQ_C4 => "SQ_C4",
            SQ_C5 => "SQ_C5",
            SQ_D1 => "SQ_D1",
            SQ_D2 => "SQ_D2",
            SQ_D3 => "SQ_D3",
            SQ_D4 => "SQ_D4",
            SQ_D5 => "SQ_D5",
            SQ_E1 => "SQ_E1",
            SQ_E2 => "SQ_E2",
            SQ_E3 => "SQ_E3",
            SQ_E4 => "SQ_E4",
            SQ_E5 => "SQ_E5",
        };
        f.write_str(s)
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.col().fmt(f)?;
        self.row().fmt(f)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_new() {
        for (col, row) in itertools::iproduct!(Col::all(), Row::all()) {
            let sq = Square::new(col, row);
            assert_eq!(sq.col(), col);
            assert_eq!(sq.row(), row);
        }
    }
}

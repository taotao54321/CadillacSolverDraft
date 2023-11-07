use std::fmt::Write as _;
use std::num::NonZeroU8;

use anyhow::{anyhow, bail, ensure, Context as _};
use ascii::{AsciiChar, AsciiStr};

use crate::macros::{assert_unchecked, unreachable_unchecked};

/// カードのスート。
#[repr(u8)]
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CardSuit {
    Spade = 1,
    Club,
    Heart,
    Diamond,
}

pub const SPADE: CardSuit = CardSuit::Spade;
pub const CLUB: CardSuit = CardSuit::Club;
pub const HEART: CardSuit = CardSuit::Heart;
pub const DIAMOND: CardSuit = CardSuit::Diamond;

impl CardSuit {
    pub const NUM: usize = 4;

    pub const MIN_VALUE: u8 = 1;
    pub const MAX_VALUE: u8 = 4;

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

    pub const fn all() -> [Self; Self::NUM] {
        [SPADE, CLUB, HEART, DIAMOND]
    }

    const fn is_valid(inner: u8) -> bool {
        matches!(inner, Self::MIN_VALUE..=Self::MAX_VALUE)
    }

    fn parse_ascii_char(ch: AsciiChar) -> anyhow::Result<Self> {
        match ch {
            AsciiChar::S => Ok(SPADE),
            AsciiChar::C => Ok(CLUB),
            AsciiChar::H => Ok(HEART),
            AsciiChar::D => Ok(DIAMOND),
            _ => bail!("無効なスート文字: '{ch}'"),
        }
    }
}

impl std::str::FromStr for CardSuit {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ch = str_to_ascii_char(s).ok_or_else(|| anyhow!("無効なスート文字列: '{s}'"))?;

        Self::parse_ascii_char(ch)
    }
}

impl std::fmt::Debug for CardSuit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            SPADE => "SPADE",
            CLUB => "CLUB",
            HEART => "HEART",
            DIAMOND => "DIAMOND",
        };
        f.write_str(s)
    }
}

impl std::fmt::Display for CardSuit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match *self {
            SPADE => 'S',
            CLUB => 'C',
            HEART => 'H',
            DIAMOND => 'D',
        };
        f.write_char(c)
    }
}

/// カードのランク。
#[repr(u8)]
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CardRank {
    Rank1 = 1,
    Rank2,
    Rank3,
    Rank4,
    Rank5,
    Rank6,
    Rank7,
    Rank8,
    Rank9,
    Rank10,
    Rank11,
    Rank12,
    Rank13,
}

pub const RANK_A: CardRank = CardRank::Rank1;
pub const RANK_2: CardRank = CardRank::Rank2;
pub const RANK_3: CardRank = CardRank::Rank3;
pub const RANK_4: CardRank = CardRank::Rank4;
pub const RANK_5: CardRank = CardRank::Rank5;
pub const RANK_6: CardRank = CardRank::Rank6;
pub const RANK_7: CardRank = CardRank::Rank7;
pub const RANK_8: CardRank = CardRank::Rank8;
pub const RANK_9: CardRank = CardRank::Rank9;
pub const RANK_T: CardRank = CardRank::Rank10;
pub const RANK_J: CardRank = CardRank::Rank11;
pub const RANK_Q: CardRank = CardRank::Rank12;
pub const RANK_K: CardRank = CardRank::Rank13;

impl CardRank {
    pub const NUM: usize = 13;

    pub const MIN_VALUE: u8 = 1;
    pub const MAX_VALUE: u8 = 13;

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

    pub const fn next(self) -> Self {
        match self {
            RANK_A => RANK_2,
            RANK_2 => RANK_3,
            RANK_3 => RANK_4,
            RANK_4 => RANK_5,
            RANK_5 => RANK_6,
            RANK_6 => RANK_7,
            RANK_7 => RANK_8,
            RANK_8 => RANK_9,
            RANK_9 => RANK_T,
            RANK_T => RANK_J,
            RANK_J => RANK_Q,
            RANK_Q => RANK_K,
            RANK_K => RANK_A,
        }
    }

    pub const fn prev(self) -> Self {
        match self {
            RANK_A => RANK_K,
            RANK_2 => RANK_A,
            RANK_3 => RANK_2,
            RANK_4 => RANK_3,
            RANK_5 => RANK_4,
            RANK_6 => RANK_5,
            RANK_7 => RANK_6,
            RANK_8 => RANK_7,
            RANK_9 => RANK_8,
            RANK_T => RANK_9,
            RANK_J => RANK_T,
            RANK_Q => RANK_J,
            RANK_K => RANK_Q,
        }
    }

    pub const fn all() -> [Self; Self::NUM] {
        [
            RANK_A, RANK_2, RANK_3, RANK_4, RANK_5, RANK_6, RANK_7, RANK_8, RANK_9, RANK_T, RANK_J,
            RANK_Q, RANK_K,
        ]
    }

    const fn is_valid(inner: u8) -> bool {
        matches!(inner, Self::MIN_VALUE..=Self::MAX_VALUE)
    }

    fn parse_ascii_char(ch: AsciiChar) -> anyhow::Result<Self> {
        match ch {
            AsciiChar::A => Ok(RANK_A),
            AsciiChar::_2 => Ok(RANK_2),
            AsciiChar::_3 => Ok(RANK_3),
            AsciiChar::_4 => Ok(RANK_4),
            AsciiChar::_5 => Ok(RANK_5),
            AsciiChar::_6 => Ok(RANK_6),
            AsciiChar::_7 => Ok(RANK_7),
            AsciiChar::_8 => Ok(RANK_8),
            AsciiChar::_9 => Ok(RANK_9),
            AsciiChar::T => Ok(RANK_T),
            AsciiChar::J => Ok(RANK_J),
            AsciiChar::Q => Ok(RANK_Q),
            AsciiChar::K => Ok(RANK_K),
            _ => bail!("無効なランク文字: '{ch}'"),
        }
    }
}

impl std::str::FromStr for CardRank {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ch = str_to_ascii_char(s).ok_or_else(|| anyhow!("無効なランク文字列: '{s}'"))?;

        Self::parse_ascii_char(ch)
    }
}

impl std::fmt::Debug for CardRank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            RANK_A => "RANK_A",
            RANK_2 => "RANK_2",
            RANK_3 => "RANK_3",
            RANK_4 => "RANK_4",
            RANK_5 => "RANK_5",
            RANK_6 => "RANK_6",
            RANK_7 => "RANK_7",
            RANK_8 => "RANK_8",
            RANK_9 => "RANK_9",
            RANK_T => "RANK_T",
            RANK_J => "RANK_J",
            RANK_Q => "RANK_Q",
            RANK_K => "RANK_K",
        };
        f.write_str(s)
    }
}

impl std::fmt::Display for CardRank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match *self {
            RANK_A => 'A',
            RANK_2 => '2',
            RANK_3 => '3',
            RANK_4 => '4',
            RANK_5 => '5',
            RANK_6 => '6',
            RANK_7 => '7',
            RANK_8 => '8',
            RANK_9 => '9',
            RANK_T => 'T',
            RANK_J => 'J',
            RANK_Q => 'Q',
            RANK_K => 'K',
        };
        f.write_char(c)
    }
}

/// カード。
#[repr(transparent)]
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Card(NonZeroU8);

pub const CARD_SA: Card = Card::new(SPADE, RANK_A);
pub const CARD_S2: Card = Card::new(SPADE, RANK_2);
pub const CARD_S3: Card = Card::new(SPADE, RANK_3);
pub const CARD_S4: Card = Card::new(SPADE, RANK_4);
pub const CARD_S5: Card = Card::new(SPADE, RANK_5);
pub const CARD_S6: Card = Card::new(SPADE, RANK_6);
pub const CARD_S7: Card = Card::new(SPADE, RANK_7);
pub const CARD_S8: Card = Card::new(SPADE, RANK_8);
pub const CARD_S9: Card = Card::new(SPADE, RANK_9);
pub const CARD_ST: Card = Card::new(SPADE, RANK_T);
pub const CARD_SJ: Card = Card::new(SPADE, RANK_J);
pub const CARD_SQ: Card = Card::new(SPADE, RANK_Q);
pub const CARD_SK: Card = Card::new(SPADE, RANK_K);
pub const CARD_CA: Card = Card::new(CLUB, RANK_A);
pub const CARD_C2: Card = Card::new(CLUB, RANK_2);
pub const CARD_C3: Card = Card::new(CLUB, RANK_3);
pub const CARD_C4: Card = Card::new(CLUB, RANK_4);
pub const CARD_C5: Card = Card::new(CLUB, RANK_5);
pub const CARD_C6: Card = Card::new(CLUB, RANK_6);
pub const CARD_C7: Card = Card::new(CLUB, RANK_7);
pub const CARD_C8: Card = Card::new(CLUB, RANK_8);
pub const CARD_C9: Card = Card::new(CLUB, RANK_9);
pub const CARD_CT: Card = Card::new(CLUB, RANK_T);
pub const CARD_CJ: Card = Card::new(CLUB, RANK_J);
pub const CARD_CQ: Card = Card::new(CLUB, RANK_Q);
pub const CARD_CK: Card = Card::new(CLUB, RANK_K);
pub const CARD_HA: Card = Card::new(HEART, RANK_A);
pub const CARD_H2: Card = Card::new(HEART, RANK_2);
pub const CARD_H3: Card = Card::new(HEART, RANK_3);
pub const CARD_H4: Card = Card::new(HEART, RANK_4);
pub const CARD_H5: Card = Card::new(HEART, RANK_5);
pub const CARD_H6: Card = Card::new(HEART, RANK_6);
pub const CARD_H7: Card = Card::new(HEART, RANK_7);
pub const CARD_H8: Card = Card::new(HEART, RANK_8);
pub const CARD_H9: Card = Card::new(HEART, RANK_9);
pub const CARD_HT: Card = Card::new(HEART, RANK_T);
pub const CARD_HJ: Card = Card::new(HEART, RANK_J);
pub const CARD_HQ: Card = Card::new(HEART, RANK_Q);
pub const CARD_HK: Card = Card::new(HEART, RANK_K);
pub const CARD_DA: Card = Card::new(DIAMOND, RANK_A);
pub const CARD_D2: Card = Card::new(DIAMOND, RANK_2);
pub const CARD_D3: Card = Card::new(DIAMOND, RANK_3);
pub const CARD_D4: Card = Card::new(DIAMOND, RANK_4);
pub const CARD_D5: Card = Card::new(DIAMOND, RANK_5);
pub const CARD_D6: Card = Card::new(DIAMOND, RANK_6);
pub const CARD_D7: Card = Card::new(DIAMOND, RANK_7);
pub const CARD_D8: Card = Card::new(DIAMOND, RANK_8);
pub const CARD_D9: Card = Card::new(DIAMOND, RANK_9);
pub const CARD_DT: Card = Card::new(DIAMOND, RANK_T);
pub const CARD_DJ: Card = Card::new(DIAMOND, RANK_J);
pub const CARD_DQ: Card = Card::new(DIAMOND, RANK_Q);
pub const CARD_DK: Card = Card::new(DIAMOND, RANK_K);

impl Card {
    pub const NUM: usize = CardSuit::NUM * CardRank::NUM;

    pub const fn new(suit: CardSuit, rank: CardRank) -> Self {
        let inner = (suit.to_inner() << 4) | rank.to_inner();
        Self(unsafe { NonZeroU8::new_unchecked(inner) })
    }

    /// 原作における内部値からカードを作る。
    pub const fn from_cadillac_value(value: u8) -> Option<Self> {
        let Some(suit) = CardSuit::from_inner(1 + (value >> 4)) else {
            return None;
        };
        let Some(rank) = CardRank::from_inner(value & 0xF) else {
            return None;
        };

        Some(Self::new(suit, rank))
    }

    pub const fn suit(self) -> CardSuit {
        let inner = self.0.get() >> 4;
        unsafe { CardSuit::from_inner_unchecked(inner) }
    }

    pub const fn rank(self) -> CardRank {
        let inner = self.0.get() & 0xF;
        unsafe { CardRank::from_inner_unchecked(inner) }
    }

    /// 原作における内部値を返す。
    pub const fn to_cadillac_value(self) -> u8 {
        let mask_suit = (self.suit().to_inner() - 1) << 4;
        let mask_rank = self.rank().to_inner();

        mask_suit | mask_rank
    }

    pub const fn all() -> [Self; Self::NUM] {
        #[rustfmt::skip]
        const ALL: [Card; Card::NUM] = [
            CARD_SA, CARD_S2, CARD_S3, CARD_S4, CARD_S5, CARD_S6, CARD_S7, CARD_S8, CARD_S9, CARD_ST, CARD_SJ, CARD_SQ, CARD_SK,
            CARD_CA, CARD_C2, CARD_C3, CARD_C4, CARD_C5, CARD_C6, CARD_C7, CARD_C8, CARD_C9, CARD_CT, CARD_CJ, CARD_CQ, CARD_CK,
            CARD_HA, CARD_H2, CARD_H3, CARD_H4, CARD_H5, CARD_H6, CARD_H7, CARD_H8, CARD_H9, CARD_HT, CARD_HJ, CARD_HQ, CARD_HK,
            CARD_DA, CARD_D2, CARD_D3, CARD_D4, CARD_D5, CARD_D6, CARD_D7, CARD_D8, CARD_D9, CARD_DT, CARD_DJ, CARD_DQ, CARD_DK,
        ];

        ALL
    }

    fn parse_ascii_str(s: &AsciiStr) -> anyhow::Result<Self> {
        fn parse_suit_rank(s: &AsciiStr) -> anyhow::Result<(CardSuit, CardRank)> {
            ensure!(s.len() == 2, "カード文字列は 2 文字でなければならない");
            let suit = CardSuit::parse_ascii_char(s[0])?;
            let rank = CardRank::parse_ascii_char(s[1])?;
            Ok((suit, rank))
        }

        let (suit, rank) =
            parse_suit_rank(s).with_context(|| format!("無効なカード文字列: '{s}'"))?;

        Ok(Self::new(suit, rank))
    }
}

impl std::str::FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = AsciiStr::from_ascii(s).with_context(|| format!("無効なカード文字列: '{s}'"))?;

        Self::parse_ascii_str(s)
    }
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            CARD_SA => "CARD_SA",
            CARD_S2 => "CARD_S2",
            CARD_S3 => "CARD_S3",
            CARD_S4 => "CARD_S4",
            CARD_S5 => "CARD_S5",
            CARD_S6 => "CARD_S6",
            CARD_S7 => "CARD_S7",
            CARD_S8 => "CARD_S8",
            CARD_S9 => "CARD_S9",
            CARD_ST => "CARD_ST",
            CARD_SJ => "CARD_SJ",
            CARD_SQ => "CARD_SQ",
            CARD_SK => "CARD_SK",
            CARD_CA => "CARD_CA",
            CARD_C2 => "CARD_C2",
            CARD_C3 => "CARD_C3",
            CARD_C4 => "CARD_C4",
            CARD_C5 => "CARD_C5",
            CARD_C6 => "CARD_C6",
            CARD_C7 => "CARD_C7",
            CARD_C8 => "CARD_C8",
            CARD_C9 => "CARD_C9",
            CARD_CT => "CARD_CT",
            CARD_CJ => "CARD_CJ",
            CARD_CQ => "CARD_CQ",
            CARD_CK => "CARD_CK",
            CARD_HA => "CARD_HA",
            CARD_H2 => "CARD_H2",
            CARD_H3 => "CARD_H3",
            CARD_H4 => "CARD_H4",
            CARD_H5 => "CARD_H5",
            CARD_H6 => "CARD_H6",
            CARD_H7 => "CARD_H7",
            CARD_H8 => "CARD_H8",
            CARD_H9 => "CARD_H9",
            CARD_HT => "CARD_HT",
            CARD_HJ => "CARD_HJ",
            CARD_HQ => "CARD_HQ",
            CARD_HK => "CARD_HK",
            CARD_DA => "CARD_DA",
            CARD_D2 => "CARD_D2",
            CARD_D3 => "CARD_D3",
            CARD_D4 => "CARD_D4",
            CARD_D5 => "CARD_D5",
            CARD_D6 => "CARD_D6",
            CARD_D7 => "CARD_D7",
            CARD_D8 => "CARD_D8",
            CARD_D9 => "CARD_D9",
            CARD_DT => "CARD_DT",
            CARD_DJ => "CARD_DJ",
            CARD_DQ => "CARD_DQ",
            CARD_DK => "CARD_DK",
            _ => unsafe { unreachable_unchecked!() },
        };
        f.write_str(s)
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.suit().fmt(f)?;
        self.rank().fmt(f)?;

        Ok(())
    }
}

fn str_to_ascii_char(s: &str) -> Option<AsciiChar> {
    let s = AsciiStr::from_ascii(s).ok()?;
    (s.len() == 1).then(|| s[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_new() {
        for (suit, rank) in itertools::iproduct!(CardSuit::all(), CardRank::all()) {
            let card = Card::new(suit, rank);
            assert_eq!(card.suit(), suit);
            assert_eq!(card.rank(), rank);
        }
    }

    #[test]
    fn test_card_io() {
        for card_orig in Card::all() {
            let s = card_orig.to_string();
            let card: Card = s.parse().unwrap();
            assert_eq!(card, card_orig);
        }
    }
}

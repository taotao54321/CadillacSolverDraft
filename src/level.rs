use crate::macros::assert_unchecked;

/// 原作のゲームレベル (`1..=10`)。
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Level {
    Level1 = 1,
    Level2,
    Level3,
    Level4,
    Level5,
    Level6,
    Level7,
    Level8,
    Level9,
    Level10,
}

pub const LEVEL_1: Level = Level::Level1;
pub const LEVEL_2: Level = Level::Level2;
pub const LEVEL_3: Level = Level::Level3;
pub const LEVEL_4: Level = Level::Level4;
pub const LEVEL_5: Level = Level::Level5;
pub const LEVEL_6: Level = Level::Level6;
pub const LEVEL_7: Level = Level::Level7;
pub const LEVEL_8: Level = Level::Level8;
pub const LEVEL_9: Level = Level::Level9;
pub const LEVEL_10: Level = Level::Level10;

impl Level {
    pub const MIN_VALUE: u8 = 1;
    pub const MAX_VALUE: u8 = 10;

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

    const fn is_valid(inner: u8) -> bool {
        matches!(inner, Self::MIN_VALUE..=Self::MAX_VALUE)
    }
}

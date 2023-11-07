use anyhow::{anyhow, bail};
use bitvec::prelude::*;

use crate::square::{Col, COL_A, COL_B, COL_C, COL_D, COL_E};
use crate::PLY_COUNT_MAX;

/// ゲームの解手順。レベル 9, 10 専用。
///
/// 1 手あたり 3bit で記録する。
/// レベル 9 または 10 は 45 手かかるので、解は `3 * 45 = 135` bit となる。
#[derive(Clone, Default, Eq, PartialEq)]
pub struct Solution(BitArr!(for 3 * PLY_COUNT_MAX, in u32));

impl Solution {
    /// 空の手順を返す。
    pub fn new() -> Self {
        Self::default()
    }

    /// `ply` 手目の手を返す。
    pub fn get_move(&self, ply: usize) -> Option<Col> {
        let inner: u8 = self.0[3 * ply..][..3].load();
        Col::from_inner(inner)
    }

    /// `ply` 手目に `mv` を追加した手順を返す。
    pub fn add_move(&self, ply: usize, mv: Col) -> Self {
        let mut res = self.clone();
        res.add_move_inplace(ply, mv);
        res
    }

    /// `ply` 手目に `mv` を追加する。in-place 処理。
    pub fn add_move_inplace(&mut self, ply: usize, mv: Col) {
        let inner = mv.to_inner();
        self.0[3 * ply..][..3].store(inner);
    }

    /// 手数を返す。計算量は `O(手数)` であることに注意。
    pub fn len(&self) -> usize {
        (0..PLY_COUNT_MAX)
            .map(|ply| self.get_move(ply))
            .position(|mv| mv.is_none())
            .unwrap_or(PLY_COUNT_MAX)
    }

    /// 手順が空かどうかを返す。
    pub fn is_empty(&self) -> bool {
        self.get_move(0).is_none()
    }

    /// 手を最初から順に列挙する。
    pub fn iter(&self) -> impl Iterator<Item = Col> + std::iter::FusedIterator + '_ {
        (0..PLY_COUNT_MAX)
            .map(|ply| self.get_move(ply))
            .fuse()
            .flatten()
    }
}

impl std::str::FromStr for Solution {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .strip_prefix('[')
            .and_then(|s| s.strip_suffix(']'))
            .ok_or_else(|| anyhow!("手順文字列が [] でくくられていない"))?;

        let mut sol = Solution::new();

        let tokens = s.split(',').map(str::trim);
        for (ply, token) in tokens.enumerate() {
            let mv = match token {
                "A" => COL_A,
                "B" => COL_B,
                "C" => COL_C,
                "D" => COL_D,
                "E" => COL_E,
                _ => bail!("ply {ply}: 無効な着手文字列: {token}"),
            };
            sol.add_move_inplace(ply, mv);
        }

        Ok(sol)
    }
}

impl std::fmt::Debug for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as std::fmt::Display>::fmt(self, f)
    }
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        for (ply, mv) in self.iter().enumerate() {
            if ply != 0 {
                f.write_str(", ")?;
            }
            MoveDisplay(mv).fmt(f)?;
        }
        f.write_str("]")?;

        Ok(())
    }
}

#[derive(Debug)]
struct MoveDisplay(Col);

impl std::fmt::Display for MoveDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            COL_A => f.write_str("A"),
            COL_B => f.write_str("B"),
            COL_C => f.write_str("C"),
            COL_D => f.write_str("D"),
            COL_E => f.write_str("E"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::square::*;

    use super::*;

    #[test]
    fn test() {
        {
            let sol = Solution::new();

            assert_eq!(sol.len(), 0);
            assert!(sol.is_empty());
            assert_eq!(sol.get_move(0), None);
        }

        {
            let sol = Solution::new().add_move(0, COL_A).add_move(1, COL_B);
            assert_eq!(sol.len(), 2);
            assert!(!sol.is_empty());
            let cols: Vec<_> = sol.iter().collect();
            assert_eq!(cols, [COL_A, COL_B]);
        }

        {
            let mut sol = Solution::new();
            for ply in 0..PLY_COUNT_MAX {
                sol.add_move_inplace(ply, COL_A);
            }

            assert_eq!(sol.len(), PLY_COUNT_MAX);
            assert!(!sol.is_empty());
            for ply in 0..PLY_COUNT_MAX {
                assert_eq!(sol.get_move(ply), Some(COL_A));
            }
            let cols: Vec<_> = sol.iter().collect();
            assert_eq!(cols, [COL_A; PLY_COUNT_MAX]);
        }

        {
            let mut sol = Solution::new();
            sol.add_move_inplace(0, COL_A);
            sol.add_move_inplace(1, COL_B);
            sol.add_move_inplace(2, COL_C);
            sol.add_move_inplace(3, COL_D);
            sol.add_move_inplace(4, COL_E);

            assert_eq!(sol.len(), 5);
            let cols: Vec<_> = sol.iter().collect();
            assert_eq!(cols, [COL_A, COL_B, COL_C, COL_D, COL_E]);
        }
    }

    #[test]
    fn test_io() {
        let case = "[A, B, C, D, E]";
        let sol: Solution = case.parse().unwrap();
        assert_eq!(sol.to_string(), case);
    }
}

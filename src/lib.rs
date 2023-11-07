mod board;
mod card;
mod endgame;
mod level;
mod macros;
mod midgame;
mod position;
mod solution;
mod square;
mod state;
mod yaku;

pub use self::board::*;
pub use self::card::*;
pub use self::endgame::*;
pub use self::level::*;
pub use self::midgame::*;
pub use self::position::*;
pub use self::solution::*;
pub use self::square::*;
pub use self::state::*;
pub use self::yaku::*;

/// フレーム数。
pub type Frame = u16;

/// 金額 (10 ドル単位)。
pub type Money = u16;

/// レベル 9 または 10 の終了までにかかる手数 (レベル 8 以下はサポートしない)。
pub const PLY_COUNT_MAX: usize = 45;

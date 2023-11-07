//! 終盤の完全読み。

use crate::level::{Level, LEVEL_10, LEVEL_9};
use crate::position::CardPile;
use crate::state::State;
use crate::{Frame, PLY_COUNT_MAX};

/// 完全読み手数(山札残り枚数)の最大値。とりあえず 10 手読みを上限とする (`5^10 ~ 10^7`)。
pub const ENDGAME_PLY_COUNT_MAX: usize = 10;

/// 完全読みを行い、解集合を出力する。
pub fn solve_endgame(level: Level, mut pile: CardPile, state_ini: State, mut frame_best: Frame) {
    assert!(level >= LEVEL_9, "レベル 8 以下は未サポート");
    assert!(
        pile.len() <= ENDGAME_PLY_COUNT_MAX,
        "完全読みは {ENDGAME_PLY_COUNT_MAX} 手が上限"
    );

    dfs(level, &mut pile, state_ini, &mut frame_best);
}

fn dfs(level: Level, pile: &mut CardPile, state: State, frame_best: &mut Frame) {
    // 枝刈り。
    if state.frame() >= *frame_best {
        return;
    }

    let Some(card) = pile.pop() else {
        if state_is_ok(level, &state) {
            *frame_best = state.frame();
            print_answer(&state);
        }
        return;
    };

    let ply = PLY_COUNT_MAX - 1 - pile.len();

    for neighbor in state.neighbors(ply, card) {
        dfs(level, pile, neighbor, frame_best);
    }

    pile.push(card);
}

fn state_is_ok(level: Level, state: &State) -> bool {
    let money_min = match level {
        LEVEL_9 => 200,
        LEVEL_10 => 250,
        _ => unreachable!(),
    };

    state.money() >= money_min && state.card_count() == 0
}

fn print_answer(state: &State) {
    println!("{}\t{}\t{}", state.frame(), state.money(), state.solution());
}

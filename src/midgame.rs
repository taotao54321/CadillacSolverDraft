//! 中盤終わりまでの探索。

use std::cmp::Reverse;

use ordered_float::NotNan;
use rand::prelude::*;

use crate::level::{Level, LEVEL_10, LEVEL_9};
use crate::position::CardPile;
use crate::state::State;
use crate::{Money, PLY_COUNT_MAX};

const BEAM_WIDTH_MAX: usize = 10_000_000;

/// 中盤終わりまでの探索 (`ply_count` 手) を行い、(有望と思われる状態集合, 残りの山札) を返す。
///
/// 返される状態集合はスコアの良い順にソートされている。
pub fn solve_midgame(
    level: Level,
    money: Money,
    pile: CardPile,
    ply_count: usize,
    beam_width: usize,
    rng_seed: u64,
) -> (Vec<State>, CardPile) {
    assert!(ply_count <= PLY_COUNT_MAX);
    assert!(beam_width <= BEAM_WIDTH_MAX);

    let mut rng = SmallRng::seed_from_u64(rng_seed);

    let f_eval = match level {
        LEVEL_9 => eval_level9,
        LEVEL_10 => eval_level10,
        _ => panic!("レベル 8 以下は未サポート"),
    };

    let (state_ini, mut pile) = State::new_initial(level, money, pile);

    eprintln!("中盤終わりまでの探索開始");
    eprintln!("{state_ini}");
    eprintln!();

    let mut beam = Vec::<State>::with_capacity(beam_width);
    beam.push(state_ini);

    let mut beam_nxt = Vec::<State>::with_capacity(5 * beam_width);

    for ply in 0..ply_count {
        eprintln!("midgame ply={ply}");

        let card = pile.pop().unwrap();

        for state in beam.drain(..) {
            beam_nxt.extend(state.neighbors(ply, card));
        }

        // beam_nxt 内に盤面の重複がある場合、フレームコストが最小のもののみを残す。
        beam_nxt.sort_unstable_by(|lhs, rhs| {
            (lhs.board(), lhs.frame()).cmp(&(rhs.board(), rhs.frame()))
        });
        beam_nxt.dedup_by(|a, b| a.board() == b.board());

        // beam_nxt をスコア上位 beam_width 件に絞る。
        if beam_nxt.len() > beam_width {
            beam_nxt.select_nth_unstable_by_key(beam_width, |state| {
                Reverse(f_eval(&mut rng, ply, state))
            });
            beam_nxt.truncate(beam_width);
        }

        beam.append(&mut beam_nxt);
    }

    beam.sort_unstable_by_key(|state| Reverse(state.money()));

    (beam, pile)
}

/// レベル 9 用の評価関数。
fn eval_level9(rng: &mut SmallRng, ply: usize, state: &State) -> NotNan<f64> {
    // 所持金は特に意識しなくても足りるっぽい。

    let frame = f64::from(state.frame());
    //let money = f64::from(state.money());
    let card_count = f64::from(state.card_count());

    //let value_money = 2200.0 * (1.0 / (1.0 + (-0.0015 * money).exp()) - 0.5);

    /*
    let value_frame = -frame;
    let value_card_count = match ply {
        0..=30 => 0.0,
        31..=37 => -50.0 * card_count,
        38.. => -100.0 * card_count,
        _ => unreachable!(),
    };
    let value = value_frame + value_card_count;
    */

    let value_frame = -frame;
    let value_card_count = match ply {
        0..=30 => 0.0,
        31.. => -50.0 * card_count,
        _ => unreachable!(),
    };
    let value_rand = match ply {
        0..=20 => rng.gen_range(0.0..300.0),
        21..=30 => rng.gen_range(0.0..200.0),
        31..=35 => rng.gen_range(0.0..100.0),
        36.. => rng.gen_range(0.0..50.0),
        _ => unreachable!(),
    };
    let value = value_frame + value_card_count + value_rand;

    NotNan::new(value).unwrap()
}

/// レベル 10 用の評価関数。
fn eval_level10(rng: &mut SmallRng, ply: usize, state: &State) -> NotNan<f64> {
    // 所持金は足りてるので特に変える必要なかった。

    eval_level9(rng, ply, state)
}

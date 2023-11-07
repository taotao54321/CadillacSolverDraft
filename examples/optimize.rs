use std::path::PathBuf;

use anyhow::ensure;
use clap::Parser;

use cadillac_solver::*;

/// 既存の解に対してさらに終盤完全読みを行う。
#[derive(Debug, Parser)]
struct Cli {
    /// ゲームレベル。
    #[arg(long, default_value_t = 9, value_parser = clap::value_parser!(u8).range(9..=10))]
    level: u8,

    /// 初期山札配列メモリダンプのパス。
    path_pile: PathBuf,

    /// 既存の解たちが書かれたファイルのパス。
    path_answers: PathBuf,

    /// 終盤完全読み手数。
    endgame_len: usize,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let level = Level::from_inner(cli.level).unwrap();

    let pile = std::fs::read_to_string(&cli.path_pile)?;
    let pile = CardPile::parse_memory_initial(pile)?;

    let answers = std::fs::read_to_string(&cli.path_answers)?;
    let answers = answers
        .lines()
        .map(|line| line.parse::<Answer>())
        .collect::<Result<Vec<_>, _>>()?;

    for answer in answers {
        let (state, pile) = answer.endgame_state(level, pile.clone(), cli.endgame_len);
        optimize(pile, state, answer.frame);
    }

    Ok(())
}

fn optimize(mut pile: CardPile, state_ini: State, mut frame_best: Frame) {
    dfs(&mut pile, state_ini, &mut frame_best);
}

fn dfs(pile: &mut CardPile, state: State, frame_best: &mut Frame) {
    // 枝刈り。
    if state.frame() >= *frame_best {
        return;
    }

    let Some(card) = pile.pop() else {
        // 所持金は足りるものと仮定する。
        if state.card_count() == 0 {
            *frame_best = state.frame();
            print_answer(&state);
        }
        return;
    };

    let ply = PLY_COUNT_MAX - 1 - pile.len();

    for neighbor in state.neighbors(ply, card) {
        dfs(pile, neighbor, frame_best);
    }

    pile.push(card);
}

fn print_answer(state: &State) {
    println!("{}\t{}\t{}", state.frame(), state.money(), state.solution());
}

#[derive(Debug)]
struct Answer {
    frame: Frame,
    solution: Solution,
}

impl Answer {
    fn endgame_state(&self, level: Level, pile: CardPile, endgame_len: usize) -> (State, CardPile) {
        let (mut state, mut pile) = State::new_initial(level, 0, pile);
        for ply in 0..PLY_COUNT_MAX - endgame_len {
            let card = pile.pop().unwrap();
            let col = self.solution.get_move(ply).unwrap();
            state = state.do_move(ply, card, col);
        }

        (state, pile)
    }
}

impl std::str::FromStr for Answer {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields: Vec<_> = s.split('\t').collect();
        ensure!(fields.len() == 3);

        let frame: Frame = fields[0].parse()?;
        let solution: Solution = fields[2].parse()?;

        Ok(Self { frame, solution })
    }
}

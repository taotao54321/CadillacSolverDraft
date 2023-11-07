use std::path::PathBuf;

use anyhow::Context as _;
use clap::Parser;

use cadillac_solver::*;

#[derive(Debug, Parser)]
struct Cli {
    /// ゲームレベル。
    #[arg(long, default_value_t = 9, value_parser = clap::value_parser!(u8).range(9..=10))]
    level: u8,

    /// 開始時の所持金。
    #[arg(long, default_value_t = 0)]
    money: Money,

    /// 既知の最速解のフレーム数。枝刈り用。
    #[arg(long, default_value_t = Frame::MAX)]
    frame_best: Frame,

    /// 中盤終わりまでの探索におけるビーム幅。
    #[arg(long, default_value_t = 10_000_000)]
    midgame_beam_width: usize,

    /// 上位から何件の状態を終盤完全読みの対象とするか。
    #[arg(long, default_value_t = 1_000)]
    endgame_state_count: usize,

    /// 終盤完全読み手数。
    #[arg(long, default_value_t = 7, value_parser = clap::value_parser!(u8).range(1..=10))]
    endgame_len: u8,

    /// 評価関数用の乱数シード。
    #[arg(long, default_value_t = 0)]
    rng_seed: u64,

    /// 初期山札配列メモリダンプのパス。
    path_pile: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let level = Level::from_inner(cli.level).unwrap();
    let endgame_len = usize::from(cli.endgame_len);

    let pile = std::fs::read_to_string(&cli.path_pile).with_context(|| {
        format!(
            "初期山札配列メモリダンプ '{}' を読み取れない",
            cli.path_pile.display()
        )
    })?;
    let pile = CardPile::parse_memory_initial(pile).with_context(|| {
        format!(
            "初期山札配列メモリダンプ '{}' のパースに失敗",
            cli.path_pile.display()
        )
    })?;

    let (mut cands, pile) = solve_midgame(
        level,
        cli.money,
        pile,
        PLY_COUNT_MAX - endgame_len,
        cli.midgame_beam_width,
        cli.rng_seed,
    );
    eprintln!("cands: {}", cands.len());
    eprintln!("上位候補:");
    for cand in &cands[..10.min(cands.len())] {
        eprintln!("{cand}");
        eprintln!();
    }
    assert_eq!(pile.len(), endgame_len);

    cands.truncate(cli.endgame_state_count);

    for (i, cand) in cands.into_iter().enumerate() {
        if i % 100 == 0 {
            eprintln!("endgame cand={i}");
        }
        solve_endgame(level, pile.clone(), cand, cli.frame_best);
    }

    Ok(())
}

use bolt_lang::*;

declare_id!("BCrmcoi7dEgg7UY3SpZfM4dihAWaYuNk3wprXsy1Xp5X");

#[component]
#[derive(Default)]
pub struct Leaderboard {
    pub rank: u64,
    pub player: Pubkey,
    pub total_pnl: i64,
    pub total_trades: u64,
    pub win_rate: u64,
    pub last_updated: i64,
}
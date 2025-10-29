use bolt_lang::*;

declare_id!("3PDo9AKeLhU6hcUC7gft3PKQuotH4624mcevqdSiyTPS");

#[component]
#[derive(Default)]
pub struct TradingAccount {
    pub owner: Pubkey,
    pub balance: u64,
    pub total_pnl: i64,
    pub total_trades: u64,
    pub winning_trades: u64,
    pub losing_trades: u64,
    pub created_at: i64,
}
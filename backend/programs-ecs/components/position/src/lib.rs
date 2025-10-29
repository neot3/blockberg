use bolt_lang::*;

declare_id!("9ACLRxNoDHXpHugLUmDtBGTQ6Q5vwnD4wUVSaWaNaVbv");

#[component]
pub struct Position {
    pub owner: Pubkey,
    pub pair_index: u8,
    pub direction: u8,
    pub entry_price: u64,
    pub size: u64,
    pub take_profit: u64,
    pub stop_loss: u64,
    pub current_price: u64,
    pub pnl: i64,
    pub is_open: bool,
    pub opened_at: i64,
    pub closed_at: i64,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            owner: Pubkey::default(),
            pair_index: 0,
            direction: 0,
            entry_price: 0,
            size: 0,
            take_profit: 0,
            stop_loss: 0,
            current_price: 0,
            pnl: 0,
            is_open: false,
            opened_at: 0,
            closed_at: 0,
        }
    }
}
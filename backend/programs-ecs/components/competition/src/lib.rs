use bolt_lang::*;

declare_id!("FPKpeKHnfYuYo8JDiDW7mNzZB8qgf1mLYwpQAcbGyVhJ");

#[component]
#[derive(Default)]
pub struct Competition {
    pub authority: Pubkey,
    pub start_time: i64,
    pub end_time: i64,
    pub total_participants: u64,
    pub prize_pool: u64,
    pub is_active: bool,
    #[max_len(50)]
    pub name: String,
}
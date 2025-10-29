use bolt_lang::*;
use trading_account::TradingAccount;
use competition::Competition;

declare_id!("5aJzg88rRLAFGN1imRwK84WMD4JyZBvz7n47nSQz9oGm");

#[system]
pub mod join_competition {

    pub fn execute(ctx: Context<Components>, _args: Vec<u8>) -> Result<Components> {
        let clock = Clock::get()?;
        let trading_account = &mut ctx.accounts.trading_account;
        let competition = &mut ctx.accounts.competition;

        require!(competition.is_active, ErrorCode::CompetitionNotActive);

        trading_account.owner = *ctx.accounts.authority.key;
        trading_account.balance = 10000 * 1_000_000;
        trading_account.total_pnl = 0;
        trading_account.total_trades = 0;
        trading_account.winning_trades = 0;
        trading_account.losing_trades = 0;
        trading_account.created_at = clock.unix_timestamp;

        competition.total_participants += 1;

        Ok(ctx.accounts)
    }

    #[system_input]
    pub struct Components {
        pub competition: Competition,
        pub trading_account: TradingAccount,
    }

}

#[error_code]
pub enum ErrorCode {
    #[msg("Competition is not active")]
    CompetitionNotActive,
}
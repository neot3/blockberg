use bolt_lang::*;
use position::Position;
use trading_account::TradingAccount;
use competition::Competition;

declare_id!("GdWvbNgbNxWHbSDTBweSi9zPgtRhggGxaJsCxL5vwDp9");

#[system]
pub mod open_position {

    pub fn execute(ctx: Context<Components>, args: Vec<u8>) -> Result<Components> {
        let args: OpenPositionArgs = OpenPositionArgs::try_from_slice(&args).unwrap();
        let clock = Clock::get()?;

        let position = &mut ctx.accounts.position;
        let trading_account = &mut ctx.accounts.trading_account;
        let competition = &ctx.accounts.competition;

        require!(competition.is_active, ErrorCode::CompetitionNotActive);

        position.owner = *ctx.accounts.authority.key;
        position.pair_index = args.pair_index;
        position.direction = args.direction;
        position.entry_price = args.current_price;
        position.size = args.size;
        position.take_profit = args.take_profit.unwrap_or(0);
        position.stop_loss = args.stop_loss.unwrap_or(0);
        position.current_price = args.current_price;
        position.pnl = 0;
        position.is_open = true;
        position.opened_at = clock.unix_timestamp;
        position.closed_at = 0;

        trading_account.total_trades += 1;

        Ok(ctx.accounts)
    }

    #[system_input]
    pub struct Components {
        pub competition: Competition,
        pub trading_account: TradingAccount,
        pub position: Position,
    }

}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct OpenPositionArgs {
    pub pair_index: u8,
    pub direction: u8,
    pub current_price: u64,
    pub size: u64,
    pub take_profit: Option<u64>,
    pub stop_loss: Option<u64>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Competition is not active")]
    CompetitionNotActive,
}
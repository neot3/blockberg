use bolt_lang::*;
use position::Position;
use trading_account::TradingAccount;
use competition::Competition;

declare_id!("CXnKyp5DGMWRHsj9JsbECqBbDP1GeUF3c8AYSPZMmNb2");

#[system]
pub mod close_position {

    pub fn execute(ctx: Context<Components>, _args: Vec<u8>) -> Result<Components> {
        let clock = Clock::get()?;

        let position = &mut ctx.accounts.position;
        let trading_account = &mut ctx.accounts.trading_account;

        require!(position.is_open, ErrorCode::PositionNotOpen);

        let pnl = if position.direction == 0 {
            ((position.current_price as i128 - position.entry_price as i128) * position.size as i128 / position.entry_price as i128) as i64
        } else {
            ((position.entry_price as i128 - position.current_price as i128) * position.size as i128 / position.entry_price as i128) as i64
        };

        position.pnl = pnl;
        position.is_open = false;
        position.closed_at = clock.unix_timestamp;

        trading_account.total_pnl += pnl;
        if pnl > 0 {
            trading_account.winning_trades += 1;
        } else {
            trading_account.losing_trades += 1;
        }

        Ok(ctx.accounts)
    }

    #[system_input]
    pub struct Components {
        pub competition: Competition,
        pub trading_account: TradingAccount,
        pub position: Position,
    }

}

#[error_code]
pub enum ErrorCode {
    #[msg("Position is not open")]
    PositionNotOpen,
}
use anchor_lang::prelude::*;

use crate::MarketError;
use crate::states::{Market,MarketInitialization};

pub fn _initialize_market(
    ctx: Context<InitializeMarket>,
    target_price: u64,
    feed_id: [u8;66], // from https://pyth.network/developers/price-feed-ids#solana-stables
    market_duration: u64,
) -> Result<()> {
    require_eq!(feed_id.len(), 66, MarketError::IncorrectFeedIDLength);
    require_gte!(market_duration, 1200, MarketError::ShortMarketDuration); //more than one hour

    let market = &mut ctx.accounts.market;
    
    let clock = Clock::get()?;
    market.start_time = clock.slot;
   
    market.target_price = target_price;
    market.market_duration = market_duration;

    market.feed_id = feed_id;
    
    market.creator = ctx.accounts.market_creator.key();
    
    market.bump = ctx.bumps.market;

    market.initialization = MarketInitialization::InitializedMarket;

    Ok(())
}

#[derive(Accounts)]
#[instruction(target_price:u64,feed_id:[u8;66],market_duration:u64)]
pub struct InitializeMarket<'info> {
    #[account(
        init,
        payer = market_creator,
        space = 8 + Market::INIT_SPACE,
        seeds = [
            market_creator.key().as_ref(), 
            &feed_id,
            &target_price.to_le_bytes(), 
            &market_duration.to_le_bytes(),
        ],
        bump
    )]
    pub market: Box<Account<'info, Market>>,

    #[account(mut)]
    pub market_creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

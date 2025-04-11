use anchor_lang::prelude::*;

declare_id!("5gKE9tm1ReAY1P3WmTCyC9FcYXZooCEy89eAX9XNTMyj");

#[program]
pub mod credora {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

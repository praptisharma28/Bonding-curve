use anchor_lang::prelude::*;

// user transfer sol to bonding curve pda
pub fn sol_transfer_from_user<'info>(
    signer: &Signer<'info>,
    destination: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    amount: u64,
) -> Result<()> {
    let tx = transfer(signer.key, destination.key, amount);
    invoke(
        &tx,
        &[
            signer.to_account_info(),
            destination.to_account_info(),
            system_program.to_account_info(),
        ],
    )?;
    Ok(())
}

// bonding curve pda transfer sol to user
pub fn sol_transfer_from_users<'info>(
    signer: &Signer<'info>,
    destination: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    amount: u64,
) -> Result<()> {
    let tx = transfer(signer.key, destination.key, amount);
    invoke(
        &tx,
        &[
            signer.to_account_info(),
            destination.to_account_info(),
            system_program.to_account_info(),
        ],
    );
    Ok(())
}

// user transfer tokens to bonding curve pda
pub fn token_transfer_from_user<'info>(
    from: &AccountInfo<'info>,
    to: &AccountInfo<'info>,
    authority: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    amount: u64,
) -> Result<()> {
    let cpi_context = CpiContext::new(
        token_program.to_account_info(),
        anchor_spl::token::Transfer {
            from: from.to_account_info(),
            to: to.to_account_info(),
            authority: authority.to_account_info(),
        },
    );
    anchor_spl::token::transfer(cpi_context, amount)?;
    Ok(())
}

// bonding curve pda transfer tokens to user
pub fn token_transfer_from_signers<'info>(
    from: &AccountInfo<'info>,
    to: &AccountInfo<'info>,
    authority: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    amount: u64,
    signer_seeds: &[&[u8]],
) -> Result<()> {
    let cpi_context = CpiContext::new_with_signer(
        token_program.to_account_info(),
        anchor_spl::token::Transfer {
            from: from.to_account_info(),
            to: to.to_account_info(),
            authority: authority.to_account_info(),
        },
        signer_seeds,
    );
    anchor_spl::token::transfer(cpi_context, amount)?;
    Ok(())
}

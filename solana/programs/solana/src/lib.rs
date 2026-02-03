pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("CxhYzyLC7jxr3ki8i2roSRwSVrPqtPuNsjEgbgT5gR9x");

#[program]
pub mod solana {
    use super::*;

    pub fn registrar_libro(
        ctx: Context<RegistrarLibro>,
        id: u64,
        titulo: String,
        descripcion: String,
    ) -> Result<()> {
        let libro = &mut ctx.accounts.libro;

        libro.id = id;
        libro.titulo = titulo;
        libro.descripcion = descripcion;
        libro.propietario = ctx.accounts.authority.key();

        Ok(())
    }

    pub fn actualizar_libro(
        ctx: Context<ActualizarLibro>,
        titulo: String,
        descripcion: String,
    ) -> Result<()> {
        let libro = &mut ctx.accounts.libro;

        require!(
            libro.propietario == ctx.accounts.authority.key(),
            ErrorCode::NoAutorizado
        );

        libro.titulo = titulo;
        libro.descripcion = descripcion;

        Ok(())
    }

    pub fn transferir_libro(
        ctx: Context<ActualizarLibro>,
        nuevo_propietario: Pubkey,
    ) -> Result<()> {
        let libro = &mut ctx.accounts.libro;

        require!(
            libro.propietario == ctx.accounts.authority.key(),
            ErrorCode::NoAutorizado
        );

        libro.propietario = nuevo_propietario;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct RegistrarLibro<'info> {
    #[account(
        init,
        payer = authority,
        seeds = [b"libro", id.to_le_bytes().as_ref()],
        bump,
        space = 8 + 8 + 4 + 100 + 4 + 200 + 32
    )]
    pub libro: Account<'info, Libro>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActualizarLibro<'info> {
    #[account(mut)]
    pub libro: Account<'info, Libro>,

    pub authority: Signer<'info>,
}

#[account]
pub struct Libro {
    pub id: u64,
    pub titulo: String,
    pub descripcion: String,
    pub propietario: Pubkey,
}

#[error_code]
pub enum ErrorCode {
    #[msg("No autorizado para modificar este libro")]
    NoAutorizado,
}

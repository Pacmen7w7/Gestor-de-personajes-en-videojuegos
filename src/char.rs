use anchor_lang::prelude::*;

// ID del programa en Solana
declare_id!("39ydvze2nMAvKAnvqXzRdteKoup3VDna6EsrNrbwGGU4");

#[program]
pub mod game_characters {
    use super::*;

    // CREATE
    // Crear la base de personajes del jugador
    pub fn crear_coleccion(context: Context<NuevaColeccion>, nombre: String) -> Result<()>{
        
        let owner_id = context.accounts.owner.key();

        let personajes: Vec<Personaje> = Vec::new();

        context.accounts.coleccion.set_inner(Coleccion{
            owner: owner_id,
            nombre,
            personajes,
        });

        Ok(())
    }

    // CREATE
    // Agregar personaje
    pub fn agregar_personaje(
        context: Context<NuevoPersonaje>,
        nombre: String,
        nivel: u16
    ) -> Result<()>{

        let personaje = Personaje{
            nombre,
            nivel,
            activo: true,
        };

        context.accounts.coleccion.personajes.push(personaje);

        Ok(())
    }

    // DELETE
    // Eliminar personaje
    pub fn eliminar_personaje(
        context: Context<NuevoPersonaje>,
        nombre:String
    ) -> Result<()>{

        let personajes = &mut context.accounts.coleccion.personajes;

        for i in 0..personajes.len(){

            if personajes[i].nombre == nombre {

                personajes.remove(i);

                msg!("Personaje {} eliminado!", nombre);

                return Ok(());

            }

        }

        Err(Errores::PersonajeNoExiste.into())
    }

    // READ
    // Ver lista de personajes
    pub fn ver_personajes(context: Context<NuevoPersonaje>) -> Result<()>{

        let personajes = &context.accounts.coleccion.personajes;

        msg!("Lista de personajes: {:#?}", personajes);

        Ok(())
    }

    // UPDATE
    // Activar o desactivar personaje
    pub fn alternar_estado(
        context: Context<NuevoPersonaje>,
        nombre: String
    ) -> Result<()>{

        let personajes = &mut context.accounts.coleccion.personajes;

        for i in 0..personajes.len(){

            if personajes[i].nombre == nombre {

                personajes[i].activo = !personajes[i].activo;

                msg!(
                    "El personaje {} ahora está activo: {}",
                    nombre,
                    personajes[i].activo
                );

                return Ok(());

            }

        }

        Err(Errores::PersonajeNoExiste.into())
    }

}

// ERRORES PERSONALIZADOS
#[error_code]
pub enum Errores{

    #[msg("Error: No eres el propietario.")]
    NoEresElOwner,

    #[msg("Error: El personaje no existe.")]
    PersonajeNoExiste,

}

// CUENTA PRINCIPAL (PDA)
#[account]
#[derive(InitSpace)]
pub struct Coleccion{

    // dueño de la colección
    owner: Pubkey,

    // nombre de la colección
    #[max_len(60)]
    nombre: String,

    // lista de personajes
    #[max_len(10)]
    personajes: Vec<Personaje>,

}

// ESTRUCTURA PERSONAJE
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Personaje{

    #[max_len(60)]
    nombre: String,

    nivel: u16,

    activo: bool,

}

// CONTEXTO PARA CREAR LA COLECCIÓN
#[derive(Accounts)]
pub struct NuevaColeccion<'info>{

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = Coleccion::INIT_SPACE + 8,
        seeds = [b"coleccion", owner.key().as_ref()],
        bump
    )]
    pub coleccion: Account<'info, Coleccion>,

    pub system_program: Program<'info, System>,

}

// CONTEXTO PARA OPERACIONES CRUD
#[derive(Accounts)]
pub struct NuevoPersonaje<'info>{

    pub owner: Signer<'info>,

    #[account(mut)]
    pub coleccion: Account<'info, Coleccion>,

}
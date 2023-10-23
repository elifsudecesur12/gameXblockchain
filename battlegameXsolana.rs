use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::clock,
};

solana_program::declare_id!("YourProgramIDHere");

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Player {
    pub id: u64,
    pub pubkey: Pubkey,
    pub energy: u64,
    pub troops: u64,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BattleField {
    pub id: u64,
    pub player1: Pubkey,
    pub player2: Pubkey,
    pub player1_troops: u64,
    pub player2_troops: u64,
}

fn parse_instruction_data(instruction_data: &[u8]) -> Result<(u8, u64), ProgramError> {
    if instruction_data.len() != 9 {
        return Err(ProgramError::InvalidInstructionData);
    }
    let action = instruction_data[0];
    let amount = u64::from_le_bytes(instruction_data[1..9].try_into().unwrap());
    Ok((action, amount))
}

#[entrypoint]
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let player1_account = next_account_info(accounts_iter)?;
    let player2_account = next_account_info(accounts_iter)?;

    let battlefield_account = next_account_info(accounts_iter)?;

    if battlefield_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    let (action, amount) = parse_instruction_data(instruction_data)?;

    let mut player1 = Player::load(player1_account)?;
    let mut player2 = Player::load(player2_account)?;
    let mut battlefield = BattleField::load(battlefield_account)?;

    match action {
        1 => {
            if player1.energy >= amount {
                player1.energy -= amount;
                player1.troops += amount;
                msg!("Oyuncu 1, {} enerji harcayarak {} asker gönderdi.", amount, amount);
            }
        }
        2 => {
            // İkinci oyuncu saldırı yapar
            if player2.energy >= amount {
                player2.energy -= amount;
                player2.troops += amount;
                msg!("Oyuncu 2, {} enerji harcayarak {} asker gönderdi.", amount, amount);
            }
        }
        3 => {
            let player1_power = player1.troops;
            let player2_power = player2.troops;

            if player1_power > player2_power {
                battlefield.player1_troops = player1_power;
                battlefield.player2_troops = 0;
                msg!("Oyuncu 1 savaşı kazandı!");
            } else if player2_power > player1_power {
                battlefield.player1_troops = 0;
                battlefield.player2_troops = player2_power;
                msg!("Oyuncu 2 savaşı kazandı!");
            } else {
                msg!("Savaçı berabere bitti!");
            }
        }
        _ => {
            return Err(ProgramError::InvalidInstructionData);
        }
    }

    player1.save(player1_account)?;
    player2.save(player2_account)?;
    battlefield.save(battlefield_account)?;

    Ok(())
}

impl Player {
    pub fn load(account: &AccountInfo) -> Result<Player, ProgramError> {
        let data = account.try_borrow_data()?;
        let (id, pubkey, energy, troops) = array_refs![data, 8, 32, 8, 8];

        Ok(Player {
            id: u64::from_le_bytes(*id),
            pubkey: Pubkey::new_from_array(*pubkey),
            energy: u64::from_le_bytes(*energy),
            troops: u64::from_le_bytes(*troops),
        })
    }

    pub fn save(&self, account: &AccountInfo) -> Result<(), ProgramError> {
        let data = account.try_borrow_mut_data()?;
        let data_len = data.len();

        let (id_dst, pubkey_dst, energy_dst, troops_dst) =
            mut_array_refs![data, 8, 32, 8, 8];

        id_dst.copy_from_slice(&self.id.to_le_bytes());
        pubkey_dst.copy_from_slice(self.pubkey.to_bytes().as_slice());
        energy_dst.copy_from_slice(&self.energy.to_le_bytes());
        troops_dst.copy_from_slice(&self.troops.to_le_bytes());

        if data_len > 56 {
            for byte in &mut data[56..] {
                *byte = 0;
            }
        }

        Ok(())
    }
}

impl BattleField {
    pub fn load(account: &AccountInfo) -> Result<BattleField, ProgramError> {
        // Veriyi oku ve ayrıştır
        let data = account.try_borrow_data()?;
        let (id, player1, player2, player1_troops, player2_troops) =
            array_refs![data, 8, 32, 32, 8, 8];

        Ok(BattleField {
            id: u64::from_le_bytes(*id),
            player1: Pubkey::new_from_array(*player1),
            player2: Pubkey::new_from_array(*player2),
            player1_troops: u64::from_le_bytes(*player1_troops),
            player2_troops: u64::from_le_bytes(*player2_troops),
        })
    }

    pub fn save(&self, account: &AccountInfo) -> Result<(), ProgramError> {
       
        let data = account.try_borrow_mut_data()?;
        let data_len = data.len();

        let (id_dst, player1_dst, player2_dst, player1_troops_dst, player2_troops_dst) =
            mut_array_refs![data, 8, 32, 32, 8, 8];

        id_dst.copy_from_slice(&self.id.to_le_bytes());
        player1_dst.copy_from_slice(self.player1.to_bytes().as_slice());
        player2_dst.copy_from_slice(self.player2.to_bytes().as_slice());
        player1_troops_dst.copy_from_slice(&self.player1_troops.to_le_bytes());
        player2_troops_dst.copy_from_slice(&self.player2_troops.to_le_bytes());

        
        if data_len > 80 {
            for byte in &mut data[80..] {
                *byte = 0;
            }
        }

        Ok(())
    }
}

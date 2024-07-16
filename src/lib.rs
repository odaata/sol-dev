use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

use crate::instructions::CounterInstructions;

pub mod instructions;

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct CounterAccount {
    pub counter: u32,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instructions_data: &[u8],
) -> ProgramResult {
    msg!("Counter program entry point");

    let instruction: CounterInstructions = CounterInstructions::unpack(instructions_data)?;

    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    let mut counter_account = CounterAccount::try_from_slice(&account.data.borrow())?;

    match instruction {
        CounterInstructions::Increment(args) => {
            counter_account.counter += args.value;
        }
        CounterInstructions::Decrement(args) => match args.value > counter_account.counter {
            true => {
                counter_account.counter = 0;
            }
            false => {
                counter_account.counter -= args.value;
            }
        },
        CounterInstructions::Reset => {
            counter_account.counter = 0;
        }
        CounterInstructions::Update(args) => {
            counter_account.counter = args.value;
        }
    }

    counter_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use solana_program::{clock::Epoch, pubkey::Pubkey};
    use std::mem;

    #[test]
    fn test_increment() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();

        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );

        let accounts = vec![account];

        let mut increment_instruction_data: Vec<u8> = vec![0];
        let increment_value = 48u32;
        increment_instruction_data.extend_from_slice(&increment_value.to_le_bytes());
        process_instruction(&program_id, &accounts, &increment_instruction_data).unwrap();

        let increment_result = CounterAccount::try_from_slice(&accounts[0].data.borrow())
            .unwrap()
            .counter;
        assert_eq!(increment_result, 48);
    }

    #[test]
    fn test_decrement() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();

        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );

        let accounts = vec![account];

        let mut increment_instruction_data: Vec<u8> = vec![0];
        let increment_value = 48u32;
        increment_instruction_data.extend_from_slice(&increment_value.to_le_bytes());
        process_instruction(&program_id, &accounts, &increment_instruction_data).unwrap();

        let increment_result = CounterAccount::try_from_slice(&accounts[0].data.borrow())
            .unwrap()
            .counter;
        assert_eq!(increment_result, 48);

        let mut decrement_instruction_data: Vec<u8> = vec![1];
        let value = 16u32;
        decrement_instruction_data.extend_from_slice(&value.to_le_bytes());
        process_instruction(&program_id, &accounts, &decrement_instruction_data).unwrap();
        let result = CounterAccount::try_from_slice(&accounts[0].data.borrow())
            .unwrap()
            .counter;
        assert_eq!(result, 32);

        let mut big_decrement_instruction_data: Vec<u8> = vec![1];
        big_decrement_instruction_data.extend_from_slice(&100u32.to_le_bytes());
        process_instruction(&program_id, &accounts, &big_decrement_instruction_data).unwrap();

        let result = CounterAccount::try_from_slice(&accounts[0].data.borrow())
            .unwrap()
            .counter;
        assert_eq!(result, 0);
    }

    #[test]
    fn test_update_counter() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();

        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );

        let accounts = vec![account];

        let mut increment_instruction_data: Vec<u8> = vec![0];
        let increment_value = 48u32;
        increment_instruction_data.extend_from_slice(&increment_value.to_le_bytes());
        process_instruction(&program_id, &accounts, &increment_instruction_data).unwrap();
        let result = CounterAccount::try_from_slice(&accounts[0].data.borrow())
            .unwrap()
            .counter;
        assert_eq!(result, 48);

        let mut instruction_data: Vec<u8> = vec![2];
        instruction_data.extend_from_slice(&33u32.to_le_bytes());
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        let result = CounterAccount::try_from_slice(&accounts[0].data.borrow())
            .unwrap()
            .counter;
        assert_eq!(result, 33);
    }

    #[test]
    fn test_reset_counter() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();

        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );

        let accounts = vec![account];

        let mut increment_instruction_data: Vec<u8> = vec![0];
        let increment_value = 48u32;
        increment_instruction_data.extend_from_slice(&increment_value.to_le_bytes());
        process_instruction(&program_id, &accounts, &increment_instruction_data).unwrap();
        let result = CounterAccount::try_from_slice(&accounts[0].data.borrow())
            .unwrap()
            .counter;
        assert_eq!(result, 48);

        let reset_instruction_data: Vec<u8> = vec![3];
        process_instruction(&program_id, &accounts, &reset_instruction_data).unwrap();

        let result = CounterAccount::try_from_slice(&accounts[0].data.borrow())
            .unwrap()
            .counter;
        assert_eq!(result, 0);
    }
}

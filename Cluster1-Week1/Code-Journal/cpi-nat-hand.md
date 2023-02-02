Code journal for [CPI NAT HAND](https://github.com/solana-developers/program-examples/blob/main/basics/cross-program-invocation/native/programs/hand/src/lib.rs)


```rust
// Binary Object Representation Serializer for Hashing
// https://github.com/near/borsh-rs
use borsh::BorshDeserialize;
use lever::SetPowerStatus;
use solana_program::{
    account_info::{ next_account_info, AccountInfo },
    entrypoint,
    entrypoint::ProgramResult,
    instruction::{ AccountMeta, Instruction },
    pubkey::Pubkey,
    program::invoke,
};

entrypoint!(pull_lever);

fn pull_lever(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter(); // return iterator of 'accounts' (2)
    let power = next_account_info(accounts_iter)?; // parse out 'power' account
    let lever_program = next_account_info(accounts_iter)?; // parse out 'lever_program' account

    let set_power_status_instruction = SetPowerStatus::try_from_slice(instruction_data)?; // deserealize instruction data with try_from_slice from setPowerStatus (CPI NAT LEVER) and return instruction data 

    // create new instruction by seralizing the data with Instruction::new_with_borsh
    let ix = Instruction::new_with_borsh(
        lever_program.key.clone(), // our lever program's ID
        &set_power_status_instruction, // passing instructions through from SetPowerStatus::try_from_slice(ix_data)
        vec![AccountMeta::new(power.key.clone(), false)] // Just the required account for the other program
    );

    // program::invoke method provided by solana_program.
    // CPI requires two arguments, a reference ix data created above and a reference to our our power account
    invoke(&ix, &[power.clone()])
}
```
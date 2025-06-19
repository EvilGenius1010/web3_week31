use borsh::{BorshDeserialize, BorshSerialize};
//next_account_info is an ierator that allows you to iterate over the accounts passed in the program
//ACcountInfo is a struct that contains the account information
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    //macro to specify the entrypoint fn for the program
    entrypoint,
    //return type for the program
    entrypoint::ProgramResult,
    //msg! macro equivalent from anchor
    msg,
    pubkey::Pubkey,
};

entrypoint!(program_counter_contract);

pub fn program_counter_contract(
    account_addr: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    
    msg!("Testing ");

    let acc_iter = next_account_info(accounts.iter())?;

    match acc_iter {
        Ok(account_info) => {
            msg!(
                "Correct account taken and its data is {:?}!",
                account_info.data
            );
        }
        Err(e) => return e,
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::clock::Epoch;
}

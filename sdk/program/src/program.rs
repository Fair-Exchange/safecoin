use crate::{account_info::AccountInfo, entrypoint::ProgramResult, instruction::Instruction};

/// Invoke a cross-program instruction.
///
/// Notes:
/// - RefCell checking can be compute unit expensive, to avoid that expense use
///   `invoke_unchecked` instead, but at your own risk.
/// - The program id of the instruction being issued must also be included in
///   `account_infos`.
pub fn invoke(instruction: &Instruction, account_infos: &[AccountInfo]) -> ProgramResult {
    invoke_signed(instruction, account_infos, &[])
}

/// Invoke a cross-program instruction but don't enforce RefCell handling.
///
/// Notes:
/// - The missing checks ensured that the invocation doesn't violate the borrow
///   rules of the `AccountInfo` fields that are wrapped in `RefCell`s.  To
///   include the checks call `invoke` instead.
/// - The program id of the instruction being issued must also be included in
///   `account_infos`.
pub fn invoke_unchecked(instruction: &Instruction, account_infos: &[AccountInfo]) -> ProgramResult {
    invoke_signed_unchecked(instruction, account_infos, &[])
}

/// Invoke a cross-program instruction with program signatures
///
/// Notes:
/// - RefCell checking can be compute unit expensive, to avoid that expense use
///   `invoke_signed_unchecked` instead, but at your own risk.
/// - The program id of the instruction being issued must also be included in
///   `account_infos`.
pub fn invoke_signed(
    instruction: &Instruction,
    account_infos: &[AccountInfo],
    signers_seeds: &[&[&[u8]]],
) -> ProgramResult {
    // Check that the account RefCells are consistent with the request
    for account_meta in instruction.accounts.iter() {
        for account_info in account_infos.iter() {
            if account_meta.pubkey == *account_info.key {
                if account_meta.is_writable {
                    let _ = account_info.try_borrow_mut_lamports()?;
                    let _ = account_info.try_borrow_mut_data()?;
                } else {
                    let _ = account_info.try_borrow_lamports()?;
                    let _ = account_info.try_borrow_data()?;
                }
                break;
            }
        }
    }

    invoke_signed_unchecked(instruction, account_infos, signers_seeds)
}

/// Invoke a cross-program instruction with program signatures but don't check
/// RefCell handling.
///
/// Note:
/// - The missing checks ensured that the invocation doesn't violate the borrow
///   rules of the `AccountInfo` fields that are wrapped in `RefCell`s.  To
///   include the checks call `invoke_signed` instead.
/// - The program id of the instruction being issued must also be included in
///   `account_infos`.
pub fn invoke_signed_unchecked(
    instruction: &Instruction,
    account_infos: &[AccountInfo],
    signers_seeds: &[&[&[u8]]],
) -> ProgramResult {
    #[cfg(target_arch = "bpf")]
    {
        let result = unsafe {
            sol_invoke_signed_rust(
                instruction as *const _ as *const u8,
                account_infos as *const _ as *const u8,
                account_infos.len() as u64,
                signers_seeds as *const _ as *const u8,
                signers_seeds.len() as u64,
            )
        };
        match result {
            crate::entrypoint::SUCCESS => Ok(()),
            _ => Err(result.into()),
        }
    }

    #[cfg(not(target_arch = "bpf"))]
    crate::program_stubs::sol_invoke_signed(instruction, account_infos, signers_seeds)
}

#[cfg(target_arch = "bpf")]
extern "C" {
    fn sol_invoke_signed_rust(
        instruction_addr: *const u8,
        account_infos_addr: *const u8,
        account_infos_len: u64,
        signers_seeds_addr: *const u8,
        signers_seeds_len: u64,
    ) -> u64;
}

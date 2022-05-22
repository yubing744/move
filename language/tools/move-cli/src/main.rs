// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use move_core_types::{account_address::AccountAddress, errmap::ErrorMapping};

fn main() -> Result<()> {
    #[cfg(feature = "no_web")]
    let error_descriptions: ErrorMapping = bcs::from_bytes(move_stdlib::error_descriptions())?;
    #[cfg(feature = "no_web")]
    let cost_table = &move_vm_types::gas_schedule::INITIAL_COST_SCHEDULE;

    move_cli::move_cli(
        #[cfg(feature = "no_web")] move_stdlib::natives::all_natives(AccountAddress::from_hex_literal("0x1").unwrap()),
        #[cfg(feature = "no_web")] cost_table,
        #[cfg(feature = "no_web")] &error_descriptions,
    )
}

use core::fmt::Debug;

use edr_eth::{Address, Bytes, U256};

use crate::{data::ProviderData, time::TimeSinceEpoch, ProviderError};

pub fn handle_set_balance<LoggerErrorT: Debug, TimerT: Clone + TimeSinceEpoch>(
    data: &mut ProviderData<LoggerErrorT, TimerT>,
    address: Address,
    balance: U256,
) -> Result<bool, ProviderError<LoggerErrorT>> {
    data.set_balance(address, balance)?;

    Ok(true)
}

pub fn handle_set_code<LoggerErrorT: Debug, TimerT: Clone + TimeSinceEpoch>(
    data: &mut ProviderData<LoggerErrorT, TimerT>,
    address: Address,
    code: Bytes,
) -> Result<bool, ProviderError<LoggerErrorT>> {
    data.set_code(address, code)?;

    Ok(true)
}

pub fn handle_set_nonce<LoggerErrorT: Debug, TimerT: Clone + TimeSinceEpoch>(
    data: &mut ProviderData<LoggerErrorT, TimerT>,
    address: Address,
    nonce: u64,
) -> Result<bool, ProviderError<LoggerErrorT>> {
    data.set_nonce(address, nonce)?;

    Ok(true)
}

pub fn handle_set_storage_at<LoggerErrorT: Debug, TimerT: Clone + TimeSinceEpoch>(
    data: &mut ProviderData<LoggerErrorT, TimerT>,
    address: Address,
    index: U256,
    value: U256,
) -> Result<bool, ProviderError<LoggerErrorT>> {
    data.set_account_storage_slot(address, index, value)?;

    Ok(true)
}

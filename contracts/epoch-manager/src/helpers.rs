use crate::ContractError;
use amm::constants::DAY_IN_SECONDS;
use cosmwasm_std::{ensure, Uint64};

/// Validates the epoch duration.
pub fn validate_epoch_duration(epoch_duration: Uint64) -> Result<(), ContractError> {
    ensure!(
        epoch_duration >= Uint64::from(DAY_IN_SECONDS),
        ContractError::InvalidEpochDuration {
            min: DAY_IN_SECONDS
            //@note minimum epoch duration is one day, if duration is less than one day it reverts
        }
    );

    Ok(())
}
//checked
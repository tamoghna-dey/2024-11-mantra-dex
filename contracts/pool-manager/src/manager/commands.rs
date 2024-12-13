use cosmwasm_std::{
    attr, ensure, Attribute, BankMsg, Coin, CosmosMsg, DepsMut, Env, MessageInfo, Response, Uint128,
};

use amm::coin::is_factory_token;
use amm::constants::LP_SYMBOL;
use amm::fee::PoolFee;
use amm::pool_manager::{PoolInfo, PoolType};
use amm::tokenfactory::utils::get_factory_denom_creation_fee;

use crate::helpers::{
    validate_fees_are_paid, validate_no_additional_funds_sent_with_pool_creation,
    validate_pool_identifier,
};
use crate::state::{get_pool_by_identifier, POOL_COUNTER};
use crate::{
    state::{Config, CONFIG, POOLS},
    ContractError,
};

pub const MAX_ASSETS_PER_POOL: usize = 4usize;
pub const MIN_ASSETS_PER_POOL: usize = 2usize;

/// The prefix used when creation a pool with an explicitly provided ID
pub const EXPLICIT_POOL_ID_PREFIX: &str = "o.";

/// The prefix used when creation a pool with an auto-generated ID
pub const AUTO_POOL_ID_PREFIX: &str = "p.";

/// Creates a pool with 2, 3, or N assets. The function dynamically handles different numbers of assets,
/// allowing for the creation of pools with varying configurations. The maximum number of assets per pool is defined by
/// the constant `MAX_ASSETS_PER_POOL`.
///
/// # Example
///
/// ```rust
/// # use cosmwasm_std::{DepsMut, Decimal, Env, MessageInfo, Response, CosmosMsg, WasmMsg, to_json_binary};
/// # use amm::fee::PoolFee;
/// # use amm::fee::Fee;
/// # use pool_manager::error::ContractError;
/// # use pool_manager::manager::commands::MAX_ASSETS_PER_POOL;
/// # use pool_manager::manager::commands::create_pool;
/// # use std::convert::TryInto;
/// # use amm::pool_manager::PoolType;
/// #
/// # fn example(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
/// let asset_infos = vec![
///     "uatom".into(),
///     "uscrt".into(),
/// ];
/// let asset_decimals = vec![6, 6];
///
/// let pool_fees = PoolFee {
///     protocol_fee: Fee {
///         share: Decimal::percent(5u64),
///     },
///     swap_fee: Fee {
///         share: Decimal::percent(7u64),
///     },
///     burn_fee: Fee {
///         share: Decimal::zero(),
///     },
///    extra_fees: vec![],
/// };
///
/// let pool_type = PoolType::ConstantProduct;
/// let token_factory_lp = false;
///
/// let response = create_pool(deps, env, info, asset_infos, asset_decimals, pool_fees, pool_type, None)?;
/// # Ok(response)
/// # }
/// ```
#[allow(unreachable_code)]
#[allow(clippy::too_many_arguments)]
pub fn create_pool(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    asset_denoms: Vec<String>,
    asset_decimals: Vec<u8>,
    pool_fees: PoolFee,
    pool_type: PoolType,
    pool_identifier: Option<String>,
) -> Result<Response, ContractError> {
    // Load config for pool creation fee
    let config: Config = CONFIG.load(deps.storage)?;

    // Ensure that the number of assets and decimals match, and that they are not empty
    ensure!(
        !asset_denoms.is_empty()
            && asset_denoms.len() >= MIN_ASSETS_PER_POOL
            && asset_denoms.len() == asset_decimals.len(),
        ContractError::AssetMismatch
    );

    // Ensure that the number of assets is within the allowed range
    ensure!(
        asset_denoms.len() <= MAX_ASSETS_PER_POOL,
        ContractError::TooManyAssets {
            assets_provided: asset_denoms.len(),
        }
    );

    // check if the pool and token factory fees were paid
    let total_fees = validate_fees_are_paid(
        &config.pool_creation_fee,
        get_factory_denom_creation_fee(deps.as_ref())?,
        &info,
    )?;

    // make sure the user doesn't accidentally send more tokens than needed
    validate_no_additional_funds_sent_with_pool_creation(&info, total_fees)?;

    // Prepare the sending of pool creation fee
    let mut messages: Vec<CosmosMsg> = vec![];
    if !config.pool_creation_fee.amount.is_zero() {
        // send pool creation fee to the fee collector
        messages.push(
            BankMsg::Send {
                to_address: config.fee_collector_addr.to_string(),
                amount: vec![config.pool_creation_fee],
            }
            .into(),
        );
    }

    // Check if the asset infos are the same
    if asset_denoms
        .iter()
        .any(|asset| asset_denoms.iter().filter(|&a| a == asset).count() > 1)
    {
        return Err(ContractError::SameAsset); //what if two assets are same but one is in lowercase n one is in upper
    }

    // Verify pool fees
    pool_fees.is_valid()?;

    let identifier = if let Some(id) = pool_identifier {
        format!("{EXPLICIT_POOL_ID_PREFIX}{id}")
    } else {
        // if no identifier is provided, use the pool counter (id) as identifier
        let pool_counter =
            POOL_COUNTER.update(deps.storage, |mut counter| -> Result<_, ContractError> {
                counter += 1;
                Ok(counter)
            })?;
        format!("{AUTO_POOL_ID_PREFIX}{pool_counter}")
    };

    validate_pool_identifier(&identifier)?;

    // check if there is an existing pool with the given identifier
    let pool = get_pool_by_identifier(&deps.as_ref(), &identifier);
    if pool.is_ok() {
        return Err(ContractError::PoolExists {
            asset_infos: asset_denoms
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(", "),
            identifier,
        });
    }

    let mut attributes = Vec::<Attribute>::new();

    // Convert all asset_infos into assets with 0 balances
    let assets = asset_denoms
        .iter()
        .map(|asset_info| Coin {
            denom: asset_info.clone(),
            amount: Uint128::zero(),
        })
        .collect::<Vec<_>>();

    let lp_symbol = format!("{identifier}.{LP_SYMBOL}");
    let lp_asset = format!("{}/{}/{}", "factory", env.contract.address, lp_symbol);

    // sanity check for LP asset
    ensure!(
        is_factory_token(&lp_asset),
        ContractError::InvalidLpAsset {
            lp_asset: lp_asset.clone()
        }
    );

    #[allow(clippy::redundant_clone)]
    POOLS.save(
        deps.storage,
        &identifier,
        &PoolInfo {
            pool_identifier: identifier.clone(),
            asset_denoms,
            pool_type: pool_type.clone(),
            lp_denom: lp_asset.clone(),
            asset_decimals,
            pool_fees,
            assets,
        },
    )?;

    attributes.push(attr("lp_asset", lp_asset));

    messages.push(amm::tokenfactory::create_denom::create_denom(
        env.contract.address,
        lp_symbol,
    ));

    attributes.push(attr("action", "create_pool"));
    attributes.push(attr("pool_identifier", identifier.as_str()));
    attributes.push(attr("pool_type", pool_type.get_label()));

    Ok(Response::new()
        .add_attributes(attributes)
        .add_messages(messages))
}

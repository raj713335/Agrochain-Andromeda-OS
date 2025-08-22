use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo,
    Response, StdResult, Uint128, Addr, Decimal, Timestamp, CosmosMsg, WasmMsg, WasmQuery, QueryRequest,
    coin, coins, BankMsg, SubMsg, SubMsgResult, SubMsgResponse
};
use cw2::set_contract_version;
use cw721::{Cw721ExecuteMsg, Cw721QueryMsg, Cw721ReceiveMsg};
use cw20::{Cw20ExecuteMsg, Cw20QueryMsg, Cw20ReceiveMsg, Cw20Coin};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::ContractError;
use crate::msg::*;
use crate::state::*;
use crate::ai::*;
use crate::verification::*;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:carbon-credit-ado";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    
    let owner = deps.api.addr_validate(&msg.owner)?;
    OWNER.save(deps.storage, &owner)?;
    
    // Initialize AI model parameters
    let ai_config = AIConfig {
        model_version: msg.ai_model_version,
        carbon_factor: msg.carbon_factor,
        verification_threshold: msg.verification_threshold,
        update_frequency: msg.update_frequency,
    };
    AI_CONFIG.save(deps.storage, &ai_config)?;
    
    // Initialize marketplace settings
    let marketplace_config = MarketplaceConfig {
        fee_percentage: msg.fee_percentage,
        min_carbon_amount: msg.min_carbon_amount,
        max_carbon_amount: msg.max_carbon_amount,
        listing_duration: msg.listing_duration,
    };
    MARKETPLACE_CONFIG.save(deps.storage, &marketplace_config)?;
    
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", owner))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::RegisterFarmer { farmer_data } => execute_register_farmer(deps, env, info, farmer_data),
        ExecuteMsg::MintCarbonCredit { 
            carbon_amount, 
            verification_data, 
            farming_practices 
        } => execute_mint_carbon_credit(deps, env, info, carbon_amount, verification_data, farming_practices),
        ExecuteMsg::UpdateCarbonScore { credit_id, new_score } => {
            execute_update_carbon_score(deps, env, info, credit_id, new_score)
        },
        ExecuteMsg::ListCarbonCredit { credit_id, price, quantity } => {
            execute_list_carbon_credit(deps, env, info, credit_id, price, quantity)
        },
        ExecuteMsg::BuyCarbonCredit { listing_id } => execute_buy_carbon_credit(deps, env, info, listing_id),
        ExecuteMsg::StakeCarbonCredit { credit_id, amount } => {
            execute_stake_carbon_credit(deps, env, info, credit_id, amount)
        },
        ExecuteMsg::UnstakeCarbonCredit { credit_id, amount } => {
            execute_unstake_carbon_credit(deps, env, info, credit_id, amount)
        },
        ExecuteMsg::ProcessIOTData { iot_data } => execute_process_iot_data(deps, env, info, iot_data),
        ExecuteMsg::UpdateAIConfig { ai_config } => execute_update_ai_config(deps, env, info, ai_config),
        ExecuteMsg::CrossChainTransfer { 
            credit_id, 
            target_chain, 
            recipient, 
            amount 
        } => execute_cross_chain_transfer(deps, env, info, credit_id, target_chain, recipient, amount),
    }
}

pub fn execute_register_farmer(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    farmer_data: FarmerData,
) -> Result<Response, ContractError> {
    // Verify farmer data
    verify_farmer_data(&farmer_data)?;
    
    // Check if farmer already exists
    if FARMERS.has(deps.storage, &info.sender) {
        return Err(ContractError::FarmerAlreadyExists {});
    }
    
    // Create farmer profile
    let farmer = Farmer {
        address: info.sender.clone(),
        name: farmer_data.name,
        location: farmer_data.location,
        land_area: farmer_data.land_area,
        contact: farmer_data.contact,
        iot_device_id: farmer_data.iot_device_id,
        registration_date: _env.block.time,
        carbon_score: Decimal::zero(),
        total_credits_minted: Uint128::zero(),
        total_credits_sold: Uint128::zero(),
        farming_practices: farmer_data.farming_practices,
        verification_status: VerificationStatus::Pending,
    };
    
    FARMERS.save(deps.storage, &info.sender, &farmer)?;
    
    Ok(Response::new()
        .add_attribute("method", "register_farmer")
        .add_attribute("farmer", info.sender)
        .add_attribute("name", farmer_data.name))
}

pub fn execute_mint_carbon_credit(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    carbon_amount: Uint128,
    verification_data: VerificationData,
    farming_practices: Vec<String>,
) -> Result<Response, ContractError> {
    // Verify farmer exists
    let farmer = FARMERS.load(deps.storage, &info.sender)?;
    
    // Verify IoT data
    verify_iot_data(&verification_data)?;
    
    // Calculate AI-powered carbon score
    let ai_score = calculate_carbon_score(
        &carbon_amount,
        &verification_data,
        &farming_practices,
        &env.block.time,
    )?;
    
    // Create carbon credit
    let credit_id = generate_credit_id(&info.sender, &env.block.time);
    let carbon_credit = CarbonCredit {
        id: credit_id.clone(),
        farmer_address: info.sender.clone(),
        carbon_amount,
        verification_data: verification_data.clone(),
        ai_score,
        status: CreditStatus::Active,
        created_at: env.block.time,
        farming_practices,
        last_updated: env.block.time,
        cross_chain_data: None,
    };
    
    CARBON_CREDITS.save(deps.storage, &credit_id, &carbon_credit)?;
    
    // Update farmer stats
    let mut updated_farmer = farmer;
    updated_farmer.total_credits_minted += carbon_amount;
    updated_farmer.carbon_score = ai_score;
    FARMERS.save(deps.storage, &info.sender, &updated_farmer)?;
    
    // Mint CW20 tokens
    let mint_msg = Cw20ExecuteMsg::Mint {
        recipient: info.sender.to_string(),
        amount: carbon_amount,
    };
    
    let cw20_contract = CARBON_TOKEN.load(deps.storage)?;
    let sub_msg = SubMsg::new(WasmMsg::Execute {
        contract_addr: cw20_contract.to_string(),
        msg: to_binary(&mint_msg)?,
        funds: vec![],
    });
    
    Ok(Response::new()
        .add_submessage(sub_msg)
        .add_attribute("method", "mint_carbon_credit")
        .add_attribute("credit_id", credit_id)
        .add_attribute("farmer", info.sender)
        .add_attribute("carbon_amount", carbon_amount)
        .add_attribute("ai_score", ai_score.to_string()))
}

pub fn execute_list_carbon_credit(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    credit_id: String,
    price: Uint128,
    quantity: Uint128,
) -> Result<Response, ContractError> {
    // Verify carbon credit exists and belongs to seller
    let carbon_credit = CARBON_CREDITS.load(deps.storage, &credit_id)?;
    if carbon_credit.farmer_address != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    
    // Check if credit is available for listing
    if carbon_credit.status != CreditStatus::Active {
        return Err(ContractError::CreditNotAvailable {});
    }
    
    // Create listing
    let listing_id = generate_listing_id(&credit_id, &env.block.time);
    let listing = Listing {
        id: listing_id.clone(),
        carbon_credit_id: credit_id,
        seller: info.sender.clone(),
        price,
        quantity,
        status: ListingStatus::Active,
        created_at: env.block.time,
        expires_at: env.block.time.plus_seconds(MARKETPLACE_CONFIG.load(deps.storage)?.listing_duration),
    };
    
    LISTINGS.save(deps.storage, &listing_id, &listing)?;
    
    Ok(Response::new()
        .add_attribute("method", "list_carbon_credit")
        .add_attribute("listing_id", listing_id)
        .add_attribute("seller", info.sender)
        .add_attribute("price", price)
        .add_attribute("quantity", quantity))
}

pub fn execute_buy_carbon_credit(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    listing_id: String,
) -> Result<Response, ContractError> {
    let listing = LISTINGS.load(deps.storage, &listing_id)?;
    
    // Check if listing is still active
    if listing.status != ListingStatus::Active {
        return Err(ContractError::ListingNotActive {});
    }
    
    // Check if listing has expired
    if env.block.time >= listing.expires_at {
        return Err(ContractError::ListingExpired {});
    }
    
    // Calculate total cost
    let total_cost = listing.price * listing.quantity;
    
    // Check if buyer sent enough funds
    if info.funds.iter().find(|c| c.denom == "uandr").map_or(Uint128::zero(), |c| c.amount) < total_cost {
        return Err(ContractError::InsufficientFunds {});
    }
    
    // Transfer funds to seller (minus fees)
    let marketplace_config = MARKETPLACE_CONFIG.load(deps.storage)?;
    let fee_amount = total_cost * marketplace_config.fee_percentage / Uint128::from(100u128);
    let seller_amount = total_cost - fee_amount;
    
    let owner = OWNER.load(deps.storage)?;
    
    let messages: Vec<SubMsg> = vec![
        // Transfer to seller
        SubMsg::new(BankMsg::Send {
            to_address: listing.seller.to_string(),
            amount: coins(seller_amount.u128(), "uandr"),
        }),
        // Transfer fees to owner
        SubMsg::new(BankMsg::Send {
            to_address: owner.to_string(),
            amount: coins(fee_amount.u128(), "uandr"),
        }),
        // Transfer carbon credits to buyer
        SubMsg::new(WasmMsg::Execute {
            contract_addr: CARBON_TOKEN.load(deps.storage)?.to_string(),
            msg: to_binary(&Cw20ExecuteMsg::Transfer {
                recipient: info.sender.to_string(),
                amount: listing.quantity,
            })?,
            funds: vec![],
        }),
    ];
    
    // Update listing status
    let mut updated_listing = listing;
    updated_listing.status = ListingStatus::Sold;
    LISTINGS.save(deps.storage, &listing_id, &updated_listing)?;
    
    // Update farmer stats
    let mut farmer = FARMERS.load(deps.storage, &listing.seller)?;
    farmer.total_credits_sold += listing.quantity;
    FARMERS.save(deps.storage, &listing.seller, &farmer)?;
    
    Ok(Response::new()
        .add_submessages(messages)
        .add_attribute("method", "buy_carbon_credit")
        .add_attribute("listing_id", listing_id)
        .add_attribute("buyer", info.sender)
        .add_attribute("seller", listing.seller)
        .add_attribute("amount", listing.quantity)
        .add_attribute("price", total_cost))
}

pub fn execute_process_iot_data(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    iot_data: IOTData,
) -> Result<Response, ContractError> {
    // Verify IoT device is registered
    let device_owner = IOT_DEVICES.load(deps.storage, &iot_data.device_id)?;
    if device_owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    
    // Process and store IoT data
    let processed_data = process_sensor_data(&iot_data)?;
    IOT_DATA.save(deps.storage, &iot_data.device_id, &processed_data)?;
    
    // Update carbon scores for affected farmers
    let affected_farmers = get_farmers_by_device(deps.storage, &iot_data.device_id)?;
    for farmer_addr in affected_farmers {
        let mut farmer = FARMERS.load(deps.storage, &farmer_addr)?;
        let new_score = recalculate_carbon_score(&farmer, &processed_data)?;
        farmer.carbon_score = new_score;
        FARMERS.save(deps.storage, &farmer_addr, &farmer)?;
    }
    
    Ok(Response::new()
        .add_attribute("method", "process_iot_data")
        .add_attribute("device_id", iot_data.device_id)
        .add_attribute("timestamp", env.block.time.to_string()))
}

pub fn execute_cross_chain_transfer(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    credit_id: String,
    target_chain: String,
    recipient: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    // Verify carbon credit exists and belongs to sender
    let carbon_credit = CARBON_CREDITS.load(deps.storage, &credit_id)?;
    if carbon_credit.farmer_address != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    
    // Create cross-chain transfer data
    let transfer_data = CrossChainData {
        source_chain: env.contract.address.to_string(),
        target_chain,
        recipient,
        amount,
        transfer_id: generate_transfer_id(&credit_id, &env.block.time),
        status: TransferStatus::Pending,
        created_at: env.block.time,
    };
    
    // Update carbon credit with cross-chain data
    let mut updated_credit = carbon_credit;
    updated_credit.cross_chain_data = Some(transfer_data.clone());
    updated_credit.status = CreditStatus::Transferring;
    CARBON_CREDITS.save(deps.storage, &credit_id, &updated_credit)?;
    
    // Store transfer record
    CROSS_CHAIN_TRANSFERS.save(deps.storage, &transfer_data.transfer_id, &transfer_data)?;
    
    // Initiate IBC transfer (this would be handled by the IBC module)
    let ibc_msg = create_ibc_transfer_msg(&transfer_data)?;
    
    Ok(Response::new()
        .add_message(ibc_msg)
        .add_attribute("method", "cross_chain_transfer")
        .add_attribute("credit_id", credit_id)
        .add_attribute("target_chain", target_chain)
        .add_attribute("amount", amount)
        .add_attribute("transfer_id", transfer_data.transfer_id))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetFarmer { address } => to_binary(&query_farmer(deps, address)?),
        QueryMsg::GetCarbonCredit { credit_id } => to_binary(&query_carbon_credit(deps, credit_id)?),
        QueryMsg::GetListing { listing_id } => to_binary(&query_listing(deps, listing_id)?),
        QueryMsg::ListFarmers { start_after, limit } => to_binary(&query_list_farmers(deps, start_after, limit)?),
        QueryMsg::ListCarbonCredits { start_after, limit } => to_binary(&query_list_carbon_credits(deps, start_after, limit)?),
        QueryMsg::ListListings { start_after, limit } => to_binary(&query_list_listings(deps, start_after, limit)?),
        QueryMsg::GetAIConfig {} => to_binary(&query_ai_config(deps)?),
        QueryMsg::GetMarketplaceConfig {} => to_binary(&query_marketplace_config(deps)?),
        QueryMsg::GetCarbonScore { farmer_address } => to_binary(&query_carbon_score(deps, farmer_address)?),
        QueryMsg::GetIOTData { device_id } => to_binary(&query_iot_data(deps, device_id)?),
        QueryMsg::GetCrossChainTransfer { transfer_id } => to_binary(&query_cross_chain_transfer(deps, transfer_id)?),
    }
}

fn query_farmer(deps: Deps, address: String) -> StdResult<Farmer> {
    let addr = deps.api.addr_validate(&address)?;
    FARMERS.load(deps.storage, &addr)
}

fn query_carbon_credit(deps: Deps, credit_id: String) -> StdResult<CarbonCredit> {
    CARBON_CREDITS.load(deps.storage, &credit_id)
}

fn query_listing(deps: Deps, listing_id: String) -> StdResult<Listing> {
    LISTINGS.load(deps.storage, &listing_id)
}

fn query_list_farmers(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<Vec<Farmer>> {
    let limit = limit.unwrap_or(30) as usize;
    let start = start_after.map(|addr| deps.api.addr_validate(&addr)).transpose()?;
    
    let farmers: StdResult<Vec<_>> = FARMERS
        .range(deps.storage, start.as_deref(), None, cosmwasm_std::Order::Ascending)
        .take(limit)
        .collect();
    
    farmers
}

fn query_list_carbon_credits(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<Vec<CarbonCredit>> {
    let limit = limit.unwrap_or(30) as usize;
    let start = start_after.as_deref();
    
    let credits: StdResult<Vec<_>> = CARBON_CREDITS
        .range(deps.storage, start, None, cosmwasm_std::Order::Ascending)
        .take(limit)
        .collect();
    
    credits
}

fn query_list_listings(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<Vec<Listing>> {
    let limit = limit.unwrap_or(30) as usize;
    let start = start_after.as_deref();
    
    let listings: StdResult<Vec<_>> = LISTINGS
        .range(deps.storage, start, None, cosmwasm_std::Order::Ascending)
        .take(limit)
        .collect();
    
    listings
}

fn query_ai_config(deps: Deps) -> StdResult<AIConfig> {
    AI_CONFIG.load(deps.storage)
}

fn query_marketplace_config(deps: Deps) -> StdResult<MarketplaceConfig> {
    MARKETPLACE_CONFIG.load(deps.storage)
}

fn query_carbon_score(deps: Deps, farmer_address: String) -> StdResult<Decimal> {
    let addr = deps.api.addr_validate(&farmer_address)?;
    let farmer = FARMERS.load(deps.storage, &addr)?;
    Ok(farmer.carbon_score)
}

fn query_iot_data(deps: Deps, device_id: String) -> StdResult<ProcessedIOTData> {
    IOT_DATA.load(deps.storage, &device_id)
}

fn query_cross_chain_transfer(deps: Deps, transfer_id: String) -> StdResult<CrossChainData> {
    CROSS_CHAIN_TRANSFERS.load(deps.storage, &transfer_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{from_binary, Addr};

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);
        
        let msg = InstantiateMsg {
            owner: "creator".to_string(),
            ai_model_version: "v1.0".to_string(),
            carbon_factor: Decimal::from_ratio(1u128, 100u128),
            verification_threshold: Decimal::from_ratio(80u128, 100u128),
            update_frequency: 3600,
            fee_percentage: Uint128::from(2u128),
            min_carbon_amount: Uint128::from(100u128),
            max_carbon_amount: Uint128::from(1000000u128),
            listing_duration: 86400,
        };
        
        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn test_register_farmer() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("farmer", &[]);
        
        // First instantiate
        let instantiate_msg = InstantiateMsg {
            owner: "creator".to_string(),
            ai_model_version: "v1.0".to_string(),
            carbon_factor: Decimal::from_ratio(1u128, 100u128),
            verification_threshold: Decimal::from_ratio(80u128, 100u128),
            update_frequency: 3600,
            fee_percentage: Uint128::from(2u128),
            min_carbon_amount: Uint128::from(100u128),
            max_carbon_amount: Uint128::from(1000000u128),
            listing_duration: 86400,
        };
        instantiate(deps.as_mut(), env.clone(), mock_info("creator", &[]), instantiate_msg).unwrap();
        
        // Then register farmer
        let farmer_data = FarmerData {
            name: "John Doe".to_string(),
            location: "California, USA".to_string(),
            land_area: "100 acres".to_string(),
            contact: "john@example.com".to_string(),
            iot_device_id: "device_001".to_string(),
            farming_practices: vec!["organic".to_string(), "sustainable".to_string()],
        };
        
        let msg = ExecuteMsg::RegisterFarmer { farmer_data };
        let res = execute(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }
}

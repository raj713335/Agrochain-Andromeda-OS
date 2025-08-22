use cosmwasm_std::{Addr, Uint128, Decimal};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::*;

// ============================================================================
// Instantiate Message
// ============================================================================

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub owner: String,
    pub ai_model_version: String,
    pub carbon_factor: Decimal,
    pub verification_threshold: Decimal,
    pub update_frequency: u64,
    pub fee_percentage: Uint128,
    pub min_carbon_amount: Uint128,
    pub max_carbon_amount: Uint128,
    pub listing_duration: u64,
}

// ============================================================================
// Execute Messages
// ============================================================================

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    // Farmer Management
    RegisterFarmer {
        farmer_data: FarmerData,
    },
    
    // Carbon Credit Management
    MintCarbonCredit {
        carbon_amount: Uint128,
        verification_data: VerificationData,
        farming_practices: Vec<String>,
    },
    
    UpdateCarbonScore {
        credit_id: String,
        new_score: Decimal,
    },
    
    // Marketplace Operations
    ListCarbonCredit {
        credit_id: String,
        price: Uint128,
        quantity: Uint128,
    },
    
    BuyCarbonCredit {
        listing_id: String,
    },
    
    // Staking Operations
    StakeCarbonCredit {
        credit_id: String,
        amount: Uint128,
    },
    
    UnstakeCarbonCredit {
        credit_id: String,
        amount: Uint128,
    },
    
    // IoT Data Processing
    ProcessIOTData {
        iot_data: IOTData,
    },
    
    // Configuration Updates
    UpdateAIConfig {
        ai_config: AIConfig,
    },
    
    // Cross-chain Operations
    CrossChainTransfer {
        credit_id: String,
        target_chain: String,
        recipient: String,
        amount: Uint128,
    },
    
    // Auction Operations
    CreateAuction {
        credit_id: String,
        starting_price: Uint128,
        reserve_price: Uint128,
        quantity: Uint128,
        duration: u64,
    },
    
    PlaceBid {
        auction_id: String,
        amount: Uint128,
    },
    
    EndAuction {
        auction_id: String,
    },
    
    // Vesting Operations
    CreateVesting {
        credit_id: String,
        beneficiary: String,
        amount: Uint128,
        start_time: u64,
        duration: u64,
    },
    
    ClaimVesting {
        vesting_id: String,
    },
    
    // Merkle Airdrop Operations
    CreateAirdrop {
        merkle_root: String,
        total_amount: Uint128,
        start_time: u64,
        end_time: u64,
    },
    
    ClaimAirdrop {
        airdrop_id: String,
        amount: Uint128,
        proof: Vec<String>,
    },
    
    // Splitter Operations
    CreateSplitter {
        recipients: Vec<SplitterRecipient>,
        total_amount: Uint128,
    },
    
    DistributeSplitter {
        splitter_id: String,
    },
    
    // Emergency Operations
    EmergencyPause {},
    
    EmergencyResume {},
    
    EmergencyWithdraw {
        credit_id: String,
        recipient: String,
    },
}

// ============================================================================
// Query Messages
// ============================================================================

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // Farmer Queries
    GetFarmer {
        address: String,
    },
    
    ListFarmers {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    
    // Carbon Credit Queries
    GetCarbonCredit {
        credit_id: String,
    },
    
    ListCarbonCredits {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    
    GetCarbonCreditsByFarmer {
        farmer_address: String,
    },
    
    // Marketplace Queries
    GetListing {
        listing_id: String,
    },
    
    ListListings {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    
    GetActiveListings {},
    
    // Staking Queries
    GetStakingPosition {
        staker: String,
        credit_id: String,
    },
    
    ListStakingPositions {
        staker: String,
    },
    
    // Auction Queries
    GetAuction {
        auction_id: String,
    },
    
    ListAuctions {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    
    GetActiveAuctions {},
    
    // Configuration Queries
    GetAIConfig {},
    
    GetMarketplaceConfig {},
    
    // Statistics Queries
    GetCarbonScore {
        farmer_address: String,
    },
    
    GetGlobalStats {},
    
    GetFarmerStats {
        farmer_address: String,
    },
    
    // IoT Data Queries
    GetIOTData {
        device_id: String,
    },
    
    ListIOTDevices {},
    
    // Cross-chain Queries
    GetCrossChainTransfer {
        transfer_id: String,
    },
    
    ListCrossChainTransfers {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    
    // Vesting Queries
    GetVesting {
        vesting_id: String,
    },
    
    ListVestings {
        beneficiary: String,
    },
    
    // Airdrop Queries
    GetAirdrop {
        airdrop_id: String,
    },
    
    ListAirdrops {},
    
    CheckAirdropEligibility {
        airdrop_id: String,
        address: String,
        amount: Uint128,
        proof: Vec<String>,
    },
    
    // Splitter Queries
    GetSplitter {
        splitter_id: String,
    },
    
    ListSplitters {
        creator: String,
    },
    
    // Contract Info
    GetContractInfo {},
    
    GetOwner {},
}

// ============================================================================
// Response Messages
// ============================================================================

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct FarmerData {
    pub name: String,
    pub location: String,
    pub land_area: String,
    pub contact: String,
    pub iot_device_id: String,
    pub farming_practices: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SplitterRecipient {
    pub address: String,
    pub percentage: Decimal,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct FarmerResponse {
    pub farmer: Farmer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct FarmersResponse {
    pub farmers: Vec<Farmer>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CarbonCreditResponse {
    pub carbon_credit: CarbonCredit,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CarbonCreditsResponse {
    pub carbon_credits: Vec<CarbonCredit>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ListingResponse {
    pub listing: Listing,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ListingsResponse {
    pub listings: Vec<Listing>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StakingPositionResponse {
    pub staking_position: StakingPosition,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StakingPositionsResponse {
    pub staking_positions: Vec<StakingPosition>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AuctionResponse {
    pub auction: Auction,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AuctionsResponse {
    pub auctions: Vec<Auction>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AIConfigResponse {
    pub ai_config: AIConfig,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MarketplaceConfigResponse {
    pub marketplace_config: MarketplaceConfig,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CarbonScoreResponse {
    pub carbon_score: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GlobalStatsResponse {
    pub stats: CarbonCreditStats,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct FarmerStatsResponse {
    pub farmer: Farmer,
    pub total_credits_minted: Uint128,
    pub total_credits_sold: Uint128,
    pub total_credits_staked: Uint128,
    pub average_price: Uint128,
    pub carbon_score: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct IOTDataResponse {
    pub iot_data: ProcessedIOTData,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct IOTDevicesResponse {
    pub devices: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CrossChainTransferResponse {
    pub transfer: CrossChainData,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CrossChainTransfersResponse {
    pub transfers: Vec<CrossChainData>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VestingResponse {
    pub vesting: VestingData,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VestingsResponse {
    pub vestings: Vec<VestingData>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AirdropResponse {
    pub airdrop: AirdropData,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AirdropsResponse {
    pub airdrops: Vec<AirdropData>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AirdropEligibilityResponse {
    pub eligible: bool,
    pub amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SplitterResponse {
    pub splitter: SplitterData,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SplittersResponse {
    pub splitters: Vec<SplitterData>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ContractInfoResponse {
    pub name: String,
    pub version: String,
    pub owner: String,
    pub carbon_token: String,
    pub nft_contract: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OwnerResponse {
    pub owner: String,
}

// ============================================================================
// Additional Data Structures
// ============================================================================

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VestingData {
    pub id: String,
    pub beneficiary: Addr,
    pub credit_id: String,
    pub total_amount: Uint128,
    pub claimed_amount: Uint128,
    pub start_time: u64,
    pub end_time: u64,
    pub created_at: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AirdropData {
    pub id: String,
    pub merkle_root: String,
    pub total_amount: Uint128,
    pub claimed_amount: Uint128,
    pub start_time: u64,
    pub end_time: u64,
    pub created_at: u64,
    pub creator: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SplitterData {
    pub id: String,
    pub creator: Addr,
    pub recipients: Vec<SplitterRecipient>,
    pub total_amount: Uint128,
    pub distributed_amount: Uint128,
    pub created_at: u64,
    pub status: SplitterStatus,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum SplitterStatus {
    Active,
    Paused,
    Completed,
    Cancelled,
}

// ============================================================================
// Migration Messages
// ============================================================================

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {
    pub version: String,
}

// ============================================================================
// Event Messages
// ============================================================================

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct EventData {
    pub event_type: String,
    pub data: serde_json::Value,
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct EventsResponse {
    pub events: Vec<EventData>,
}

// ============================================================================
// Pagination Support
// ============================================================================

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Pagination {
    pub start_after: Option<String>,
    pub limit: Option<u32>,
}

// ============================================================================
// Filter Support
// ============================================================================

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Filter {
    pub field: String,
    pub operator: FilterOperator,
    pub value: serde_json::Value,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum FilterOperator {
    Eq,
    Ne,
    Gt,
    Gte,
    Lt,
    Lte,
    Contains,
    In,
    NotIn,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct QueryWithFilters {
    pub pagination: Option<Pagination>,
    pub filters: Option<Vec<Filter>>,
    pub sort_by: Option<String>,
    pub sort_order: Option<SortOrder>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum SortOrder {
    Asc,
    Desc,
}

// ============================================================================
// Batch Operations
// ============================================================================

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BatchMintRequest {
    pub mints: Vec<MintRequest>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MintRequest {
    pub carbon_amount: Uint128,
    pub verification_data: VerificationData,
    pub farming_practices: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BatchMintResponse {
    pub credit_ids: Vec<String>,
    pub success_count: u32,
    pub failure_count: u32,
    pub errors: Vec<String>,
}

// ============================================================================
// Analytics and Reporting
// ============================================================================

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AnalyticsRequest {
    pub start_date: u64,
    pub end_date: u64,
    pub metrics: Vec<String>,
    pub group_by: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AnalyticsResponse {
    pub data: Vec<AnalyticsDataPoint>,
    pub summary: AnalyticsSummary,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AnalyticsDataPoint {
    pub timestamp: u64,
    pub metrics: std::collections::HashMap<String, f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AnalyticsSummary {
    pub total_credits_minted: Uint128,
    pub total_credits_sold: Uint128,
    pub total_volume: Uint128,
    pub average_price: Uint128,
    pub active_farmers: u32,
    pub average_carbon_score: Decimal,
}

// ============================================================================
// Webhook and Integration Support
// ============================================================================

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct WebhookConfig {
    pub url: String,
    pub events: Vec<String>,
    pub secret: String,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct WebhookEvent {
    pub event_type: String,
    pub data: serde_json::Value,
    pub timestamp: u64,
    pub signature: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct IntegrationConfig {
    pub name: String,
    pub config: serde_json::Value,
    pub enabled: bool,
}

// ============================================================================
// Error Response
// ============================================================================

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ErrorResponse {
    pub error: String,
    pub code: u32,
    pub details: Option<String>,
}

// ============================================================================
// Success Response
// ============================================================================

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SuccessResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

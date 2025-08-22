use cosmwasm_std::{Addr, Uint128, Decimal, Timestamp};
use cw_storage_plus::{Item, Map, Index, IndexList, IndexedMap, MultiIndex};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ============================================================================
// Data Structures
// ============================================================================

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Farmer {
    pub address: Addr,
    pub name: String,
    pub location: String,
    pub land_area: String,
    pub contact: String,
    pub iot_device_id: String,
    pub registration_date: Timestamp,
    pub carbon_score: Decimal,
    pub total_credits_minted: Uint128,
    pub total_credits_sold: Uint128,
    pub farming_practices: Vec<String>,
    pub verification_status: VerificationStatus,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CarbonCredit {
    pub id: String,
    pub farmer_address: Addr,
    pub carbon_amount: Uint128,
    pub verification_data: VerificationData,
    pub ai_score: Decimal,
    pub status: CreditStatus,
    pub created_at: Timestamp,
    pub farming_practices: Vec<String>,
    pub last_updated: Timestamp,
    pub cross_chain_data: Option<CrossChainData>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VerificationData {
    pub iot_device_id: String,
    pub sensor_readings: Vec<SensorReading>,
    pub location: String,
    pub farming_practices: Vec<String>,
    pub verification_timestamp: Timestamp,
    pub verification_score: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SensorReading {
    pub sensor_type: SensorType,
    pub value: f64,
    pub unit: String,
    pub timestamp: Timestamp,
    pub location: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum SensorType {
    Temperature,
    Humidity,
    SoilMoisture,
    CO2Level,
    NitrogenLevel,
    PhosphorusLevel,
    PotassiumLevel,
    PHLevel,
    Rainfall,
    WindSpeed,
    SolarRadiation,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Listing {
    pub id: String,
    pub carbon_credit_id: String,
    pub seller: Addr,
    pub price: Uint128,
    pub quantity: Uint128,
    pub status: ListingStatus,
    pub created_at: Timestamp,
    pub expires_at: Timestamp,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AIConfig {
    pub model_version: String,
    pub carbon_factor: Decimal,
    pub verification_threshold: Decimal,
    pub update_frequency: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MarketplaceConfig {
    pub fee_percentage: Uint128,
    pub min_carbon_amount: Uint128,
    pub max_carbon_amount: Uint128,
    pub listing_duration: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct IOTData {
    pub device_id: String,
    pub sensor_readings: Vec<SensorReading>,
    pub timestamp: Timestamp,
    pub location: String,
    pub device_status: DeviceStatus,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ProcessedIOTData {
    pub device_id: String,
    pub processed_readings: Vec<ProcessedReading>,
    pub carbon_impact_score: Decimal,
    pub sustainability_score: Decimal,
    pub processing_timestamp: Timestamp,
    pub ai_confidence: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ProcessedReading {
    pub original_reading: SensorReading,
    pub processed_value: f64,
    pub confidence: Decimal,
    pub anomaly_detected: bool,
    pub carbon_contribution: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CrossChainData {
    pub source_chain: String,
    pub target_chain: String,
    pub recipient: String,
    pub amount: Uint128,
    pub transfer_id: String,
    pub status: TransferStatus,
    pub created_at: Timestamp,
    pub completed_at: Option<Timestamp>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StakingPosition {
    pub staker: Addr,
    pub credit_id: String,
    pub amount: Uint128,
    pub staked_at: Timestamp,
    pub rewards_earned: Uint128,
    pub last_claim: Timestamp,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Auction {
    pub id: String,
    pub carbon_credit_id: String,
    pub seller: Addr,
    pub starting_price: Uint128,
    pub reserve_price: Uint128,
    pub quantity: Uint128,
    pub start_time: Timestamp,
    pub end_time: Timestamp,
    pub status: AuctionStatus,
    pub highest_bid: Option<Bid>,
    pub bids: Vec<Bid>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Bid {
    pub bidder: Addr,
    pub amount: Uint128,
    pub timestamp: Timestamp,
}

// ============================================================================
// Enums
// ============================================================================

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum CreditStatus {
    Active,
    Listed,
    Sold,
    Staked,
    Transferring,
    Expired,
    Retired,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ListingStatus {
    Active,
    Sold,
    Cancelled,
    Expired,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum VerificationStatus {
    Pending,
    Verified,
    Rejected,
    UnderReview,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum DeviceStatus {
    Online,
    Offline,
    Maintenance,
    Error,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum TransferStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum AuctionStatus {
    Scheduled,
    Active,
    Ended,
    Cancelled,
    Settled,
}

// ============================================================================
// Storage Keys
// ============================================================================

pub const OWNER: Item<Addr> = Item::new("owner");
pub const AI_CONFIG: Item<AIConfig> = Item::new("ai_config");
pub const MARKETPLACE_CONFIG: Item<MarketplaceConfig> = Item::new("marketplace_config");
pub const CARBON_TOKEN: Item<Addr> = Item::new("carbon_token");
pub const NFT_CONTRACT: Item<Addr> = Item::new("nft_contract");

// ============================================================================
// Storage Maps
// ============================================================================

pub const FARMERS: Map<&Addr, Farmer> = Map::new("farmers");
pub const CARBON_CREDITS: Map<&str, CarbonCredit> = Map::new("carbon_credits");
pub const LISTINGS: Map<&str, Listing> = Map::new("listings");
pub const IOT_DEVICES: Map<&str, Addr> = Map::new("iot_devices");
pub const IOT_DATA: Map<&str, ProcessedIOTData> = Map::new("iot_data");
pub const CROSS_CHAIN_TRANSFERS: Map<&str, CrossChainData> = Map::new("cross_chain_transfers");
pub const STAKING_POSITIONS: Map<(&Addr, &str), StakingPosition> = Map::new("staking_positions");
pub const AUCTIONS: Map<&str, Auction> = Map::new("auctions");

// ============================================================================
// Indexed Maps for Complex Queries
// ============================================================================

pub struct CarbonCreditIndexes<'a> {
    pub farmer: MultiIndex<'a, Addr, CarbonCredit, &'a str>,
    pub status: MultiIndex<'a, CreditStatus, CarbonCredit, &'a str>,
    pub created_at: MultiIndex<'a, Timestamp, CarbonCredit, &'a str>,
}

impl<'a> IndexList<CarbonCredit> for CarbonCreditIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<CarbonCredit>> + '_> {
        let v: Vec<&dyn Index<CarbonCredit>> = vec![
            &self.farmer,
            &self.status,
            &self.created_at,
        ];
        Box::new(v.into_iter())
    }
}

pub fn carbon_credits<'a>() -> IndexedMap<'a, &'a str, CarbonCredit, CarbonCreditIndexes<'a>> {
    let indexes = CarbonCreditIndexes {
        farmer: MultiIndex::new(|d| d.farmer_address.clone(), "carbon_credits", "carbon_credits__farmer"),
        status: MultiIndex::new(|d| d.status.clone(), "carbon_credits", "carbon_credits__status"),
        created_at: MultiIndex::new(|d| d.created_at, "carbon_credits", "carbon_credits__created_at"),
    };
    IndexedMap::new("carbon_credits", indexes)
}

pub struct ListingIndexes<'a> {
    pub seller: MultiIndex<'a, Addr, Listing, &'a str>,
    pub status: MultiIndex<'a, ListingStatus, Listing, &'a str>,
    pub expires_at: MultiIndex<'a, Timestamp, Listing, &'a str>,
}

impl<'a> IndexList<Listing> for ListingIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Listing>> + '_> {
        let v: Vec<&dyn Index<Listing>> = vec![
            &self.seller,
            &self.status,
            &self.expires_at,
        ];
        Box::new(v.into_iter())
    }
}

pub fn listings<'a>() -> IndexedMap<'a, &'a str, Listing, ListingIndexes<'a>> {
    let indexes = ListingIndexes {
        seller: MultiIndex::new(|d| d.seller.clone(), "listings", "listings__seller"),
        status: MultiIndex::new(|d| d.status.clone(), "listings", "listings__status"),
        expires_at: MultiIndex::new(|d| d.expires_at, "listings", "listings__expires_at"),
    };
    IndexedMap::new("listings", indexes)
}

pub struct AuctionIndexes<'a> {
    pub seller: MultiIndex<'a, Addr, Auction, &'a str>,
    pub status: MultiIndex<'a, AuctionStatus, Auction, &'a str>,
    pub end_time: MultiIndex<'a, Timestamp, Auction, &'a str>,
}

impl<'a> IndexList<Auction> for AuctionIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Auction>> + '_> {
        let v: Vec<&dyn Index<Auction>> = vec![
            &self.seller,
            &self.status,
            &self.end_time,
        ];
        Box::new(v.into_iter())
    }
}

pub fn auctions<'a>() -> IndexedMap<'a, &'a str, Auction, AuctionIndexes<'a>> {
    let indexes = AuctionIndexes {
        seller: MultiIndex::new(|d| d.seller.clone(), "auctions", "auctions__seller"),
        status: MultiIndex::new(|d| d.status.clone(), "auctions", "auctions__status"),
        end_time: MultiIndex::new(|d| d.end_time, "auctions", "auctions__end_time"),
    };
    IndexedMap::new("auctions", indexes)
}

// ============================================================================
// Helper Functions
// ============================================================================

pub fn generate_credit_id(farmer: &Addr, timestamp: &Timestamp) -> String {
    format!("credit_{}_{}", farmer, timestamp.seconds())
}

pub fn generate_listing_id(credit_id: &str, timestamp: &Timestamp) -> String {
    format!("listing_{}_{}", credit_id, timestamp.seconds())
}

pub fn generate_transfer_id(credit_id: &str, timestamp: &Timestamp) -> String {
    format!("transfer_{}_{}", credit_id, timestamp.seconds())
}

pub fn generate_auction_id(credit_id: &str, timestamp: &Timestamp) -> String {
    format!("auction_{}_{}", credit_id, timestamp.seconds())
}

// ============================================================================
// Query Helpers
// ============================================================================

pub fn get_farmers_by_device(storage: &dyn cosmwasm_std::Storage, device_id: &str) -> cosmwasm_std::StdResult<Vec<Addr>> {
    let mut farmers = Vec::new();
    let range = FARMERS.range(storage, None, None, cosmwasm_std::Order::Ascending);
    
    for item in range {
        let (addr, farmer) = item?;
        if farmer.iot_device_id == device_id {
            farmers.push(addr);
        }
    }
    
    Ok(farmers)
}

pub fn get_active_listings(storage: &dyn cosmwasm_std::Storage) -> cosmwasm_std::StdResult<Vec<Listing>> {
    let mut active_listings = Vec::new();
    let range = LISTINGS.range(storage, None, None, cosmwasm_std::Order::Ascending);
    
    for item in range {
        let (_, listing) = item?;
        if listing.status == ListingStatus::Active {
            active_listings.push(listing);
        }
    }
    
    Ok(active_listings)
}

pub fn get_carbon_credits_by_farmer(storage: &dyn cosmwasm_std::Storage, farmer: &Addr) -> cosmwasm_std::StdResult<Vec<CarbonCredit>> {
    let mut credits = Vec::new();
    let range = CARBON_CREDITS.range(storage, None, None, cosmwasm_std::Order::Ascending);
    
    for item in range {
        let (_, credit) = item?;
        if credit.farmer_address == *farmer {
            credits.push(credit);
        }
    }
    
    Ok(credits)
}

pub fn get_staking_positions_by_staker(storage: &dyn cosmwasm_std::Storage, staker: &Addr) -> cosmwasm_std::StdResult<Vec<StakingPosition>> {
    let mut positions = Vec::new();
    let range = STAKING_POSITIONS.range(storage, None, None, cosmwasm_std::Order::Ascending);
    
    for item in range {
        let ((staker_addr, _), position) = item?;
        if staker_addr == *staker {
            positions.push(position);
        }
    }
    
    Ok(positions)
}

// ============================================================================
// Statistics and Analytics
// ============================================================================

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CarbonCreditStats {
    pub total_credits_minted: Uint128,
    pub total_credits_sold: Uint128,
    pub total_credits_staked: Uint128,
    pub average_carbon_score: Decimal,
    pub total_farmers: u64,
    pub active_listings: u64,
    pub total_volume: Uint128,
}

pub fn calculate_global_stats(storage: &dyn cosmwasm_std::Storage) -> cosmwasm_std::StdResult<CarbonCreditStats> {
    let mut stats = CarbonCreditStats {
        total_credits_minted: Uint128::zero(),
        total_credits_sold: Uint128::zero(),
        total_credits_staked: Uint128::zero(),
        average_carbon_score: Decimal::zero(),
        total_farmers: 0,
        active_listings: 0,
        total_volume: Uint128::zero(),
    };
    
    // Calculate farmer stats
    let farmer_range = FARMERS.range(storage, None, None, cosmwasm_std::Order::Ascending);
    let mut total_score = Decimal::zero();
    let mut farmer_count = 0u64;
    
    for item in farmer_range {
        let (_, farmer) = item?;
        stats.total_credits_minted += farmer.total_credits_minted;
        stats.total_credits_sold += farmer.total_credits_sold;
        total_score += farmer.carbon_score;
        farmer_count += 1;
    }
    
    stats.total_farmers = farmer_count;
    if farmer_count > 0 {
        stats.average_carbon_score = total_score / Decimal::from_ratio(farmer_count, 1u128);
    }
    
    // Calculate listing stats
    let listing_range = LISTINGS.range(storage, None, None, cosmwasm_std::Order::Ascending);
    for item in listing_range {
        let (_, listing) = item?;
        if listing.status == ListingStatus::Active {
            stats.active_listings += 1;
        }
        if listing.status == ListingStatus::Sold {
            stats.total_volume += listing.price * listing.quantity;
        }
    }
    
    // Calculate staking stats
    let staking_range = STAKING_POSITIONS.range(storage, None, None, cosmwasm_std::Order::Ascending);
    for item in staking_range {
        let (_, position) = item?;
        stats.total_credits_staked += position.amount;
    }
    
    Ok(stats)
}

# AgroChain on Andromeda's Operating System (aOS)

## Overview

AgroChain has been migrated to Andromeda's Operating System (aOS) to leverage cross-chain capabilities, modular smart contracts (ADOs), and AI-native infrastructure for carbon credit tokenization and trading.

## Key Migration Changes

### From Ethereum to Andromeda aOS
- **Smart Contracts → ADOs**: Replaced Solidity contracts with Andromeda Application-Specific Digital Objects
- **Ethereum → Cosmos**: Migrated from Ethereum to Cosmos ecosystem for cross-chain interoperability
- **Ethers.js → CosmJS**: Updated frontend to use CosmJS for Cosmos blockchain interaction
- **Hardhat → Andromeda CLI**: Switched to Andromeda's development tools

### Enhanced Features
- **Cross-chain Carbon Credits**: Carbon credits can now be traded across multiple Cosmos chains
- **AI-Powered Carbon Scoring**: Integrated machine learning for accurate carbon emission prediction
- **Modular Architecture**: Using Andromeda's ADO framework for composable smart contracts
- **Real World Asset Tokenization**: Carbon credits as RWAs with fractional ownership capabilities

## Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Frontend      │    │   Andromeda     │    │   AI Services   │
│   (React +      │◄──►│   aOS           │◄──►│   (Carbon       │
│   CosmJS)       │    │   (ADOs)        │    │   Prediction)   │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Keplr Wallet  │    │   Cosmos Hub    │    │   IoT Devices   │
│   Integration   │    │   (Cross-chain) │    │   (Verification)│
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

## ADOs Used

### Core ADOs
- **CW20**: Carbon Credit Tokens
- **CW721**: Carbon Credit NFTs
- **Marketplace**: Carbon credit trading platform
- **Auction**: Bidding system for carbon credits
- **Vesting**: Gradual release of carbon credits
- **Merkle Airdrop**: Distribution of carbon credits

### Advanced ADOs
- **Conditional Splitter**: Revenue sharing between farmers and investors
- **Weighted Distribution Splitter**: Fair distribution based on carbon scores
- **CW20 Staking**: Staking carbon credits for additional rewards

## Installation & Setup

### Prerequisites
- Node.js 18+
- Andromeda CLI
- Keplr Wallet
- Go 1.19+ (for ADO compilation)

### 1. Install Andromeda CLI
```bash
curl -sSfL https://raw.githubusercontent.com/andromedaprotocol/andromeda/main/scripts/install.sh | sh
```

### 2. Clone Repository
```bash
git clone <repository-url>
cd agrochain-aos
```

### 3. Install Dependencies
```bash
npm install
```

### 4. Configure Andromeda
```bash
andromeda config init
andromeda config set chain-id andromeda-1
andromeda config set rpc-url https://rpc.andromeda-1.andromeda.zone
```

### 5. Deploy ADOs
```bash
andromeda ado deploy --path ./ados/carbon-credit
andromeda ado deploy --path ./ados/marketplace
andromeda ado deploy --path ./ados/auction
```

### 6. Start Frontend
```bash
npm start
```

## Features

### For Farmers
- **Carbon Credit Minting**: Mint NFTs representing carbon credits based on sustainable practices
- **IoT Verification**: Automatic verification through connected IoT devices
- **Cross-chain Trading**: Sell carbon credits across multiple Cosmos chains
- **Revenue Sharing**: Earn from carbon credit sales and staking rewards

### For Industries/Investors
- **Carbon Credit Purchase**: Buy verified carbon credits to meet ESG goals
- **Fractional Ownership**: Invest in portions of carbon credit portfolios
- **Cross-chain Access**: Access carbon credits from multiple chains
- **AI-Powered Valuation**: Get AI-driven carbon credit valuations

### For the Ecosystem
- **Cross-chain Liquidity**: Carbon credits can be traded across Cosmos ecosystem
- **Modular Architecture**: Easy to extend and compose new features
- **AI Integration**: Machine learning for carbon prediction and scoring
- **Real World Asset Bridge**: Physical carbon credits tokenized on blockchain

## Smart Contract Architecture (ADOs)

### Carbon Credit ADO
```rust
#[cw_serde]
pub struct CarbonCredit {
    pub id: String,
    pub farmer_address: Addr,
    pub carbon_amount: Uint128,
    pub verification_data: VerificationData,
    pub ai_score: Decimal,
    pub status: CreditStatus,
    pub created_at: u64,
}

#[cw_serde]
pub struct VerificationData {
    pub iot_device_id: String,
    pub sensor_readings: Vec<SensorReading>,
    pub location: String,
    pub farming_practices: Vec<String>,
}
```

### Marketplace ADO
```rust
#[cw_serde]
pub struct Listing {
    pub id: String,
    pub carbon_credit_id: String,
    pub seller: Addr,
    pub price: Uint128,
    pub quantity: Uint128,
    pub status: ListingStatus,
}
```

## AI Integration

### Carbon Emission Prediction
- **Crop Type Analysis**: Predict emissions based on crop type and farming practices
- **Water Usage Optimization**: AI models for irrigation efficiency
- **Yield Prediction**: Machine learning for crop yield and carbon sequestration
- **Real-time Scoring**: Dynamic carbon credit scoring based on IoT data

### AI Services
- **Carbon Score Calculation**: Real-time carbon credit scoring
- **Market Price Prediction**: AI-driven pricing for carbon credits
- **Risk Assessment**: Automated risk evaluation for carbon credit investments
- **Compliance Monitoring**: AI-powered regulatory compliance checking

## Cross-chain Capabilities

### Supported Chains
- **Andromeda**: Primary chain for carbon credit operations
- **Cosmos Hub**: Cross-chain carbon credit trading
- **Osmosis**: Liquidity provision for carbon credits
- **Juno**: Additional trading venue
- **Evmos**: EVM-compatible carbon credit access

### Interchain Features
- **IBC Transfers**: Carbon credits can be transferred between chains
- **Cross-chain Auctions**: Bidding across multiple chains
- **Multi-chain Staking**: Stake carbon credits across different chains
- **Cross-chain Analytics**: Unified view of carbon credit markets

## Development

### ADO Development
```bash
# Create new ADO
andromeda ado create my-carbon-ado

# Build ADO
andromeda ado build --path ./ados/my-carbon-ado

# Test ADO
andromeda ado test --path ./ados/my-carbon-ado

# Deploy ADO
andromeda ado deploy --path ./ados/my-carbon-ado
```

### Frontend Development
```bash
# Install dependencies
npm install

# Start development server
npm start

# Build for production
npm run build

# Run tests
npm test
```

## Testing

### Unit Tests
```bash
npm run test:unit
```

### Integration Tests
```bash
npm run test:integration
```

### E2E Tests
```bash
npm run test:e2e
```

## Deployment

### Testnet Deployment
```bash
andromeda config set chain-id andromeda-testnet-1
andromeda ado deploy --path ./ados --network testnet
```

### Mainnet Deployment
```bash
andromeda config set chain-id andromeda-1
andromeda ado deploy --path ./ados --network mainnet
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## License

MIT License - see LICENSE file for details

## Support

- **Documentation**: [Andromeda Docs](https://docs.andromeda.zone)
- **Discord**: [Andromeda Community](https://discord.gg/andromeda)
- **GitHub**: [Issues](https://github.com/andromedaprotocol/agrochain-aos/issues)

## Roadmap

### Phase 1: Core Migration ✅
- [x] Migrate smart contracts to ADOs
- [x] Implement cross-chain carbon credit trading
- [x] Add AI-powered carbon scoring

### Phase 2: Advanced Features 🚧
- [ ] Cross-chain liquidity pools
- [ ] Advanced AI models for carbon prediction
- [ ] Mobile app development
- [ ] Integration with more IoT devices

### Phase 3: Ecosystem Expansion 📋
- [ ] Integration with DeFi protocols
- [ ] Advanced analytics dashboard
- [ ] Regulatory compliance tools
- [ ] Carbon credit derivatives

---

**Built with ❤️ on Andromeda's Operating System (aOS)**

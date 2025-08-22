use cosmwasm_std::{Decimal, Timestamp, StdResult, StdError};
use crate::state::*;

// ============================================================================
// Verification Module
// ============================================================================

/// Verify farmer registration data
pub fn verify_farmer_data(farmer_data: &FarmerData) -> StdResult<()> {
    // Validate name
    if farmer_data.name.trim().is_empty() || farmer_data.name.len() < 2 {
        return Err(StdError::generic_err("Invalid farmer name"));
    }
    
    // Validate location
    if farmer_data.location.trim().is_empty() {
        return Err(StdError::generic_err("Invalid location"));
    }
    
    // Validate land area
    if farmer_data.land_area.trim().is_empty() {
        return Err(StdError::generic_err("Invalid land area"));
    }
    
    // Validate contact information
    if farmer_data.contact.trim().is_empty() {
        return Err(StdError::generic_err("Invalid contact information"));
    }
    
    // Validate IoT device ID
    if farmer_data.iot_device_id.trim().is_empty() {
        return Err(StdError::generic_err("Invalid IoT device ID"));
    }
    
    // Validate farming practices
    if farmer_data.farming_practices.is_empty() {
        return Err(StdError::generic_err("At least one farming practice must be specified"));
    }
    
    Ok(())
}

/// Verify IoT sensor data
pub fn verify_iot_data(verification_data: &VerificationData) -> StdResult<()> {
    // Validate IoT device ID
    if verification_data.iot_device_id.trim().is_empty() {
        return Err(StdError::generic_err("Invalid IoT device ID"));
    }
    
    // Validate sensor readings
    if verification_data.sensor_readings.is_empty() {
        return Err(StdError::generic_err("No sensor readings provided"));
    }
    
    // Validate each sensor reading
    for reading in &verification_data.sensor_readings {
        verify_sensor_reading(reading)?;
    }
    
    // Validate location
    if verification_data.location.trim().is_empty() {
        return Err(StdError::generic_err("Invalid location"));
    }
    
    // Validate farming practices
    if verification_data.farming_practices.is_empty() {
        return Err(StdError::generic_err("No farming practices specified"));
    }
    
    // Validate verification timestamp
    if verification_data.verification_timestamp.seconds() == 0 {
        return Err(StdError::generic_err("Invalid verification timestamp"));
    }
    
    Ok(())
}

/// Verify individual sensor reading
fn verify_sensor_reading(reading: &SensorReading) -> StdResult<()> {
    // Validate sensor type
    match reading.sensor_type {
        SensorType::Temperature => {
            if reading.value < -50.0 || reading.value > 100.0 {
                return Err(StdError::generic_err("Temperature reading out of valid range"));
            }
        }
        SensorType::Humidity => {
            if reading.value < 0.0 || reading.value > 100.0 {
                return Err(StdError::generic_err("Humidity reading out of valid range"));
            }
        }
        SensorType::SoilMoisture => {
            if reading.value < 0.0 || reading.value > 100.0 {
                return Err(StdError::generic_err("Soil moisture reading out of valid range"));
            }
        }
        SensorType::CO2Level => {
            if reading.value < 200.0 || reading.value > 2000.0 {
                return Err(StdError::generic_err("CO2 level reading out of valid range"));
            }
        }
        SensorType::NitrogenLevel => {
            if reading.value < 0.0 || reading.value > 500.0 {
                return Err(StdError::generic_err("Nitrogen level reading out of valid range"));
            }
        }
        SensorType::PhosphorusLevel => {
            if reading.value < 0.0 || reading.value > 100.0 {
                return Err(StdError::generic_err("Phosphorus level reading out of valid range"));
            }
        }
        SensorType::PotassiumLevel => {
            if reading.value < 0.0 || reading.value > 500.0 {
                return Err(StdError::generic_err("Potassium level reading out of valid range"));
            }
        }
        SensorType::PHLevel => {
            if reading.value < 0.0 || reading.value > 14.0 {
                return Err(StdError::generic_err("pH level reading out of valid range"));
            }
        }
        SensorType::Rainfall => {
            if reading.value < 0.0 || reading.value > 1000.0 {
                return Err(StdError::generic_err("Rainfall reading out of valid range"));
            }
        }
        SensorType::WindSpeed => {
            if reading.value < 0.0 || reading.value > 200.0 {
                return Err(StdError::generic_err("Wind speed reading out of valid range"));
            }
        }
        SensorType::SolarRadiation => {
            if reading.value < 0.0 || reading.value > 1200.0 {
                return Err(StdError::generic_err("Solar radiation reading out of valid range"));
            }
        }
    }
    
    // Validate unit
    if reading.unit.trim().is_empty() {
        return Err(StdError::generic_err("Invalid unit"));
    }
    
    // Validate timestamp
    if reading.timestamp.seconds() == 0 {
        return Err(StdError::generic_err("Invalid timestamp"));
    }
    
    // Validate location
    if reading.location.trim().is_empty() {
        return Err(StdError::generic_err("Invalid location"));
    }
    
    Ok(())
}

/// Process IoT sensor data
pub fn process_sensor_data(iot_data: &IOTData) -> StdResult<ProcessedIOTData> {
    let mut processed_readings = Vec::new();
    let mut total_carbon_impact = 0.0;
    let mut total_sustainability = 0.0;
    let mut total_confidence = 0.0;
    let mut valid_readings = 0;
    
    for reading in &iot_data.sensor_readings {
        let processed_reading = process_single_reading(reading)?;
        processed_readings.push(processed_reading.clone());
        
        total_carbon_impact += processed_reading.carbon_contribution.to_float();
        total_sustainability += processed_reading.confidence.to_float();
        total_confidence += processed_reading.confidence.to_float();
        valid_readings += 1;
    }
    
    if valid_readings == 0 {
        return Err(StdError::generic_err("No valid sensor readings"));
    }
    
    let carbon_impact_score = Decimal::from_ratio(
        (total_carbon_impact * 100.0 / valid_readings as f64) as u128,
        100u128
    );
    
    let sustainability_score = Decimal::from_ratio(
        (total_sustainability * 100.0 / valid_readings as f64) as u128,
        100u128
    );
    
    let ai_confidence = Decimal::from_ratio(
        (total_confidence * 100.0 / valid_readings as f64) as u128,
        100u128
    );
    
    Ok(ProcessedIOTData {
        device_id: iot_data.device_id.clone(),
        processed_readings,
        carbon_impact_score,
        sustainability_score,
        processing_timestamp: iot_data.timestamp,
        ai_confidence,
    })
}

/// Process single sensor reading
fn process_single_reading(reading: &SensorReading) -> StdResult<ProcessedReading> {
    let (processed_value, confidence, anomaly_detected, carbon_contribution) = match reading.sensor_type {
        SensorType::Temperature => process_temperature_reading(reading.value),
        SensorType::Humidity => process_humidity_reading(reading.value),
        SensorType::SoilMoisture => process_soil_moisture_reading(reading.value),
        SensorType::CO2Level => process_co2_reading(reading.value),
        SensorType::NitrogenLevel => process_nitrogen_reading(reading.value),
        SensorType::PhosphorusLevel => process_phosphorus_reading(reading.value),
        SensorType::PotassiumLevel => process_potassium_reading(reading.value),
        SensorType::PHLevel => process_ph_reading(reading.value),
        SensorType::Rainfall => process_rainfall_reading(reading.value),
        SensorType::WindSpeed => process_wind_reading(reading.value),
        SensorType::SolarRadiation => process_solar_reading(reading.value),
    };
    
    Ok(ProcessedReading {
        original_reading: reading.clone(),
        processed_value,
        confidence: Decimal::from_ratio((confidence * 100.0) as u128, 100u128),
        anomaly_detected,
        carbon_contribution: Decimal::from_ratio((carbon_contribution * 100.0) as u128, 100u128),
    })
}

// ============================================================================
// Sensor-Specific Processing Functions
// ============================================================================

fn process_temperature_reading(value: f64) -> (f64, f64, bool, f64) {
    let processed_value = value;
    let mut confidence = 0.9;
    let mut anomaly_detected = false;
    let mut carbon_contribution = 0.5;
    
    // Check for anomalies
    if value < -20.0 || value > 50.0 {
        anomaly_detected = true;
        confidence *= 0.5;
    }
    
    // Calculate carbon contribution
    if value >= 15.0 && value <= 25.0 {
        carbon_contribution = 0.9; // Optimal temperature for carbon sequestration
    } else if value >= 10.0 && value <= 30.0 {
        carbon_contribution = 0.7;
    } else if value >= 5.0 && value <= 35.0 {
        carbon_contribution = 0.5;
    } else {
        carbon_contribution = 0.3;
    }
    
    (processed_value, confidence, anomaly_detected, carbon_contribution)
}

fn process_humidity_reading(value: f64) -> (f64, f64, bool, f64) {
    let processed_value = value;
    let mut confidence = 0.9;
    let mut anomaly_detected = false;
    let mut carbon_contribution = 0.5;
    
    // Check for anomalies
    if value < 0.0 || value > 100.0 {
        anomaly_detected = true;
        confidence *= 0.5;
    }
    
    // Calculate carbon contribution
    if value >= 40.0 && value <= 70.0 {
        carbon_contribution = 0.9; // Optimal humidity for carbon sequestration
    } else if value >= 30.0 && value <= 80.0 {
        carbon_contribution = 0.7;
    } else {
        carbon_contribution = 0.5;
    }
    
    (processed_value, confidence, anomaly_detected, carbon_contribution)
}

fn process_soil_moisture_reading(value: f64) -> (f64, f64, bool, f64) {
    let processed_value = value;
    let mut confidence = 0.9;
    let mut anomaly_detected = false;
    let mut carbon_contribution = 0.5;
    
    // Check for anomalies
    if value < 0.0 || value > 100.0 {
        anomaly_detected = true;
        confidence *= 0.5;
    }
    
    // Calculate carbon contribution
    if value >= 20.0 && value <= 40.0 {
        carbon_contribution = 0.9; // Optimal soil moisture for carbon sequestration
    } else if value >= 15.0 && value <= 50.0 {
        carbon_contribution = 0.7;
    } else {
        carbon_contribution = 0.4;
    }
    
    (processed_value, confidence, anomaly_detected, carbon_contribution)
}

fn process_co2_reading(value: f64) -> (f64, f64, bool, f64) {
    let processed_value = value;
    let mut confidence = 0.9;
    let mut anomaly_detected = false;
    let mut carbon_contribution = 0.5;
    
    // Check for anomalies
    if value < 200.0 || value > 2000.0 {
        anomaly_detected = true;
        confidence *= 0.5;
    }
    
    // Calculate carbon contribution (lower CO2 is better)
    if value <= 400.0 {
        carbon_contribution = 0.9; // Excellent carbon sequestration
    } else if value <= 500.0 {
        carbon_contribution = 0.7;
    } else if value <= 600.0 {
        carbon_contribution = 0.5;
    } else {
        carbon_contribution = 0.3;
    }
    
    (processed_value, confidence, anomaly_detected, carbon_contribution)
}

fn process_nitrogen_reading(value: f64) -> (f64, f64, bool, f64) {
    let processed_value = value;
    let mut confidence = 0.9;
    let mut anomaly_detected = false;
    let mut carbon_contribution = 0.5;
    
    // Check for anomalies
    if value < 0.0 || value > 500.0 {
        anomaly_detected = true;
        confidence *= 0.5;
    }
    
    // Calculate carbon contribution
    if value >= 140.0 && value <= 200.0 {
        carbon_contribution = 0.9; // Optimal nitrogen levels
    } else if value >= 100.0 && value <= 250.0 {
        carbon_contribution = 0.7;
    } else {
        carbon_contribution = 0.5;
    }
    
    (processed_value, confidence, anomaly_detected, carbon_contribution)
}

fn process_phosphorus_reading(value: f64) -> (f64, f64, bool, f64) {
    let processed_value = value;
    let mut confidence = 0.9;
    let mut anomaly_detected = false;
    let mut carbon_contribution = 0.5;
    
    // Check for anomalies
    if value < 0.0 || value > 100.0 {
        anomaly_detected = true;
        confidence *= 0.5;
    }
    
    // Calculate carbon contribution
    if value >= 10.0 && value <= 20.0 {
        carbon_contribution = 0.9; // Optimal phosphorus levels
    } else if value >= 5.0 && value <= 30.0 {
        carbon_contribution = 0.7;
    } else {
        carbon_contribution = 0.5;
    }
    
    (processed_value, confidence, anomaly_detected, carbon_contribution)
}

fn process_potassium_reading(value: f64) -> (f64, f64, bool, f64) {
    let processed_value = value;
    let mut confidence = 0.9;
    let mut anomaly_detected = false;
    let mut carbon_contribution = 0.5;
    
    // Check for anomalies
    if value < 0.0 || value > 500.0 {
        anomaly_detected = true;
        confidence *= 0.5;
    }
    
    // Calculate carbon contribution
    if value >= 150.0 && value <= 250.0 {
        carbon_contribution = 0.9; // Optimal potassium levels
    } else if value >= 100.0 && value <= 300.0 {
        carbon_contribution = 0.7;
    } else {
        carbon_contribution = 0.5;
    }
    
    (processed_value, confidence, anomaly_detected, carbon_contribution)
}

fn process_ph_reading(value: f64) -> (f64, f64, bool, f64) {
    let processed_value = value;
    let mut confidence = 0.9;
    let mut anomaly_detected = false;
    let mut carbon_contribution = 0.5;
    
    // Check for anomalies
    if value < 0.0 || value > 14.0 {
        anomaly_detected = true;
        confidence *= 0.5;
    }
    
    // Calculate carbon contribution
    if value >= 6.0 && value <= 7.5 {
        carbon_contribution = 0.9; // Optimal pH range
    } else if value >= 5.5 && value <= 8.0 {
        carbon_contribution = 0.7;
    } else {
        carbon_contribution = 0.4;
    }
    
    (processed_value, confidence, anomaly_detected, carbon_contribution)
}

fn process_rainfall_reading(value: f64) -> (f64, f64, bool, f64) {
    let processed_value = value;
    let mut confidence = 0.9;
    let mut anomaly_detected = false;
    let mut carbon_contribution = 0.5;
    
    // Check for anomalies
    if value < 0.0 || value > 1000.0 {
        anomaly_detected = true;
        confidence *= 0.5;
    }
    
    // Calculate carbon contribution
    if value >= 20.0 && value <= 100.0 {
        carbon_contribution = 0.9; // Moderate rainfall is optimal
    } else if value >= 10.0 && value <= 150.0 {
        carbon_contribution = 0.7;
    } else {
        carbon_contribution = 0.5;
    }
    
    (processed_value, confidence, anomaly_detected, carbon_contribution)
}

fn process_wind_reading(value: f64) -> (f64, f64, bool, f64) {
    let processed_value = value;
    let mut confidence = 0.9;
    let mut anomaly_detected = false;
    let mut carbon_contribution = 0.5;
    
    // Check for anomalies
    if value < 0.0 || value > 200.0 {
        anomaly_detected = true;
        confidence *= 0.5;
    }
    
    // Calculate carbon contribution (lower wind is better)
    if value <= 10.0 {
        carbon_contribution = 0.9; // Low wind speeds are optimal
    } else if value <= 20.0 {
        carbon_contribution = 0.7;
    } else {
        carbon_contribution = 0.5;
    }
    
    (processed_value, confidence, anomaly_detected, carbon_contribution)
}

fn process_solar_reading(value: f64) -> (f64, f64, bool, f64) {
    let processed_value = value;
    let mut confidence = 0.9;
    let mut anomaly_detected = false;
    let mut carbon_contribution = 0.5;
    
    // Check for anomalies
    if value < 0.0 || value > 1200.0 {
        anomaly_detected = true;
        confidence *= 0.5;
    }
    
    // Calculate carbon contribution
    if value >= 200.0 && value <= 600.0 {
        carbon_contribution = 0.9; // Moderate solar radiation is optimal
    } else if value >= 100.0 && value <= 800.0 {
        carbon_contribution = 0.7;
    } else {
        carbon_contribution = 0.5;
    }
    
    (processed_value, confidence, anomaly_detected, carbon_contribution)
}

// ============================================================================
// Data Validation Functions
// ============================================================================

/// Validate IoT device registration
pub fn validate_iot_device(device_id: &str, owner: &str) -> StdResult<()> {
    if device_id.trim().is_empty() {
        return Err(StdError::generic_err("Invalid device ID"));
    }
    
    if owner.trim().is_empty() {
        return Err(StdError::generic_err("Invalid device owner"));
    }
    
    // Check device ID format (should be alphanumeric with optional hyphens)
    if !device_id.chars().all(|c| c.is_alphanumeric() || c == '-') {
        return Err(StdError::generic_err("Invalid device ID format"));
    }
    
    Ok(())
}

/// Validate carbon credit amount
pub fn validate_carbon_amount(amount: &u128) -> StdResult<()> {
    if *amount == 0 {
        return Err(StdError::generic_err("Carbon amount must be greater than zero"));
    }
    
    if *amount > 1_000_000_000 {
        return Err(StdError::generic_err("Carbon amount exceeds maximum limit"));
    }
    
    Ok(())
}

/// Validate price for carbon credit listing
pub fn validate_price(price: &u128) -> StdResult<()> {
    if *price == 0 {
        return Err(StdError::generic_err("Price must be greater than zero"));
    }
    
    if *price > 1_000_000_000_000 {
        return Err(StdError::generic_err("Price exceeds maximum limit"));
    }
    
    Ok(())
}

/// Validate quantity for carbon credit listing
pub fn validate_quantity(quantity: &u128) -> StdResult<()> {
    if *quantity == 0 {
        return Err(StdError::generic_err("Quantity must be greater than zero"));
    }
    
    if *quantity > 1_000_000_000 {
        return Err(StdError::generic_err("Quantity exceeds maximum limit"));
    }
    
    Ok(())
}

// ============================================================================
// Cross-Chain Validation Functions
// ============================================================================

/// Validate cross-chain transfer data
pub fn validate_cross_chain_transfer(
    source_chain: &str,
    target_chain: &str,
    recipient: &str,
    amount: &u128,
) -> StdResult<()> {
    // Validate source chain
    if source_chain.trim().is_empty() {
        return Err(StdError::generic_err("Invalid source chain"));
    }
    
    // Validate target chain
    if target_chain.trim().is_empty() {
        return Err(StdError::generic_err("Invalid target chain"));
    }
    
    // Validate recipient address
    if recipient.trim().is_empty() {
        return Err(StdError::generic_err("Invalid recipient address"));
    }
    
    // Validate amount
    validate_carbon_amount(amount)?;
    
    // Check if source and target chains are different
    if source_chain == target_chain {
        return Err(StdError::generic_err("Source and target chains must be different"));
    }
    
    Ok(())
}

/// Create IBC transfer message
pub fn create_ibc_transfer_msg(transfer_data: &CrossChainData) -> StdResult<cosmwasm_std::CosmosMsg> {
    // This would create an actual IBC transfer message
    // For now, we'll return a placeholder
    Ok(cosmwasm_std::CosmosMsg::Custom(cosmwasm_std::CustomMsg {
        type_url: "ibc.applications.transfer.v1.MsgTransfer".to_string(),
        value: format!(
            "{{\"source_port\":\"transfer\",\"source_channel\":\"channel-0\",\"token\":{{\"denom\":\"uandr\",\"amount\":\"{}\"}},\"sender\":\"{}\",\"receiver\":\"{}\",\"timeout_height\":{{\"revision_number\":\"1\",\"revision_height\":\"1000000\"}},\"timeout_timestamp\":\"0\"}}",
            transfer_data.amount,
            transfer_data.source_chain,
            transfer_data.recipient
        ).into_bytes(),
    }))
}

// ============================================================================
// Utility Functions
// ============================================================================

/// Generate unique credit ID
pub fn generate_credit_id(farmer: &str, timestamp: &Timestamp) -> String {
    format!("credit_{}_{}", farmer, timestamp.seconds())
}

/// Generate unique listing ID
pub fn generate_listing_id(credit_id: &str, timestamp: &Timestamp) -> String {
    format!("listing_{}_{}", credit_id, timestamp.seconds())
}

/// Generate unique transfer ID
pub fn generate_transfer_id(credit_id: &str, timestamp: &Timestamp) -> String {
    format!("transfer_{}_{}", credit_id, timestamp.seconds())
}

/// Generate unique auction ID
pub fn generate_auction_id(credit_id: &str, timestamp: &Timestamp) -> String {
    format!("auction_{}_{}", credit_id, timestamp.seconds())
}

/// Check if timestamp is recent (within last 24 hours)
pub fn is_recent_timestamp(timestamp: &Timestamp) -> bool {
    let current_time = Timestamp::from_seconds(std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs());
    
    let time_diff = current_time.seconds() - timestamp.seconds();
    time_diff <= 86400 // 24 hours in seconds
}

/// Check if timestamp is within acceptable range (not too old or future)
pub fn is_valid_timestamp(timestamp: &Timestamp) -> bool {
    let current_time = Timestamp::from_seconds(std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs());
    
    let time_diff = current_time.seconds() - timestamp.seconds();
    time_diff.abs() <= 31536000 // 1 year in seconds
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::Timestamp;

    #[test]
    fn test_verify_farmer_data() {
        let valid_farmer_data = FarmerData {
            name: "John Doe".to_string(),
            location: "California, USA".to_string(),
            land_area: "100 acres".to_string(),
            contact: "john@example.com".to_string(),
            iot_device_id: "device_001".to_string(),
            farming_practices: vec!["organic".to_string()],
        };
        
        assert!(verify_farmer_data(&valid_farmer_data).is_ok());
    }

    #[test]
    fn test_verify_farmer_data_invalid() {
        let invalid_farmer_data = FarmerData {
            name: "".to_string(),
            location: "California, USA".to_string(),
            land_area: "100 acres".to_string(),
            contact: "john@example.com".to_string(),
            iot_device_id: "device_001".to_string(),
            farming_practices: vec!["organic".to_string()],
        };
        
        assert!(verify_farmer_data(&invalid_farmer_data).is_err());
    }

    #[test]
    fn test_verify_iot_data() {
        let valid_verification_data = VerificationData {
            iot_device_id: "device_001".to_string(),
            sensor_readings: vec![
                SensorReading {
                    sensor_type: SensorType::Temperature,
                    value: 20.0,
                    unit: "°C".to_string(),
                    timestamp: Timestamp::from_seconds(1000),
                    location: "test".to_string(),
                }
            ],
            location: "California, USA".to_string(),
            farming_practices: vec!["organic".to_string()],
            verification_timestamp: Timestamp::from_seconds(1000),
            verification_score: Decimal::from_ratio(85u128, 100u128),
        };
        
        assert!(verify_iot_data(&valid_verification_data).is_ok());
    }

    #[test]
    fn test_process_sensor_data() {
        let valid_iot_data = IOTData {
            device_id: "device_001".to_string(),
            sensor_readings: vec![
                SensorReading {
                    sensor_type: SensorType::Temperature,
                    value: 20.0,
                    unit: "°C".to_string(),
                    timestamp: Timestamp::from_seconds(1000),
                    location: "test".to_string(),
                }
            ],
            timestamp: Timestamp::from_seconds(1000),
            location: "test".to_string(),
            device_status: DeviceStatus::Online,
        };
        
        let processed_data = process_sensor_data(&valid_iot_data).unwrap();
        assert_eq!(processed_data.device_id, "device_001");
        assert!(!processed_data.processed_readings.is_empty());
    }

    #[test]
    fn test_validate_carbon_amount() {
        assert!(validate_carbon_amount(&1000).is_ok());
        assert!(validate_carbon_amount(&0).is_err());
        assert!(validate_carbon_amount(&2_000_000_000).is_err());
    }

    #[test]
    fn test_generate_ids() {
        let timestamp = Timestamp::from_seconds(1000);
        
        let credit_id = generate_credit_id("farmer123", &timestamp);
        assert!(credit_id.starts_with("credit_farmer123_"));
        
        let listing_id = generate_listing_id(&credit_id, &timestamp);
        assert!(listing_id.starts_with("listing_"));
        
        let transfer_id = generate_transfer_id(&credit_id, &timestamp);
        assert!(transfer_id.starts_with("transfer_"));
    }
}

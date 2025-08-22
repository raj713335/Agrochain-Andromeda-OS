use cosmwasm_std::{Decimal, Uint128, Timestamp, StdResult, StdError};
use crate::state::*;

// ============================================================================
// AI Carbon Scoring Module
// ============================================================================

/// Calculate carbon score based on IoT data and farming practices
pub fn calculate_carbon_score(
    carbon_amount: &Uint128,
    verification_data: &VerificationData,
    farming_practices: &[String],
    timestamp: &Timestamp,
) -> StdResult<Decimal> {
    // Base score from carbon amount
    let base_score = calculate_base_score(carbon_amount)?;
    
    // IoT verification score
    let iot_score = calculate_iot_score(verification_data)?;
    
    // Farming practices score
    let practices_score = calculate_practices_score(farming_practices)?;
    
    // Environmental factors score
    let environmental_score = calculate_environmental_score(verification_data)?;
    
    // Time-based adjustment
    let time_adjustment = calculate_time_adjustment(timestamp)?;
    
    // Weighted combination
    let final_score = (base_score * Decimal::from_ratio(3u128, 10u128)) +
                     (iot_score * Decimal::from_ratio(3u128, 10u128)) +
                     (practices_score * Decimal::from_ratio(2u128, 10u128)) +
                     (environmental_score * Decimal::from_ratio(15u128, 100u128)) +
                     (time_adjustment * Decimal::from_ratio(5u128, 100u128));
    
    Ok(final_score)
}

/// Calculate base score from carbon amount
fn calculate_base_score(carbon_amount: &Uint128) -> StdResult<Decimal> {
    let amount = carbon_amount.u128() as f64;
    
    // Logarithmic scaling to prevent extremely high scores
    let log_amount = (amount + 1.0).ln();
    let normalized_score = (log_amount / 10.0).min(1.0).max(0.0);
    
    Ok(Decimal::from_ratio((normalized_score * 100.0) as u128, 100u128))
}

/// Calculate score based on IoT sensor data
fn calculate_iot_score(verification_data: &VerificationData) -> StdResult<Decimal> {
    let mut total_score = 0.0;
    let mut valid_readings = 0;
    
    for reading in &verification_data.sensor_readings {
        let reading_score = calculate_sensor_score(reading)?;
        total_score += reading_score;
        valid_readings += 1;
    }
    
    if valid_readings == 0 {
        return Ok(Decimal::zero());
    }
    
    let average_score = total_score / valid_readings as f64;
    Ok(Decimal::from_ratio((average_score * 100.0) as u128, 100u128))
}

/// Calculate score for individual sensor reading
fn calculate_sensor_score(reading: &SensorReading) -> StdResult<f64> {
    match reading.sensor_type {
        SensorType::Temperature => calculate_temperature_score(reading.value),
        SensorType::Humidity => calculate_humidity_score(reading.value),
        SensorType::SoilMoisture => calculate_soil_moisture_score(reading.value),
        SensorType::CO2Level => calculate_co2_score(reading.value),
        SensorType::NitrogenLevel => calculate_nitrogen_score(reading.value),
        SensorType::PhosphorusLevel => calculate_phosphorus_score(reading.value),
        SensorType::PotassiumLevel => calculate_potassium_score(reading.value),
        SensorType::PHLevel => calculate_ph_score(reading.value),
        SensorType::Rainfall => calculate_rainfall_score(reading.value),
        SensorType::WindSpeed => calculate_wind_score(reading.value),
        SensorType::SolarRadiation => calculate_solar_score(reading.value),
    }
}

/// Calculate score based on farming practices
fn calculate_practices_score(practices: &[String]) -> StdResult<Decimal> {
    let mut total_score = 0.0;
    let mut practice_count = 0;
    
    for practice in practices {
        let practice_score = match practice.to_lowercase().as_str() {
            "organic" => 0.9,
            "sustainable" => 0.85,
            "regenerative" => 0.95,
            "no-till" => 0.8,
            "crop-rotation" => 0.75,
            "cover-cropping" => 0.8,
            "precision-agriculture" => 0.85,
            "drip-irrigation" => 0.8,
            "composting" => 0.85,
            "natural-pesticides" => 0.8,
            "biodiversity" => 0.9,
            "soil-conservation" => 0.85,
            "water-conservation" => 0.8,
            "renewable-energy" => 0.9,
            "carbon-farming" => 0.95,
            _ => 0.5, // Default score for unknown practices
        };
        
        total_score += practice_score;
        practice_count += 1;
    }
    
    if practice_count == 0 {
        return Ok(Decimal::from_ratio(50u128, 100u128)); // Default score
    }
    
    let average_score = total_score / practice_count as f64;
    Ok(Decimal::from_ratio((average_score * 100.0) as u128, 100u128))
}

/// Calculate environmental factors score
fn calculate_environmental_score(verification_data: &VerificationData) -> StdResult<Decimal> {
    let mut environmental_score = 0.0;
    let mut factor_count = 0;
    
    // Location-based scoring
    let location_score = calculate_location_score(&verification_data.location)?;
    environmental_score += location_score;
    factor_count += 1;
    
    // Seasonal adjustments
    let seasonal_score = calculate_seasonal_score(&verification_data.verification_timestamp)?;
    environmental_score += seasonal_score;
    factor_count += 1;
    
    // Weather impact
    let weather_score = calculate_weather_impact(verification_data)?;
    environmental_score += weather_score;
    factor_count += 1;
    
    if factor_count == 0 {
        return Ok(Decimal::from_ratio(50u128, 100u128));
    }
    
    let average_score = environmental_score / factor_count as f64;
    Ok(Decimal::from_ratio((average_score * 100.0) as u128, 100u128))
}

/// Calculate time-based adjustment
fn calculate_time_adjustment(timestamp: &Timestamp) -> StdResult<Decimal> {
    // Recent credits get a slight boost
    let current_time = Timestamp::from_seconds(std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs());
    
    let time_diff = current_time.seconds() - timestamp.seconds();
    let days_diff = time_diff / 86400; // Convert to days
    
    // Boost for credits minted in the last 30 days
    if days_diff <= 30 {
        Ok(Decimal::from_ratio(5u128, 100u128))
    } else if days_diff <= 90 {
        Ok(Decimal::from_ratio(2u128, 100u128))
    } else {
        Ok(Decimal::zero())
    }
}

// ============================================================================
// Sensor-Specific Scoring Functions
// ============================================================================

fn calculate_temperature_score(value: f64) -> StdResult<f64> {
    // Optimal temperature range for most crops: 15-25°C
    if value >= 15.0 && value <= 25.0 {
        Ok(0.9)
    } else if value >= 10.0 && value <= 30.0 {
        Ok(0.7)
    } else if value >= 5.0 && value <= 35.0 {
        Ok(0.5)
    } else {
        Ok(0.3)
    }
}

fn calculate_humidity_score(value: f64) -> StdResult<f64> {
    // Optimal humidity range: 40-70%
    if value >= 40.0 && value <= 70.0 {
        Ok(0.9)
    } else if value >= 30.0 && value <= 80.0 {
        Ok(0.7)
    } else {
        Ok(0.5)
    }
}

fn calculate_soil_moisture_score(value: f64) -> StdResult<f64> {
    // Optimal soil moisture: 20-40%
    if value >= 20.0 && value <= 40.0 {
        Ok(0.9)
    } else if value >= 15.0 && value <= 50.0 {
        Ok(0.7)
    } else {
        Ok(0.4)
    }
}

fn calculate_co2_score(value: f64) -> StdResult<f64> {
    // Lower CO2 levels are better for carbon sequestration
    if value <= 400.0 {
        Ok(0.9)
    } else if value <= 500.0 {
        Ok(0.7)
    } else if value <= 600.0 {
        Ok(0.5)
    } else {
        Ok(0.3)
    }
}

fn calculate_nitrogen_score(value: f64) -> StdResult<f64> {
    // Optimal nitrogen levels: 140-200 ppm
    if value >= 140.0 && value <= 200.0 {
        Ok(0.9)
    } else if value >= 100.0 && value <= 250.0 {
        Ok(0.7)
    } else {
        Ok(0.5)
    }
}

fn calculate_phosphorus_score(value: f64) -> StdResult<f64> {
    // Optimal phosphorus levels: 10-20 ppm
    if value >= 10.0 && value <= 20.0 {
        Ok(0.9)
    } else if value >= 5.0 && value <= 30.0 {
        Ok(0.7)
    } else {
        Ok(0.5)
    }
}

fn calculate_potassium_score(value: f64) -> StdResult<f64> {
    // Optimal potassium levels: 150-250 ppm
    if value >= 150.0 && value <= 250.0 {
        Ok(0.9)
    } else if value >= 100.0 && value <= 300.0 {
        Ok(0.7)
    } else {
        Ok(0.5)
    }
}

fn calculate_ph_score(value: f64) -> StdResult<f64> {
    // Optimal pH range: 6.0-7.5
    if value >= 6.0 && value <= 7.5 {
        Ok(0.9)
    } else if value >= 5.5 && value <= 8.0 {
        Ok(0.7)
    } else {
        Ok(0.4)
    }
}

fn calculate_rainfall_score(value: f64) -> StdResult<f64> {
    // Moderate rainfall is optimal
    if value >= 20.0 && value <= 100.0 {
        Ok(0.9)
    } else if value >= 10.0 && value <= 150.0 {
        Ok(0.7)
    } else {
        Ok(0.5)
    }
}

fn calculate_wind_score(value: f64) -> StdResult<f64> {
    // Lower wind speeds are better for carbon sequestration
    if value <= 10.0 {
        Ok(0.9)
    } else if value <= 20.0 {
        Ok(0.7)
    } else {
        Ok(0.5)
    }
}

fn calculate_solar_score(value: f64) -> StdResult<f64> {
    // Moderate solar radiation is optimal
    if value >= 200.0 && value <= 600.0 {
        Ok(0.9)
    } else if value >= 100.0 && value <= 800.0 {
        Ok(0.7)
    } else {
        Ok(0.5)
    }
}

// ============================================================================
// Environmental Scoring Functions
// ============================================================================

fn calculate_location_score(location: &str) -> StdResult<f64> {
    // Score based on geographic location and climate zone
    let location_lower = location.to_lowercase();
    
    if location_lower.contains("tropical") || location_lower.contains("equatorial") {
        Ok(0.8) // High carbon sequestration potential
    } else if location_lower.contains("temperate") {
        Ok(0.9) // Optimal for sustainable farming
    } else if location_lower.contains("mediterranean") {
        Ok(0.85) // Good for sustainable practices
    } else if location_lower.contains("arid") || location_lower.contains("desert") {
        Ok(0.6) // Challenging but possible with proper practices
    } else {
        Ok(0.7) // Default score
    }
}

fn calculate_seasonal_score(timestamp: &Timestamp) -> StdResult<f64> {
    // Extract month from timestamp
    let datetime = chrono::DateTime::from_timestamp(timestamp.seconds() as i64, 0)
        .ok_or_else(|| StdError::generic_err("Invalid timestamp"))?;
    
    let month = datetime.month();
    
    // Score based on growing season
    let seasonal_score = match month {
        3..=5 => 0.9,   // Spring - optimal growing season
        6..=8 => 0.85,  // Summer - good growing season
        9..=11 => 0.8,  // Fall - harvest season
        _ => 0.7,       // Winter - off-season
    };
    
    Ok(seasonal_score)
}

fn calculate_weather_impact(verification_data: &VerificationData) -> StdResult<f64> {
    let mut weather_score = 0.0;
    let mut weather_factors = 0;
    
    // Analyze sensor readings for weather patterns
    for reading in &verification_data.sensor_readings {
        match reading.sensor_type {
            SensorType::Temperature => {
                let temp_score = calculate_temperature_score(reading.value)?;
                weather_score += temp_score;
                weather_factors += 1;
            }
            SensorType::Humidity => {
                let humidity_score = calculate_humidity_score(reading.value)?;
                weather_score += humidity_score;
                weather_factors += 1;
            }
            SensorType::Rainfall => {
                let rainfall_score = calculate_rainfall_score(reading.value)?;
                weather_score += rainfall_score;
                weather_factors += 1;
            }
            _ => {}
        }
    }
    
    if weather_factors == 0 {
        return Ok(0.7); // Default weather score
    }
    
    Ok(weather_score / weather_factors as f64)
}

// ============================================================================
// AI Model Prediction Functions
// ============================================================================

/// Predict carbon emission based on farming practices and environmental factors
pub fn predict_carbon_emission(
    farming_practices: &[String],
    environmental_data: &VerificationData,
) -> StdResult<Uint128> {
    let mut base_emission = 1000u128; // Base emission per acre
    
    // Adjust based on farming practices
    for practice in farming_practices {
        let reduction = match practice.to_lowercase().as_str() {
            "organic" => 0.3,        // 30% reduction
            "no-till" => 0.2,        // 20% reduction
            "cover-cropping" => 0.15, // 15% reduction
            "crop-rotation" => 0.1,   // 10% reduction
            "precision-agriculture" => 0.25, // 25% reduction
            "drip-irrigation" => 0.1, // 10% reduction
            "composting" => 0.2,      // 20% reduction
            "renewable-energy" => 0.4, // 40% reduction
            _ => 0.0,
        };
        
        base_emission = (base_emission as f64 * (1.0 - reduction)) as u128;
    }
    
    // Adjust based on environmental factors
    let environmental_factor = calculate_environmental_factor(environmental_data)?;
    base_emission = (base_emission as f64 * environmental_factor) as u128;
    
    Ok(Uint128::from(base_emission))
}

/// Calculate environmental factor for emission prediction
fn calculate_environmental_factor(environmental_data: &VerificationData) -> StdResult<f64> {
    let mut factor = 1.0;
    
    // Analyze sensor data for environmental impact
    for reading in &environmental_data.sensor_readings {
        match reading.sensor_type {
            SensorType::Temperature => {
                if reading.value > 30.0 {
                    factor *= 1.2; // Higher emissions in hot weather
                } else if reading.value < 10.0 {
                    factor *= 1.1; // Slightly higher in cold weather
                }
            }
            SensorType::SoilMoisture => {
                if reading.value < 15.0 {
                    factor *= 1.15; // Higher emissions in dry soil
                }
            }
            SensorType::CO2Level => {
                if reading.value > 500.0 {
                    factor *= 1.1; // Higher emissions in high CO2 environments
                }
            }
            _ => {}
        }
    }
    
    Ok(factor)
}

/// Recalculate carbon score for existing farmer
pub fn recalculate_carbon_score(
    farmer: &Farmer,
    processed_data: &ProcessedIOTData,
) -> StdResult<Decimal> {
    // Use processed IoT data to recalculate score
    let iot_score = processed_data.carbon_impact_score;
    let sustainability_score = processed_data.sustainability_score;
    let ai_confidence = processed_data.ai_confidence;
    
    // Weighted combination with existing farmer data
    let existing_score = farmer.carbon_score;
    let new_score = (iot_score * Decimal::from_ratio(4u128, 10u128)) +
                   (sustainability_score * Decimal::from_ratio(3u128, 10u128)) +
                   (existing_score * Decimal::from_ratio(2u128, 10u128)) +
                   (ai_confidence * Decimal::from_ratio(1u128, 10u128));
    
    Ok(new_score)
}

// ============================================================================
// Machine Learning Utilities
// ============================================================================

/// Simple linear regression for carbon prediction
pub fn linear_regression_prediction(
    x_values: &[f64],
    y_values: &[f64],
    new_x: f64,
) -> StdResult<f64> {
    if x_values.len() != y_values.len() || x_values.is_empty() {
        return Err(StdError::generic_err("Invalid input data"));
    }
    
    let n = x_values.len() as f64;
    let sum_x: f64 = x_values.iter().sum();
    let sum_y: f64 = y_values.iter().sum();
    let sum_xy: f64 = x_values.iter().zip(y_values.iter()).map(|(x, y)| x * y).sum();
    let sum_x2: f64 = x_values.iter().map(|x| x * x).sum();
    
    let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x * sum_x);
    let intercept = (sum_y - slope * sum_x) / n;
    
    Ok(slope * new_x + intercept)
}

/// Calculate correlation coefficient between two datasets
pub fn calculate_correlation(x_values: &[f64], y_values: &[f64]) -> StdResult<f64> {
    if x_values.len() != y_values.len() || x_values.is_empty() {
        return Err(StdError::generic_err("Invalid input data"));
    }
    
    let n = x_values.len() as f64;
    let sum_x: f64 = x_values.iter().sum();
    let sum_y: f64 = y_values.iter().sum();
    let sum_xy: f64 = x_values.iter().zip(y_values.iter()).map(|(x, y)| x * y).sum();
    let sum_x2: f64 = x_values.iter().map(|x| x * x).sum();
    let sum_y2: f64 = y_values.iter().map(|y| y * y).sum();
    
    let numerator = n * sum_xy - sum_x * sum_y;
    let denominator = ((n * sum_x2 - sum_x * sum_x) * (n * sum_y2 - sum_y * sum_y)).sqrt();
    
    if denominator == 0.0 {
        return Ok(0.0);
    }
    
    Ok(numerator / denominator)
}

/// Anomaly detection using Z-score method
pub fn detect_anomaly(value: f64, mean: f64, std_dev: f64, threshold: f64) -> bool {
    if std_dev == 0.0 {
        return false;
    }
    
    let z_score = (value - mean).abs() / std_dev;
    z_score > threshold
}

/// Calculate moving average for time series data
pub fn calculate_moving_average(values: &[f64], window_size: usize) -> StdResult<Vec<f64>> {
    if values.len() < window_size {
        return Err(StdError::generic_err("Insufficient data for moving average"));
    }
    
    let mut moving_averages = Vec::new();
    
    for i in (window_size - 1)..values.len() {
        let window_sum: f64 = values[i - window_size + 1..=i].iter().sum();
        moving_averages.push(window_sum / window_size as f64);
    }
    
    Ok(moving_averages)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::Uint128;

    #[test]
    fn test_calculate_carbon_score() {
        let carbon_amount = Uint128::from(1000u128);
        let verification_data = VerificationData {
            iot_device_id: "test_device".to_string(),
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
        let farming_practices = vec!["organic".to_string(), "sustainable".to_string()];
        let timestamp = Timestamp::from_seconds(1000);
        
        let score = calculate_carbon_score(&carbon_amount, &verification_data, &farming_practices, &timestamp).unwrap();
        assert!(score > Decimal::zero());
    }

    #[test]
    fn test_calculate_practices_score() {
        let practices = vec!["organic".to_string(), "sustainable".to_string()];
        let score = calculate_practices_score(&practices).unwrap();
        assert!(score > Decimal::from_ratio(50u128, 100u128));
    }

    #[test]
    fn test_linear_regression_prediction() {
        let x_values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y_values = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        let prediction = linear_regression_prediction(&x_values, &y_values, 6.0).unwrap();
        assert!((prediction - 12.0).abs() < 0.1);
    }
}

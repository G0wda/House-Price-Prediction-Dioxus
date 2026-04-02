use linfa::prelude::*;
use ndarray::Array2;

pub struct HouseInput {
    pub longitude: f64,
    pub latitude: f64,
    pub housing_median_age: f64,
    pub total_rooms: f64,
    pub total_bedrooms: f64,
    pub population: f64,
    pub households: f64,
    pub median_income: f64,
    pub ocean_proximity: String,
}


pub fn encode_ocean(value: &str) -> f64 {
    match value.trim() {
        "NEAR BAY" => 4.0,
        "NEAR OCEAN" => 3.0,
        "INLAND" => 2.0,
        "ISLAND" => 1.0,
        _ => 0.0,
    }
}

pub fn to_features_vec(input: &HouseInput) -> Vec<f64> {
    vec![
        input.longitude,
        input.latitude,
        input.housing_median_age,
        input.total_rooms,
        input.total_bedrooms,
        input.population,
        input.households,
        input.median_income,
        encode_ocean(&input.ocean_proximity),
    ]
}

pub fn predict_house_price(
    model: &linfa_linear::FittedLinearRegression<f64>,
    input: HouseInput,
) -> f64 {
    let features = to_features_vec(&input);
    let x = Array2::from_shape_vec((1, features.len()), features).unwrap();
    let pred = model.predict(&x);
    pred[0]
}
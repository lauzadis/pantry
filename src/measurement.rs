use bigdecimal::BigDecimal;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum Measurement {
    Volume {value: f32, unit: VolumeUnit},
    Mass {value: f32, unit: MassUnit},
}


#[derive(Serialize, Deserialize)]
pub enum VolumeUnit {
    Teaspoon,
    Tablespoon,
    FluidOunce,
    Cup,
    // if you use any of the below, I must ask, what (or for how many people) are you cooking?
    Pint,
    Quart,
    Gallon
}

#[derive(Serialize, Deserialize)]
pub enum MassUnit {
    Milligram,
    Gram,
    Ounce,
    Pound,
}
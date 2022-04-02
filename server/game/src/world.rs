use once_cell::sync::Lazy;

use minecrevy_key::key;
use minecrevy_protocol_latest::types::{DimensionRegistry, DimensionType};

pub fn default_dimension_registry() -> &'static DimensionRegistry {
    static JSON: &'static str = include_str!("../../../data/default_dimension_codec.json");
    static PARSED: Lazy<DimensionRegistry> = Lazy::new(|| {
        serde_json::from_str::<DimensionRegistry>(JSON)
            .expect("failed to parse default_dimension_codec.json")
    });

    &(*PARSED)
}

pub fn default_dimension_type() -> &'static DimensionType {
    default_dimension_registry()
        .dimension_type_registry
        .element(key!("minecraft:overworld"))
        .expect("dimension type is not registered: 'minecraft:overworld'")
}

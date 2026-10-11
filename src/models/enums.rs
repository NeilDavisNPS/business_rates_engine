use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Jurisdiction {
    ENGLAND,
    WALES,
    SCOTLAND,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PartyType {
    INDIVIDUAL,
    CORPORATE,
    CHARITY,
    CASC,
    LOCAL_AUTHORITY,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OccupancyType {
    OCCUPIED,
    UNOCCUPIED,
    PART_OCCUPIED,
}

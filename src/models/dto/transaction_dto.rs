use serde::{Serialize, Deserialize};
use crate::models::transaction_models::{TransactionStatus, TransactionType};


#[derive(Serialize, Deserialize)]
pub struct TransactionDto {
    pub name: String,
    pub status: TransactionStatus,
    pub value: f32, 
    pub category: String,
    pub transaction_date: String,
    pub transaction_type: TransactionType,
}

#[derive(Deserialize)]
pub struct TransactionQuery {
    pub transaction_type: Option<String>,
}

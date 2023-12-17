use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum TransactionType {
    Deposit,
    Withdraw,
}
#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub id: Uuid,
    pub name: String,
    pub status: TransactionStatus,
    pub value: f32, 
    pub category: String,
    pub transaction_date: String,
    pub transaction_type: TransactionType,
}

impl Transaction {
    pub fn new(id: Uuid, name: String, status: TransactionStatus, value: f32, category: String, transaction_date: String, transaction_type: TransactionType) -> Transaction {
        Transaction {
            id,
            name,
            status,
            value,
            category,
            transaction_date,
            transaction_type,
        }
    }
}


use crate::models::transaction_models::{Transaction, TransactionType, TransactionStatus};
use crate::models::dto::transaction_dto::{TransactionDto, TransactionQuery};
use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref TRANSACTIONS: Mutex<Vec<Transaction>> = Mutex::new(
        vec![
            Transaction::new(
                Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap(),
                "Transação de Depósito".to_string(), 
                TransactionStatus::Completed, 
                1500.0, 
                "Salário".to_string(), 
                "2023-01-01".to_string(), 
                TransactionType::Deposit
            ),
            Transaction::new(
                Uuid::parse_str("e6e40e13-fe42-4446-92ce-ce02051e3d90").unwrap(),
                "Compra de Eletrônicos".to_string(), 
                TransactionStatus::Pending, 
                300.0, 
                "Eletrônicos".to_string(), 
                "2023-01-05".to_string(), 
                TransactionType::Withdraw
            ),
            Transaction::new(
                Uuid::parse_str("4e4c8bf7-a73e-4acd-a9e3-d7d55eb18b9c").unwrap(),
                "Transação de Depósito".to_string(), 
                TransactionStatus::Failed, 
                500.0, 
                "Presente".to_string(), 
                "2023-01-10".to_string(), 
                TransactionType::Deposit
            ),
        ]
    );
}

pub async fn get_transactions(query: web::Query<TransactionQuery>) -> impl Responder {
    let transactions = TRANSACTIONS.lock().unwrap();

    let filtered_transactions: Vec<Transaction> = if let Some(transaction_type) = &query.transaction_type {
        transactions.iter().filter(|t| {
            match transaction_type.as_str() {
                "deposit" => t.transaction_type == TransactionType::Deposit,
                "withdraw" => t.transaction_type == TransactionType::Withdraw,
                _ => true,
            }
        }).cloned().collect()
    } else {
        transactions.clone()
    };

    HttpResponse::Ok().json(&filtered_transactions)
}

pub async fn add_transaction(new_transaction: web::Json<TransactionDto>) -> impl Responder {
    let mut transactions = TRANSACTIONS.lock().unwrap();

    let transaction_dto = new_transaction.into_inner();
    let transaction = Transaction {
        id: Uuid::new_v4(),
        name: transaction_dto.name,
        status: transaction_dto.status,
        value: transaction_dto.value,
        category: transaction_dto.category,
        transaction_date: transaction_dto.transaction_date,
        transaction_type: transaction_dto.transaction_type,
    };

    transactions.push(transaction.clone());
    HttpResponse::Ok().json(&transaction)
}

pub async fn update_transaction(
    path: web::Path<Uuid>, 
    updated_transaction: web::Json<TransactionDto>
) -> impl Responder {
    let mut transactions = TRANSACTIONS.lock().unwrap();
    let transaction_id = path.into_inner();

    for transaction in &mut *transactions {
        if transaction.id == transaction_id {
            transaction.name = updated_transaction.name.clone();
            transaction.status = updated_transaction.status;
            transaction.value = updated_transaction.value;
            transaction.category = updated_transaction.category.clone();
            transaction.transaction_date = updated_transaction.transaction_date.clone();
            transaction.transaction_type = updated_transaction.transaction_type;
            return HttpResponse::Ok().json("Transação atualizada com sucesso");
        }
    }

    HttpResponse::NotFound().json("Transação não encontrada")
}

pub async fn delete_transaction(path: web::Path<Uuid>) -> impl Responder {
    let mut transactions = TRANSACTIONS.lock().unwrap();
    let transaction_id = path.into_inner();

    let index = transactions.iter().position(|t| t.id == transaction_id);
    if let Some(index) = index {
        transactions.remove(index);
        return HttpResponse::Ok().json("Transação excluída com sucesso");
    }

    HttpResponse::NotFound().json("Transação não encontrada")
}

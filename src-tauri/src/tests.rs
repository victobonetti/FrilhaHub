use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;

use crate::Account;
use crate::Item;
use crate::Payment;
use crate::Product;
use crate::User; // Importe o módulo user do arquivo principal

fn setup() -> PooledConnection<SqliteConnectionManager> {
    // Crie uma conexão do banco de dados
    let manager = SqliteConnectionManager::file("test.db");
    let pool = Pool::builder().max_size(1).build(manager).unwrap();
    let conn = pool.get().unwrap();

    // Crie a tabela users
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT NOT NULL,
            password TEXT NOT NULL,
            account_id TEXT 
        )",
        params![],
    )
    .unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS products (
            id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                price REAL NOT NULL
            )",
        params![],
    )
    .unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS accounts (
            id TEXT PRIMARY KEY,
            user_id INTEGER NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users(id)
        )",
        params![],
    )
    .unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS payments (
            id TEXT PRIMARY KEY,
            amount REAL NOT NULL,
            account_id TEXT NOT NULL,
            FOREIGN KEY (account_id) REFERENCES accounts(id)
        )",
        params![],
    )
    .unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS items (
            id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                quantity INTEGER NOT NULL,
                product_id TEXT NOT NULL,
                price REAL NOT NULL,
                notes TEXT,
                account_id TEXT NOT NULL,
                FOREIGN KEY (product_id) REFERENCES products(id),
                FOREIGN KEY (account_id) REFERENCES accounts(id)
            )",
        params![],
    )
    .unwrap();

    return conn;
}

#[test]
fn test_user_crud() {
    let conn = setup();

    // Crie um usuário
    let username = "john";
    let password = "password";
    let test_id = User::create_one(&conn, username.to_string(), password.to_string())
        .unwrap()
        .clone();

    // Encontre o usuário criado
    let found_user = User::find_one(&conn, test_id.clone()).unwrap();
    assert!(found_user.is_some());
    let user = found_user.unwrap();
    assert_eq!(user.username, username);
    assert_eq!(user.password, password);

    // Exclua o usuário
    User::delete_one(&conn, test_id.clone()).unwrap();

    // Verifique se o usuário foi removido
    let users = User::find_one(&conn, test_id.clone()).unwrap();
    assert!(users.is_none());
}

#[test]
fn test_product_crud() {
    let conn = setup();
    // Crie uma conexão do banco de dados
    let manager = SqliteConnectionManager::file("test.db");
    let pool = Pool::builder().max_size(1).build(manager).unwrap();
    let conn = pool.get().unwrap();

    // Crie um produto
    let name = "Product A";
    let price = 10.0;
    let test_id = Product::create_one(&conn, name, price).unwrap().clone();

    // Edite o preço do produto
    let new_price = 15.0;
    Product::edit_price(&conn, test_id.clone(), new_price).unwrap();

    // Encontre o produto atualizado
    let found_product = Product::find_one(&conn, test_id.clone()).unwrap();
    assert!(found_product.is_some());
    let product = found_product.unwrap();
    assert_eq!(product.price, new_price);

    // Edite o nome do produto
    let new_name = "Product B";
    Product::edit_name(&conn, test_id.clone(), new_name).unwrap();

    // Encontre o produto atualizado
    let found_product = Product::find_one(&conn, test_id.clone()).unwrap();
    assert!(found_product.is_some());
    let product = found_product.unwrap();
    assert_eq!(product.name, new_name);

    // Exclua o produto
    Product::delete_one(&conn, test_id.clone()).unwrap();

    // Verifique se o produto foi removido
    let products = Product::find_one(&conn, test_id.clone()).unwrap();
    assert!(products.is_none());
}

#[test]
fn test_payment_crud() {
    let conn = setup();
    // Crie uma conexão do banco de dados
    let manager = SqliteConnectionManager::file("test.db");
    let pool = Pool::builder().max_size(1).build(manager).unwrap();
    let conn = pool.get().unwrap();

    // Crie um usuário
    let username = "john";
    let password = "password";
    let test_id = User::create_one(&conn, username.to_string(), password.to_string())
        .unwrap()
        .clone();

    // Crie uma conta
    let account_id: String = Account::create_account(&conn, test_id.clone()).unwrap();

    // Crie um pagamento
    let amount = 100.0;
    Payment::create_one(&conn, amount, account_id.clone()).unwrap();

    // Encontre o pagamento criado
    let mut stmt = conn
        .prepare("SELECT * FROM payments WHERE account_id = ?1")
        .unwrap();
    let payment_iter = stmt
        .query_map(params![account_id], |row| {
            Ok(Payment {
                id: row.get(0)?,
                amount: row.get(1)?,
                account_id: row.get(2)?,
            })
        })
        .unwrap();

    let mut payments = Vec::new();
    for payment_result in payment_iter {
        let payment = payment_result.unwrap();
        payments.push(payment);
    }

    assert_eq!(payments.len(), 1);
    let payment = payments.pop().unwrap();
    assert_eq!(payment.amount, amount);

    // Atualize o valor do pagamento
    let new_amount = 200.0;
    Payment::update_amount(&conn, payment.id.clone(), new_amount).unwrap();

    // Encontre o pagamento atualizado
    let payment = Payment::find_one(&conn, payment.id.clone()).unwrap();
    assert!(payment.is_some());
    let payment = payment.unwrap();
    assert_eq!(payment.amount, new_amount);

    // Exclua o pagamento
    Payment::delete_one(&conn, payment.id.clone()).unwrap();

    // Verifique se o pagamento foi removido
    let payment = Payment::find_one(&conn, payment.id.clone()).unwrap();
    assert!(payment.is_none());

    // Exclua a conta
    Account::delete_one(&conn, account_id.clone()).unwrap();

    // Exclua o usuário
    User::delete_one(&conn, test_id.clone()).unwrap();

    // Verifique se o usuário foi removido
    let users = User::find_one(&conn, test_id.clone()).unwrap();
    assert!(users.is_none());
}

#[test]
fn test_item_crud() {
    let conn = setup();

    // Crie um usuário
    let username = "john";
    let password = "password";
    let user_id = User::create_one(&conn, username.to_string(), password.to_string())
        .unwrap()
        .clone();

    // Crie uma conta
    let account_id: String = Account::create_account(&conn, user_id.clone()).unwrap();

    // Crie um produto
    let name = "Product A";
    let price = 10.0;
    let product_id = Product::create_one(&conn, name, price).unwrap().clone();

    // Crie um item
    let name = "Item A".to_string();
    let quantity = 2;
    let price = 20.0;
    let item_id: String = Item::create_item(
        &conn,
        name.clone(),
        quantity,
        price,
        account_id.clone(),
        product_id.clone(),
    )
    .unwrap();

    // Encontre o item criado
    let item = Item::find_item(&conn, item_id.clone()).unwrap();
    let item = item.unwrap();
    assert_eq!(item.name, name);
    assert_eq!(item.quantity, quantity);
    assert_eq!(item.price, price);

    // Edite a nota do item
    let notes = Some("Some notes".to_string());
    Item::edit_note(&conn, item_id.clone(), notes.clone()).unwrap();

    // Encontre o item atualizado
    let item = Item::find_item(&conn, item_id.clone()).unwrap();
    assert!(item.is_some());
    let item = item.unwrap();
    assert_eq!(item.notes, notes);

    // Edite o preço do item
    let new_price = 25.0;
    Item::edit_price(&conn, item_id.clone(), new_price).unwrap();

    // Encontre o item atualizado
    let item = Item::find_item(&conn, item_id.clone()).unwrap();
    assert!(item.is_some());
    let item = item.unwrap();
    assert_eq!(item.price, new_price);

    // Edite o nome do item
    let new_name = "New Item".to_string();
    Item::edit_name(&conn, item.id.clone(), new_name.clone()).unwrap();

    // Encontre o item atualizado
    let item = Item::find_item(&conn, item_id.clone()).unwrap();
    assert!(item.is_some());
    let item = item.unwrap();
    assert_eq!(item.name, new_name);

    // Exclua o item
    Item::delete_one(&conn, item.id.clone()).unwrap();

    // Verifique se o item foi removido
    let item = Item::find_item(&conn, item.id.clone()).unwrap();
    assert!(item.is_none());

    // Exclua a conta
    Account::delete_one(&conn, account_id.clone()).unwrap();
    // Exclua o produto
    Product::delete_one(&conn, product_id.clone()).unwrap();
}

#[test]
fn test_account_crud() {
    let conn = setup();

    // Crie um usuário
    let username = "john";
    let password = "password";
    let user_id = User::create_one(&conn, username.to_string(), password.to_string())
        .unwrap()
        .clone();

    // Crie um produto
    let name = "Product A";
    let price = 10.0;
    let product_id = Product::create_one(&conn, name, price).unwrap().clone();

    // Crie uma conta
    let account_id = Account::create_account(&conn, user_id.clone()).unwrap();

    // Verifique se a conta foi criada corretamente
    let account = Account::find_one(&conn, account_id.clone()).unwrap();
    assert!(account.is_some());
    let account = account.unwrap();
    assert_eq!(account.id, account_id);
    assert_eq!(account.user_id, user_id);
    assert_eq!(account.paid_amount, 0.0);
    assert_eq!(account.account_total, 0.0);

    // Crie um item
    let name = "Item A".to_string();
    let quantity = 2;
    let price = 20.0;
    let item_id: String = Item::create_item(
        &conn,
        name.clone(),
        quantity,
        price,
        account_id.clone(),
        product_id.clone(),
    )
    .unwrap();

// Crie um pagamento
let amount = 100.0;
Payment::create_one(&conn, amount, account_id.clone()).unwrap();

    // Atualize os pagamentos e itens da conta
    let account = Account::find_one(&conn, account_id.clone()).unwrap();
    assert!(account.is_some());
    let account = account.unwrap();
    assert_eq!(account.items.unwrap().len(), 1);
    assert_eq!(account.payments.unwrap().len(), 1);
    assert_eq!(account.paid_amount, 100.0);
    assert_eq!(account.account_total, 40.0);

    // Exclua a conta
    Account::delete_one(&conn, account_id.clone()).unwrap();

    // Verifique se a conta foi removida corretamente
    let account = Account::find_one(&conn, account_id.clone()).unwrap();
    assert!(account.is_none());

    // Verifique se o item foi removido corretamente
    let item = Item::find_item(&conn, item_id.clone()).unwrap();
    assert!(item.is_none());

    // Exclua o usuário
    User::delete_one(&conn, user_id.clone()).unwrap();
}

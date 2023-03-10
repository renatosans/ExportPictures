#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

extern crate mysql;
extern crate serde;
extern crate diesel;

use mysql as my;
use mysql::prelude::*;
use serde::{Serialize, Deserialize};
//use diesel::prelude::*;
//use diesel::mysql::MysqlConnection;


/*
fn get_products_orm(conn: &MysqlConnection) -> String {
    let products: Vec<Product> = products::table.load(conn)?;

    // Convert the results to a JSON string
    let json_string = serde_json::to_string(&products).unwrap_or_else(|_| {
        eprintln!("Error serializing vector to JSON string");
        std::process::exit(1);
    });

    json_string
}
*/

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn insert_product() {
    let pool = my::Pool::new("mysql://root:p@ssw0rd@localhost:3306/commercedb").unwrap_or_else(|_| {
        eprintln!("Error creating connection pool");
        std::process::exit(1);
    });
    let mut conn = pool.get_conn().unwrap_or_else(|_| {
        eprintln!("Error acquiring connection from pool");
        std::process::exit(1);
    });

    let query = "INSERT INTO `commercedb`.`produto` (nome, preco, foto, formatoImagem) VALUES (?, ?, ?, ?)";
    let params = vec!["Product 1", "1.99", "filename", "extension"];
    conn.exec_iter(query, params).unwrap_or_else(|_| {
        eprintln!("Error inserting product");
        std::process::exit(1);
    });

    // pool.release(conn);
}

#[tauri::command]
fn get_products() -> String {
    let pool = my::Pool::new("mysql://root:p@ssw0rd@localhost:3306/commercedb").unwrap_or_else(|_| {
        eprintln!("Error creating connection pool");
        std::process::exit(1);
    });
    let mut conn = pool.get_conn().unwrap_or_else(|_| {
        eprintln!("Error acquiring connection from pool");
        std::process::exit(1);
    });

    let products: Vec<Product> = conn.query_map("SELECT nome, descricao, preco, foto FROM produto", |(nome, descricao, preco, foto)| {
            Product { nome, descricao, preco, foto }
        }).unwrap_or_else(|e| {
            eprintln!("Error running SELECT query: {}", e );
            std::process::exit(1);
        });

    // Convert the results to a JSON string
    let json_string = serde_json::to_string(&products).unwrap_or_else(|_| {
        eprintln!("Error serializing vector to JSON string");
        std::process::exit(1);
    });

    json_string
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Product {
        nome: String,
        descricao: String,
        preco: f64,
        foto: String,
}

/*
fn retrieve_documents(conn: &mut my::Conn, filter: &str) -> Result<Vec<Document>, Box<dyn Error>> {
    let mut documents = Vec::new();
    let query = if filter.is_empty() {
        "SELECT * FROM `commercedb`.`produto`".to_string()
    } else {
        format!("SELECT * FROM `commercedb`.`produto` WHERE {}", filter)
    };
    let rows = conn.query(query)?;
    for row in rows {
        let (nome, descricao, preco, foto, formato_imagem) = my::from_row(row?);
        let formato_imagem = get_file_format(&formato_imagem);
        documents.push(Document {
                nome,
                descricao,
                preco,
                foto,
                formato_imagem,
        });
    }
    Ok(documents)
}
*/

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_products])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

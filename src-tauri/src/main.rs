#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("So {} you re trying to call RUST from Typescript", name)
}

#[tauri::command]
fn get_documents(conn: &mut my::Conn, filter: &str) -> Result<Vec<Document>, Box<dyn Error>> {
    let mut documents = Vec::new();
    let query = if filter.is_empty() {
        "SELECT * FROM `commercedb`.`produto`".to_string()
    } else {
        format!("SELECT * FROM `commercedb`.`produto` WHERE {}", filter)
    };
    let rows = conn.query(query)?;
    for row in rows {
        let (nome, foto, formato_imagem) = my::from_row(row?);
        let formato_imagem = get_file_format(&formato_imagem);
        documents.push(Document {
            nome,
            foto,
            formato_imagem,
        });
    }
    Ok(documents)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

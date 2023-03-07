use std::fs::rename;
use std::io;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};
use regex::Regex;

fn main() -> io::Result<()> {
    // Busca los ficheros con extension docx.pdf y muestra sus nombres y rutas
    println!("Ficheros con extension docx.pdf:");
    let re = Regex::new(r"\.docx\.pdf$").unwrap();
    for entry in WalkDir::new("/") {
        let entry = entry?;
        if entry.file_type().is_file() {
            let path = entry.path();
            if let Some(file_name) = path.file_name() {
                if let Some(file_name_str) = file_name.to_str() {
                    if re.is_match(file_name_str) {
                        println!("{}", path.display());

                        // Pide al usuario que seleccione si desea renombrar los ficheros
                        let mut input = String::new();
                        println!("¿Desea renombrar el fichero a .pdf? (s/n)");
                        io::stdin().read_line(&mut input)?;
                        if input.trim() == "s" {
                            // Obtiene la ruta y el nombre del fichero sin la extension
                            let mut new_path = PathBuf::from(path.parent().unwrap());
                            new_path.push(file_name_str.trim_end_matches(".docx.pdf"));
                            new_path.set_extension("pdf");

                            // Renombra el fichero
                            rename(path, new_path)?;
                            println!("Fichero renombrado: {}", new_path.display());
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

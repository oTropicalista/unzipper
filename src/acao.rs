// Todo o trabalho duro com os zips fica aqui
use std::fs;
use std::io;
use std::env;
use zip;

pub fn read_args(i: i32) {
    let args: Vec<_> = env::args().collect();
}

pub fn extrair() {
    println!("Extrair");
    let args: Vec<_> = env::args().collect();
    let fname = std::path::Path::new(&*args[2]);
    let file = fs::File::open(&fname).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();
}

//zipar um arquivo
pub fn zipar(fname: &str) -> zip::result::ZipResult<()> {
    let path = std::path::Path::new(fname);
    let file = std::fs::File::create(&path).unwrap();

    let mut zip = zip::ZipWriter::new(file);

    let options = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o755);
    zip.start_file("teste/ola.txt", options)?;    
    zip.write_all(b"Ola mundo zipado\n")?;

    zip.finish()?;


    Ok(())
}

//funcao para tratar o diretorio para zipar
//pegar o metodo para zipar
pub fn tratar_dir(x: &str, y: &str) {

    
}

pub fn zipar_dir(src: &str, dst: &str) -> zip::result::ZipResult<()> {
    //code
}

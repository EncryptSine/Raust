use anyhow::Result;
use std::path::Path;
use std::io::Write;

pub fn save_to_file(path: &Path, salt: &[u8], nonce: &[u8], data: &[u8]) -> Result<()> {

    let mut fichier = std::fs::File::create(path)?;
    fichier.write_all(salt)?;
    fichier.write_all(nonce)?;
    fichier.write_all(data)?;

    Ok(())

}

pub fn load_from_file(path: &Path) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>)> {

    let d = std::fs::read(path)?;
    if d.len() < 28 { return Err(anyhow::anyhow!("Fichier invalide")); }

    let salt = d[0..16].to_vec();
    let nonce = d[16..28].to_vec();
    let en = d[28..].to_vec();


    Ok((salt, nonce, en))
}
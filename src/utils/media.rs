use std::io;

use axum::body::Bytes;
use tokio::{fs::File, io::AsyncWriteExt};

pub async fn save_user_file(path: &str, file: Bytes) -> io::Result<()> {

    let mut file_writer = File::create(path).await?;
    file_writer.write_all(&file).await?;

    Ok(())
}
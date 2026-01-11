use crate::error::{AppError, AppResult};
use image::Luma;
use qrcode::QrCode;

pub fn generate_qr_code(url: &str) -> AppResult<Vec<u8>> {
    let code = QrCode::new(url.as_bytes())
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to generate QR code: {}", e)))?;

    let image = code.render::<Luma<u8>>().build();

    let mut buffer = Vec::new();

    image
        .write_to(
            &mut std::io::Cursor::new(&mut buffer),
            image::ImageFormat::Png,
        )
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to encode QR code: {}", e)))?;

    Ok(buffer)
}

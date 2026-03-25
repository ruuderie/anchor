use leptos::*;

pub fn generate_pdf(_name: &str, _role: &str) -> std::result::Result<Vec<u8>, String> {
    // Tectonic C++ build scripts fail on Apple Silicon's Graphite2 distribution.
    // Return empty mock for now.
    Ok(b"%PDF-1.4\n% dummy pdf".to_vec())
}

#[server(DownloadResume, "/api")]
pub async fn download_resume() -> Result<Vec<u8>, ServerFnError> {
    match generate_pdf("Erik Henderson", "Systems Architect") {
        Ok(pdf_bytes) => Ok(pdf_bytes),
        Err(e) => Err(ServerFnError::ServerError(e)),
    }
}

pub fn print() {
    tracing::info!("CARGO_PKG_VERSION: {}", env!("CARGO_PKG_VERSION"));
    tracing::info!("VERGEN_BUILD_TIMESTAMP: {}", env!("VERGEN_BUILD_TIMESTAMP"));
    tracing::info!("VERGEN_GIT_SHA: {}", env!("VERGEN_GIT_SHA"));
    tracing::info!("VERGEN_GIT_COMMIT_TIMESTAMP: {}", env!("VERGEN_GIT_COMMIT_TIMESTAMP"));
}
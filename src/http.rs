#[cfg(all(feature = "http-reqwest", feature = "http-wreq"))]
compile_error!("Enable only one of the features: `http-reqwest` or `http-wreq`.");

#[cfg(not(any(feature = "http-reqwest", feature = "http-wreq")))]
compile_error!("Enable one HTTP client feature: `http-reqwest` or `http-wreq`.");

#[cfg(feature = "http-wreq")]
use wreq_util::Emulation;

#[cfg(feature = "http-reqwest")]
/// Build a reqwest HTTP client with sensible defaults.
pub fn http_client() -> reqwest::Client {
    reqwest::ClientBuilder::new()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .unwrap()
}

#[cfg(feature = "http-wreq")]
/// Build a wreq HTTP client with sensible defaults.
pub fn http_client() -> wreq::Client {
    wreq::ClientBuilder::new()
        .emulation(Emulation::Chrome137)
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .unwrap()
}

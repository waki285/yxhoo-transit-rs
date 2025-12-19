#[cfg(feature = "http-wreq")]
use wreq_util::Emulation;

#[cfg(feature = "http-reqwest")]
pub fn http_client() -> reqwest::Client {
    reqwest::ClientBuilder::new()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .unwrap()
}

#[cfg(feature = "http-wreq")]
pub fn http_client() -> wreq::Client {
    wreq::ClientBuilder::new()
        .emulation(Emulation::Chrome137)
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .unwrap()
}

/// Structure to configure `Website` crawler
/// ```rust
/// use website_crawler::spider::website::Website;
/// let mut website: Website = Website::new("https://choosealicense.com");
/// website.configuration.user_agent = "Android";
/// website.crawl().await;
/// ```
#[derive(Debug, Default)]
pub struct Configuration {
    /// User-Agent
    pub user_agent: String
}

// todo: extra headers
// proxy: urls

impl Configuration {
    /// Represents crawl configuration for a website.
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

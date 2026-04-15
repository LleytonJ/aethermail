//! HTML text extraction utility
//!
//! Provides clean text extraction from HTML email content,
//! removing CSS styles and JavaScript code.

use scraper::{Html, Selector};

/// HTML text extractor for email content
pub struct HtmlExtractor;

impl HtmlExtractor {
    /// Create a new HTML extractor
    pub fn new() -> Self {
        HtmlExtractor
    }

    /// Extract clean text from HTML content
    /// Removes CSS styles and JavaScript code
    pub fn extract_clean_text(html: &str) -> String {
        // Parse HTML document
        let document = Html::parse_document(html);

        // Create selector for visible content (exclude style, script, head)
        let selector = Selector::parse("body :not(style):not(script):not(head)")
            .expect("Invalid CSS selector");

        // Extract text from matching elements
        let texts: Vec<String> = document
            .select(&selector)
            .map(|el| {
                el.text()
                    .collect::<Vec<_>>()
                    .join("")
            })
            .filter(|t| !t.trim().is_empty())
            .collect();

        // Join and clean up whitespace
        let result = texts.join(" ");
        result
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Check if text content is clean (no CSS/JS patterns)
    pub fn is_text_clean(text: &str) -> bool {
        // Detect common CSS/JS patterns that indicate unclean extraction
        let css_patterns = [
            "font-family:",
            "font-size:",
            "{",
            "}",
            ".ck-content",
            ".image-style",
            "word-wrap:",
            "white-space:",
            "word-break:",
            "border-box",
            "box-sizing:",
            "margin-left:",
            "max-width:",
        ];

        // Check for CSS pattern density
        let pattern_count = css_patterns
            .iter()
            .filter(|p| text.contains(*p))
            .count();

        // If more than 3 CSS patterns found, text is likely unclean
        pattern_count <= 3
    }
}

impl Default for HtmlExtractor {
    fn default() -> Self {
        Self::new()
    }
}
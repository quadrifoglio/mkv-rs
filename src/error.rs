//! Error handling functionality.

error_chain! {
    errors {
        ElementNotFound {
            description("Element not found")
        }
    }

    foreign_links {
        Ebml(::ebml::error::Error);
    }
}


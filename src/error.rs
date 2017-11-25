//! Error handling functionality.

error_chain!{
    errors {
        ElementNotFound {
            description("The required element was not found")
        }

        UnexpectedElement(expected: u64, got: u64) {
            description("Received an unexpected element"),
            display("Unexpected element: was expecting {:X}, but got {:X}", expected, got)
        }
    }

    foreign_links {
        Ebml(::ebml::error::Error);
    }
}

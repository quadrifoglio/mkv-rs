//! Error handling functionality.

error_chain! {
    errors {
        ElementNotFound {
            description("Element not found")
        }

        UnexpectedElement(expected: u64, found: u64) {
            description("Unexpected element"),
            display("Unexpected element: was expecting '0x{:X}', but got '0x{:X}'", expected, found)
        }

        InvalidElementValue(el: u64) {
            description("Got an invalid value for an element"),
            display("Invalid value for element '0x{:X}'", el)
        }
    }

    foreign_links {
        Ebml(::ebml::error::Error);
    }
}


error_chain!{
    foreign_links {
        Ebml(::ebml::error::Error);
    }
}

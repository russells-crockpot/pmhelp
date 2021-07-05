#[macro_export]
macro_rules! from_parens {
    ($from:expr) => {
        {
            let content;
            let _ = ::syn::parenthesized!(content in $from);
            content
        }
    }
}

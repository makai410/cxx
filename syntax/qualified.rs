use syn::ext::IdentExt;
use syn::parse::{ParseStream, Result};
use syn::{Ident, Token};

pub struct QualifiedName {
    pub segments: Vec<Ident>,
}

impl QualifiedName {
    pub fn parse_unquoted(input: ParseStream) -> Result<Self> {
        let mut segments = Vec::new();
        let mut trailing_punct = true;
        while trailing_punct && input.peek(Ident::peek_any) {
            let ident = Ident::parse_any(input)?;
            segments.push(ident);
            let colons: Option<Token![::]> = input.parse()?;
            trailing_punct = colons.is_some();
        }
        if segments.is_empty() {
            return Err(input.error("expected path"));
        } else if trailing_punct {
            return Err(input.error("expected path segment"));
        }
        Ok(QualifiedName { segments })
    }
}

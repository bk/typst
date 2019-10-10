//! Alignment function.

use super::prelude::*;
use crate::layout::Alignment;


/// Allows to align content in different ways.
#[derive(Debug, PartialEq)]
pub struct AlignFunc {
    alignment: Alignment,
    body: Option<SyntaxTree>,
}

impl Function for AlignFunc {
    fn parse(header: &FuncHeader, body: Option<&str>, ctx: ParseContext)
        -> ParseResult<Self> where Self: Sized {

        if header.args.len() != 1 || !header.kwargs.is_empty() {
            return err("expected exactly one positional argument specifying the alignment");
        }

        let alignment = if let Expression::Ident(ident) = &header.args[0] {
            match ident.as_str() {
                "left" => Alignment::Left,
                "right" => Alignment::Right,
                s => return err(format!("invalid alignment specifier: '{}'", s)),
            }
        } else {
            return err(format!("expected alignment specifier, found: '{}'", header.args[0]));
        };

        let body = if let Some(body) = body {
            Some(parse(body, ctx)?)
        } else {
            None
        };

        Ok(AlignFunc { alignment, body })
    }

    fn layout(&self, mut ctx: LayoutContext) -> LayoutResult<Option<Layout>> {
        if let Some(body) = &self.body {
            // Override the previous alignment and do the layouting.
            ctx.space.alignment = self.alignment;
            layout(body, ctx)
                .map(|l| Some(Layout::Boxed(l)))
        } else {
            unimplemented!("context-modifying align func")
        }
    }
}

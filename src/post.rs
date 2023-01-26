use std::{io::Write, iter::Filter};

use pulldown_cmark::{Event, Parser};

pub fn parse<'p>(source: &'p str) -> Parser<'p, 'p> {
    Parser::new(source)
}

pub trait Post<'p>: Iterator<Item = Event<'p>> {
    fn skip_closing(&mut self) -> Filter<Self, _> {
        self.filter(|event| match event {
            Event::End(_) => false,
            _ => true,
        })
    }
    fn write(&mut self, writer: impl Write) -> std::io::Result<()> {
        pulldown_cmark::html::write_html(writer, self)
    }
    fn build(&mut self) -> String {
        let mut html = String::new();
        pulldown_cmark::html::push_html(&mut html, self);
        html
    }
}

impl<'p, I> Post<'p> for I where I: Iterator<Item = Event<'p>> {}

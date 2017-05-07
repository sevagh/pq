use serde_json::ser::{Formatter, PrettyFormatter};
use serde_json::error::{Result, Error};
use std::io::Write;

#[derive(Default)]
pub struct NewlineFormatter {
    pretty: PrettyFormatter<'static>,
    depth: usize,
}

impl Formatter for NewlineFormatter {
    fn begin_array<W: ?Sized + Write>(&mut self, w: &mut W) -> Result<()> {
        self.pretty.begin_array(w)
    }
    fn end_array<W: ?Sized + Write>(&mut self, w: &mut W) -> Result<()> {
        self.pretty.end_array(w)
    }
    fn begin_array_value<W: ?Sized + Write>(&mut self, w: &mut W, first: bool) -> Result<()> {
        self.pretty.begin_array_value(w, first)
    }
    fn end_array_value<W: ?Sized + Write>(&mut self, w: &mut W) -> Result<()> {
        self.pretty.end_array_value(w)
    }
    fn begin_object<W: ?Sized + Write>(&mut self, w: &mut W) -> Result<()> {
        self.depth += 1;
        self.pretty.begin_object(w)
    }
    fn end_object<W: ?Sized + Write>(&mut self, w: &mut W) -> Result<()> {
        self.pretty.end_object(w).and_then(|()| {
            self.depth -= 1;
            if self.depth == 0 {
                match w.write_all(b"\n") {
                    Ok(_) => Ok(()),
                    Err(e) => Err(Error::from(e)),
                }
            } else {
                Ok(())
            }
        })
    }
    fn begin_object_key<W: ?Sized + Write>(&mut self, w: &mut W, first: bool) -> Result<()> {
        self.pretty.begin_object_key(w, first)
    }
    fn begin_object_value<W: ?Sized + Write>(&mut self, w: &mut W) -> Result<()> {
        self.pretty.begin_object_value(w)
    }
    fn end_object_value<W: ?Sized + Write>(&mut self, w: &mut W) -> Result<()> {
        self.pretty.end_object_value(w)
    }
}

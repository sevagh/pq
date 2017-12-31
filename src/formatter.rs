use serde_json::ser::{CompactFormatter, Formatter, PrettyFormatter};
use erased_serde_json::Formatter as ErasedFormatter;
use std::io::{self, Write};
use std::boxed::Box;

pub struct CustomFormatter {
    formatter: Box<ErasedFormatter>,
    depth: usize,
}

impl CustomFormatter {
    pub fn new(is_tty: bool) -> Self {
        let f: Box<ErasedFormatter> = if is_tty {
            Box::new(PrettyFormatter::default())
        } else {
            Box::new(CompactFormatter)
        };
        CustomFormatter {
            formatter: f,
            depth: 0,
        }
    }
}

impl Formatter for CustomFormatter {
    fn begin_array<W: ?Sized + Write>(&mut self, w: &mut W) -> io::Result<()> {
        self.formatter.begin_array(w)
    }
    fn end_array<W: ?Sized + Write>(&mut self, w: &mut W) -> io::Result<()> {
        self.formatter.end_array(w)
    }
    fn begin_array_value<W: ?Sized + Write>(&mut self, w: &mut W, first: bool) -> io::Result<()> {
        self.formatter.begin_array_value(w, first)
    }
    fn end_array_value<W: ?Sized + Write>(&mut self, w: &mut W) -> io::Result<()> {
        self.formatter.end_array_value(w)
    }
    fn begin_object<W: ?Sized + Write>(&mut self, w: &mut W) -> io::Result<()> {
        self.depth += 1;
        self.formatter.begin_object(w)
    }
    fn end_object<W: ?Sized + Write>(&mut self, w: &mut W) -> io::Result<()> {
        self.formatter.end_object(w).and_then(|()| {
            self.depth -= 1;
            if self.depth == 0 {
                w.write_all(b"\n")
            } else {
                Ok(())
            }
        })
    }
    fn begin_object_key<W: ?Sized + Write>(&mut self, w: &mut W, first: bool) -> io::Result<()> {
        self.formatter.begin_object_key(w, first)
    }
    fn begin_object_value<W: ?Sized + Write>(&mut self, w: &mut W) -> io::Result<()> {
        self.formatter.begin_object_value(w)
    }
    fn end_object_value<W: ?Sized + Write>(&mut self, w: &mut W) -> io::Result<()> {
        self.formatter.end_object_value(w)
    }
}

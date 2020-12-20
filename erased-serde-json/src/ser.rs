use std::io;

use serde_json::ser::CharEscape;

pub trait Formatter {
    fn erased_write_null(&mut self, writer: &mut dyn io::Write) -> Result<(), io::Error>;
    fn erased_write_bool(
        &mut self,
        writer: &mut dyn io::Write,
        value: bool,
    ) -> Result<(), io::Error>;
    fn erased_write_i8(&mut self, writer: &mut dyn io::Write, value: i8) -> Result<(), io::Error>;
    fn erased_write_i16(&mut self, writer: &mut dyn io::Write, value: i16)
        -> Result<(), io::Error>;
    fn erased_write_i32(&mut self, writer: &mut dyn io::Write, value: i32)
        -> Result<(), io::Error>;
    fn erased_write_i64(&mut self, writer: &mut dyn io::Write, value: i64)
        -> Result<(), io::Error>;
    fn erased_write_u8(&mut self, writer: &mut dyn io::Write, value: u8) -> Result<(), io::Error>;
    fn erased_write_u16(&mut self, writer: &mut dyn io::Write, value: u16)
        -> Result<(), io::Error>;
    fn erased_write_u32(&mut self, writer: &mut dyn io::Write, value: u32)
        -> Result<(), io::Error>;
    fn erased_write_u64(&mut self, writer: &mut dyn io::Write, value: u64)
        -> Result<(), io::Error>;
    fn erased_write_f32(&mut self, writer: &mut dyn io::Write, value: f32)
        -> Result<(), io::Error>;
    fn erased_write_f64(&mut self, writer: &mut dyn io::Write, value: f64)
        -> Result<(), io::Error>;
    fn erased_begin_string(&mut self, writer: &mut dyn io::Write) -> Result<(), io::Error>;
    fn erased_end_string(&mut self, writer: &mut dyn io::Write) -> Result<(), io::Error>;
    fn erased_write_string_fragment(
        &mut self,
        writer: &mut dyn io::Write,
        fragment: &str,
    ) -> Result<(), io::Error>;
    fn erased_write_char_escape(
        &mut self,
        writer: &mut dyn io::Write,
        char_escape: CharEscape,
    ) -> Result<(), io::Error>;
    fn erased_begin_array(&mut self, writer: &mut dyn io::Write) -> Result<(), io::Error>;
    fn erased_end_array(&mut self, writer: &mut dyn io::Write) -> Result<(), io::Error>;
    fn erased_begin_array_value(
        &mut self,
        writer: &mut dyn io::Write,
        first: bool,
    ) -> Result<(), io::Error>;
    fn erased_end_array_value(&mut self, writer: &mut dyn io::Write) -> Result<(), io::Error>;
    fn erased_begin_object(&mut self, writer: &mut dyn io::Write) -> Result<(), io::Error>;
    fn erased_end_object(&mut self, writer: &mut dyn io::Write) -> Result<(), io::Error>;
    fn erased_begin_object_key(
        &mut self,
        writer: &mut dyn io::Write,
        first: bool,
    ) -> Result<(), io::Error>;
    fn erased_end_object_key(&mut self, writer: &mut dyn io::Write) -> Result<(), io::Error>;
    fn erased_begin_object_value(&mut self, writer: &mut dyn io::Write) -> Result<(), io::Error>;
    fn erased_end_object_value(&mut self, writer: &mut dyn io::Write) -> Result<(), io::Error>;
}

impl<T> Formatter for T
where
    T: serde_json::ser::Formatter,
{
    fn erased_write_null(&mut self, w: &mut dyn io::Write) -> Result<(), io::Error> {
        self.write_null(w)
    }
    fn erased_write_bool(&mut self, w: &mut dyn io::Write, v: bool) -> Result<(), io::Error> {
        self.write_bool(w, v)
    }
    fn erased_write_i8(&mut self, w: &mut dyn io::Write, v: i8) -> Result<(), io::Error> {
        self.write_i8(w, v)
    }
    fn erased_write_i16(&mut self, w: &mut dyn io::Write, v: i16) -> Result<(), io::Error> {
        self.write_i16(w, v)
    }
    fn erased_write_i32(&mut self, w: &mut dyn io::Write, v: i32) -> Result<(), io::Error> {
        self.write_i32(w, v)
    }
    fn erased_write_i64(&mut self, w: &mut dyn io::Write, v: i64) -> Result<(), io::Error> {
        self.write_i64(w, v)
    }
    fn erased_write_u8(&mut self, w: &mut dyn io::Write, v: u8) -> Result<(), io::Error> {
        self.write_u8(w, v)
    }
    fn erased_write_u16(&mut self, w: &mut dyn io::Write, v: u16) -> Result<(), io::Error> {
        self.write_u16(w, v)
    }
    fn erased_write_u32(&mut self, w: &mut dyn io::Write, v: u32) -> Result<(), io::Error> {
        self.write_u32(w, v)
    }
    fn erased_write_u64(&mut self, w: &mut dyn io::Write, v: u64) -> Result<(), io::Error> {
        self.write_u64(w, v)
    }
    fn erased_write_f32(&mut self, w: &mut dyn io::Write, v: f32) -> Result<(), io::Error> {
        self.write_f32(w, v)
    }
    fn erased_write_f64(&mut self, w: &mut dyn io::Write, v: f64) -> Result<(), io::Error> {
        self.write_f64(w, v)
    }
    fn erased_begin_string(&mut self, w: &mut dyn io::Write) -> Result<(), io::Error> {
        self.begin_string(w)
    }
    fn erased_end_string(&mut self, w: &mut dyn io::Write) -> Result<(), io::Error> {
        self.end_string(w)
    }
    fn erased_write_string_fragment(
        &mut self,
        w: &mut dyn io::Write,
        fragment: &str,
    ) -> Result<(), io::Error> {
        self.write_string_fragment(w, fragment)
    }
    fn erased_write_char_escape(
        &mut self,
        w: &mut dyn io::Write,
        char_escape: CharEscape,
    ) -> Result<(), io::Error> {
        self.write_char_escape(w, char_escape)
    }
    fn erased_begin_array(&mut self, w: &mut dyn io::Write) -> Result<(), io::Error> {
        self.begin_array(w)
    }
    fn erased_end_array(&mut self, w: &mut dyn io::Write) -> Result<(), io::Error> {
        self.end_array(w)
    }
    fn erased_begin_array_value(
        &mut self,
        w: &mut dyn io::Write,
        first: bool,
    ) -> Result<(), io::Error> {
        self.begin_array_value(w, first)
    }
    fn erased_end_array_value(&mut self, w: &mut dyn io::Write) -> Result<(), io::Error> {
        self.end_array_value(w)
    }
    fn erased_begin_object(&mut self, w: &mut dyn io::Write) -> Result<(), io::Error> {
        self.begin_object(w)
    }
    fn erased_end_object(&mut self, w: &mut dyn io::Write) -> Result<(), io::Error> {
        self.end_object(w)
    }
    fn erased_begin_object_key(
        &mut self,
        w: &mut dyn io::Write,
        first: bool,
    ) -> Result<(), io::Error> {
        self.begin_object_key(w, first)
    }
    fn erased_end_object_key(&mut self, w: &mut dyn io::Write) -> Result<(), io::Error> {
        self.end_object_key(w)
    }
    fn erased_begin_object_value(&mut self, w: &mut dyn io::Write) -> Result<(), io::Error> {
        self.begin_object_value(w)
    }
    fn erased_end_object_value(&mut self, w: &mut dyn io::Write) -> Result<(), io::Error> {
        self.end_object_value(w)
    }
}

macro_rules! impl_formatter_for_trait_object {
    ($ty:ty) => {
        impl<'a> serde_json::ser::Formatter for $ty {
            fn write_null<W: ?Sized + io::Write>(
                &mut self,
                mut w: &mut W,
            ) -> Result<(), io::Error> {
                self.erased_write_null(&mut w)
            }
            fn write_bool<W: ?Sized + io::Write>(
                &mut self,
                mut w: &mut W,
                v: bool,
            ) -> Result<(), io::Error> {
                self.erased_write_bool(&mut w, v)
            }
            fn write_i8<W: ?Sized + io::Write>(
                &mut self,
                mut w: &mut W,
                v: i8,
            ) -> Result<(), io::Error> {
                self.erased_write_i8(&mut w, v)
            }
            fn write_i16<W: ?Sized + io::Write>(
                &mut self,
                mut w: &mut W,
                v: i16,
            ) -> Result<(), io::Error> {
                self.erased_write_i16(&mut w, v)
            }
            fn write_i32<W: ?Sized + io::Write>(
                &mut self,
                mut w: &mut W,
                v: i32,
            ) -> Result<(), io::Error> {
                self.erased_write_i32(&mut w, v)
            }
            fn write_i64<W: ?Sized + io::Write>(
                &mut self,
                mut w: &mut W,
                v: i64,
            ) -> Result<(), io::Error> {
                self.erased_write_i64(&mut w, v)
            }
            fn write_u8<W: ?Sized + io::Write>(
                &mut self,
                mut w: &mut W,
                v: u8,
            ) -> Result<(), io::Error> {
                self.erased_write_u8(&mut w, v)
            }
            fn write_u16<W: ?Sized + io::Write>(
                &mut self,
                mut w: &mut W,
                v: u16,
            ) -> Result<(), io::Error> {
                self.erased_write_u16(&mut w, v)
            }
            fn write_u32<W: ?Sized + io::Write>(
                &mut self,
                mut w: &mut W,
                v: u32,
            ) -> Result<(), io::Error> {
                self.erased_write_u32(&mut w, v)
            }
            fn write_u64<W: ?Sized + io::Write>(
                &mut self,
                mut w: &mut W,
                v: u64,
            ) -> Result<(), io::Error> {
                self.erased_write_u64(&mut w, v)
            }
            fn write_f32<W: ?Sized + io::Write>(
                &mut self,
                mut w: &mut W,
                v: f32,
            ) -> Result<(), io::Error> {
                self.erased_write_f32(&mut w, v)
            }
            fn write_f64<W: ?Sized + io::Write>(
                &mut self,
                mut w: &mut W,
                v: f64,
            ) -> Result<(), io::Error> {
                self.erased_write_f64(&mut w, v)
            }
            fn begin_string<W: ?Sized + io::Write>(
                &mut self,
                mut w: &mut W,
            ) -> Result<(), io::Error> {
                self.erased_begin_string(&mut w)
            }
            fn end_string<W: ?Sized + io::Write>(
                &mut self,
                mut w: &mut W,
            ) -> Result<(), io::Error> {
                self.erased_end_string(&mut w)
            }
            fn write_string_fragment<W: ?Sized + io::Write>(
                &mut self,
                mut w: &mut W,
                fragment: &str,
            ) -> Result<(), io::Error> {
                self.erased_write_string_fragment(&mut w, fragment)
            }
            fn write_char_escape<W: ?Sized + io::Write>(
                &mut self,
                mut w: &mut W,
                char_escape: CharEscape,
            ) -> Result<(), io::Error> {
                self.erased_write_char_escape(&mut w, char_escape)
            }
            fn begin_array<W: ?Sized + io::Write>(
                &mut self,
                mut w: &mut W,
            ) -> Result<(), io::Error> {
                self.erased_begin_array(&mut w)
            }
            fn end_array<W: ?Sized + io::Write>(&mut self, mut w: &mut W) -> Result<(), io::Error> {
                self.erased_end_array(&mut w)
            }
            fn begin_array_value<W: ?Sized + io::Write>(
                &mut self,
                mut w: &mut W,
                first: bool,
            ) -> Result<(), io::Error> {
                self.erased_begin_array_value(&mut w, first)
            }
            fn end_array_value<W: ?Sized + io::Write>(
                &mut self,
                mut w: &mut W,
            ) -> Result<(), io::Error> {
                self.erased_end_array_value(&mut w)
            }
            fn begin_object<W: ?Sized + io::Write>(
                &mut self,
                mut w: &mut W,
            ) -> Result<(), io::Error> {
                self.erased_begin_object(&mut w)
            }
            fn end_object<W: ?Sized + io::Write>(
                &mut self,
                mut w: &mut W,
            ) -> Result<(), io::Error> {
                self.erased_end_object(&mut w)
            }
            fn begin_object_key<W: ?Sized + io::Write>(
                &mut self,
                mut w: &mut W,
                first: bool,
            ) -> Result<(), io::Error> {
                self.erased_begin_object_key(&mut w, first)
            }
            fn end_object_key<W: ?Sized + io::Write>(
                &mut self,
                mut w: &mut W,
            ) -> Result<(), io::Error> {
                self.erased_end_object_key(&mut w)
            }
            fn begin_object_value<W: ?Sized + io::Write>(
                &mut self,
                mut w: &mut W,
            ) -> Result<(), io::Error> {
                self.erased_begin_object_value(&mut w)
            }
            fn end_object_value<W: ?Sized + io::Write>(
                &mut self,
                mut w: &mut W,
            ) -> Result<(), io::Error> {
                self.erased_end_object_value(&mut w)
            }
        }
    };
}

impl_formatter_for_trait_object!(dyn Formatter);
impl_formatter_for_trait_object!(&'a mut dyn Formatter);
impl_formatter_for_trait_object!(&'a mut (dyn Formatter + Send));
impl_formatter_for_trait_object!(&'a mut (dyn Formatter + Sync));
impl_formatter_for_trait_object!(&'a mut (dyn Formatter + Send + Sync));

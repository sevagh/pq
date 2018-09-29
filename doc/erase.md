Type-erasing unsafe traits
==========================

[2017-08-31] Workaround to make non-object-safe traits object-safe
------------------------------------------------------------------

The problem began when I wanted to Box a `trait Formatter` in [pq](https://github.com/sevagh/pq). The serde_json Formatters (PrettyFormatter and CompactFormatter) both implement the [Formatter trait](https://docs.serde.rs/serde_json/ser/trait.Formatter.html#implementors) and I wanted to switch between pretty formatting in a tty and compact formatting outside a tty.

This is the error I got:

```rust
error[E0038]: the trait `serde_json::ser::Formatter` cannot be made into an 
object
 --> formatter.rs:7:5
  |
7 |     formatter: Box<Formatter>,
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `serde_json::ser::Formatter` cannot be made into an object
  |
  = note: method `write_null` has generic type parameters
```

### Object-safety in traits

I was pointed to [this article](https://github.com/rust-lang/rfcs/blob/master/text/0255-object-safety.md) and [this article](https://huonw.github.io/blog/2015/01/object-safety/) from the #rust IRC channel.

_tl;dr_: since the `trait Formatter` depends on type `W: ?Sized + io::Write`, the `?Sized` is a generic parameter which makes Formatter non-object-safe (and thus unusable in a Box). Side-note: [Sized is a special trait](https://huonw.github.io/blog/2015/01/the-sized-trait/) which is inherently always implemented, and an explict `?Sized` is required to indicate that an object is __not__ implementing Sized.

### Wrapper trait

From Reddit, I was pointed to [erased-serde](https://github.com/dtolnay/erased-serde), which contains type-erased copies of some serde traits - making them object safe and thus usable in a Box.

Here's a small example. Say here's our lib, `mylib`, with the non-oject-safe trait `UnsafeTrait`:

```rust
mod mylib {
    trait UnsafeTrait {
    	//?Sized ruins everything!
	fn foo<W: ?Sized + Write>(w: &mut W) -> Result<()>;
    }

    struct StructA { }
    impl UnsafeTrait for StructA { }

    struct StructB { }
    impl UnsafeTrait for StructB { }
}
```

Let's write a library called `erased_mylib` with a cross-implemented trait called `SafeTrait` with `?Sized` removed:

```rust
mod erased_mylib {
    extern crate mylib;

    use mylib;

    trait SafeTrait {
    	//no ?Sized
	fn safe_foo<W: Write>(w: &mut W) -> Result<()>
    }

    impl mylib::UnsafeTrait for SafeTrait {
	fn foo<W: ?Sized + Write>(w: &mut W) -> Result<()> {
	    self.safe_foo(&mut w);
	}
    }

    impl SafeTrait for mylib::UnsafeTrait {
	fn safe_foo<W: Write>(w: &mut W) -> Result<()> {
	    self.foo(&mut w);
	}
    }
}
```

Finally, we can use a `Box<SafeTrait>` for `StructA` and `StructB`:

```
# main.rs

use mylib::{UnsafeTrait, StructA, StructB};
use erased_mylib::SafeTrait;


//THIS WILL ERROR
let my_struct: Box<UnsafeTrait> = if cond1 { StructA::new() } else { StructB::new() };

//THIS WON'T
let my_struct: Box<SafeTrait> = if cond1 { StructA::new() } else { StructB::new() };
```

Make a safe wrapper trait, then cross-implement the safe and unsafe traits for each other.

You can see this solution for the serde_json Formatter type clearly in my [erased-serde-json code](https://github.com/sevagh/pq/tree/master/erased-serde-json).

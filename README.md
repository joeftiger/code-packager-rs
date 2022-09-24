# code-packager-rs
This program/library helps with packaging generational files for, e.g., student exercises.

## Example
Suppose you have two programing exercises on which the second exercise depends on the first.
Now you want to package the code for both exercise version and an example solution.

Of course you could split it into 3 different files like the following:

### First exercise
```rust
pub struct Foo {
  // TODO: add properties.
}
```

### Second exercise
We depend on the solution of the first exercise now:
```rust
pub struct Foo {
  // TODO: include your properties of the first exercise.
}

impl Foo {
  // TODO: Fill in the blanks to write a constructor.
  pub fn new( ... ) -> Self {
    Self { ... }
  }
}
```

### Example solution
This is a (final) solution to the above exercises.
```rust
pub struct Foo {
  bar: usize,
}

impl Foo {
  pub fn new(bar: usize) -> Self {
    Self { bar }
  }
}
```

Great.

Now imagine this on a much bigger scale where you might want to edit the exercises and the solution.
It gets rather cumbersome to maintain all those files now, doesn't it?

This is where this crate comes in! <br>

## Example using `code-packager-rs`
Instead of maintaining multiple versions of each file you can simply put tags in-place.
Give the appropriate tag descriptions to `code-packager-rs` and it does the rest.

Our above files can now be put into one single file like following:
```rust
pub struct Foo {
  // @first_exercise
  // TODO: add properties.
  // @end
  // @second_exercise
  // TODO: include your properties of the first exercise.
  // @end
  // @solution
  bar: usize,
  // @end
}

// @second_exercise
impl Foo {
  // TODO: Fill in the blanks to write a constructor.
  pub fn new( ... ) -> Self {
    Self { ... }
  }
}
// @end
// @solution
impl Foo {
  pub fn new(bar: usize) -> Self {
    Self { bar }
  }
}
```

Now we can generate the exercises and the solution like following:

### First Exercise
```shell
$ code-packager \
    --tag-prefix "//" \
    --end-tag @end \
    --include @first_exercise \
    --exclude @second_exercise,@solution
    source.rs \
    first_exercise.rs
```

### Second Exercise
```shell
$ code-packager \
    --tag-prefix "//" \
    --end-tag @end \
    --include @second_exercise \
    --exclude @first_exercise,@solution
    source.rs \
    second_exercise.rs
```

### Solution
```shell
$ code-packager \
    --tag-prefix "//" \
    --end-tag @end \
    --include @solution \
    --exclude @first_exercise,@second_exercise
    source.rs \
    solution.rs
```

You can also use multiple `--include` arguments if needed.

In any case, you can get a short description of the arguments passing the `--help` flag to the program:
```shell
$ code-packager --help
Usage: -i ARG... -e ARG... [-s ARG] [-p ARG] <INPUT> <OUTPUT>

Available positional items:
    <INPUT>   The input file to package.
    <OUTPUT>  The output file for the packaged file.

Available options:
    -i, --include <ARG>     Include code within the range of these tags.
    -e, --exclude <ARG>     Exclude code within the range of these tags, even if they are surrounded by a tag to be
                            included.
    -s, --end-tag <ARG>     The 'end' tag to mark the end of an in-/excluded section. (Default = @end)
    -p, --tag-prefix <ARG>  The prefix for each tag. (Default = //)
    -h, --help              Prints help information
    -V, --version           Prints version information
```

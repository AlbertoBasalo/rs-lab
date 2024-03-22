# Hello Rust 

Welcome to the world of Rust. A language that can be both intimidating and tremendously attractive. Don't be discouraged if it seems difficult or complicated initially; Rust has a worthwhile learning curve. Despite the toughness of his shell, deep down, he is adorable.

![Rust](1-hola_rust.jpg)

## Why Rust?

That is a good question with a clear answer. Rust is a programming language designed to create **high-performance** applications that report the most satisfaction to its developers, according to the [Stack Overflow survey] (https://insights.stackoverflow.com/survey/2020#technology-most-loved-dreaded-and-wanted-languages-loved).

It may seem like a newcomer to you, but it is more than established in large free software companies and foundations. Because it is already old, it was developed by Mozilla Research back in 2010 and has gained tremendous popularity since its first stable version in 2015. What features make it different?

- **Safe**: The compiler helps us avoid memory and concurrency errors.

- **Fast**: The code we write in Rust is as fast as the code written in C or C++.

- **Concurrent**: Allows us to write code that runs in parallel.

- **Concise**: It is a high-level language, with a simple and expressive syntax, although different...

- **Multi-paradigm**: You can use an imperative, functional, and object-oriented style.

- **Multi-platform**: Compiles the source code optimized for different platforms.

- **Free**: It is an open-source language with a very active community.

Let's see step by step how to treat this powerful friend. Are you ready?

## Installation

The official way to install Rust is to use [Rustup](https://rustup.rs/#). On a **Linux, Mac, or WSL** operating system, just run the following command in the terminal:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

###Windows

In Windows, you have to do something else because before installing Rust, you must have the [Visual Studio C++ compiler](https://visualstudio.microsoft.com/es/visual-cpp-build-tools/) as a prerequisite. From there, you can do it via a terminal or with a click.

- Chocolatey

If you use *Chocolatey* to manage packages, then you can install Rust on your Windows with the following command:

```bash
choco install rust
```

- Rustup-init.exe

Another option for Windows is to download the **rustup** installer from the [official page](https://www.rust-lang.org/es-ES/install.html) and run it.



### Terminal

In all cases, once the process is complete, you can verify that the installation was successful by executing the following command:

```bash
rustc --version
```

With Rust, you will have installed **Cargo**, the Rust package manager, and the compiler. Check that it is correctly installed with the following command:

```bash
cargo --version
```

> If you come from other languages, Cargo is like npm, pip, gem, composer, etc.

### Code Editor

To program in Rust, you can use any code editor or IDE. I use [Visual Studio Code](https://code.visualstudio.com/) with the extension [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer), but you can use whichever you prefer.

## Hello World

To start, let's create a new project with Cargo. Open your terminal and use the generator to create your first program:

```bash
cargo new hello
```

This will create a `hello` directory with the following structure:

```bash
Hello
├── Cargo.toml
└── src
     └── main.rs
```

The file `Cargo. toml` is the project configuration file, and `src/main.rs` is the entry point of the application.

> Cargo.toml is a configuration file in TOML (Tom's Obvious, Minimal Language) format. If you're not familiar with it, it's similar to JSON or YAML. The file contains information about the project, its dependencies, etc. Its function is similar to that of package.json in Node.js.

### The main function

Let's go with our *Hello World*. Open the file `src/main.rs` and check or write the following code:

```rust
fn main() {
     println!("Hello Rust!");
}
```

Every Rust program must have a `main` function that is executed when the application starts. To declare it, you use the `fn` keyword and a syntax similar to any C derivative. You already know `() {}`.

In this case, the `main` function prints the `Hello Rust!` message to the terminal. To do this, use the *macro* `println!` which is similar to a `console.log` in JavaScript.

> A macro is a function that is executed at compile time. In Rust, macros are identified by the exclamation bang `!` at the end of the name. We'll look at more macros later, but for now, think of them as special functions.

Time to compile and run the program. For that, we use Cargo again, which will search for the source code, compile it, and execute the result. Run the following command and enjoy the result:

```bash
cargo run
```

If everything went well, you will see a report of the build and the execution result with the message `Hello Rust!` in the terminal.

### Variables, immutable?

Let's do some programming. In Rust, variables are declared with the `let` keyword. If a value is assigned during declaration, the variable will be immutable, and its type will be automatically inferred.

Let's rewrite the above program so that it prints the programmer name extracted from a variable:

```rust
fn main() {
     let name = "Alberto Basalo";
     println!("Hello, {}!", name);
}
```

> Note the interpolation string `{}`; It's similar to JavaScript, for now...

As I told you, by default, in Rust, _variables are constants_. If you try to modify the value of a variable... the compiler will prevent you from doing so. We're going to try it. Change the value of the `name` variable and run the program again:

```rust
fn main() {
     let name = "Alberto Basalo";
     name = "Rust";
     println!("Hello, {}!", name);
}
```

Now, when launching `cargo run`, the compiler tells us that it cannot assign a new value to an **immutable** variable:

```bash
error[E0384]: cannot assign twice to immutable variable `name`
  --> src/main.rs:3:5
   |
2 | let name = "Alberto Basalo";
   | ------ first assignment to `name`
3 | name = "Rust";
   | ^^^^^^^^^^^^^^^ cannot assign twice to immutable variable
```

To fix this, we can declare the variable as **mutable** with the `mut` keyword:

```rust
fn main() {
     let mut name = "Alberto Basalo";
     name = "Rust";
     println!("Hello, {}!", name);
}
```

Now it compiles and works... but we still receive a warning:

```bash
warning: assigned value to `name` is never read
  --> src/main.rs:2:9
   |
2 | let mut name = "Alberto Basalo";
   | ^^^^^^^^^
   |
   = help: maybe it is overwritten before being read?
   = note: `#[warn(unused_assignments)]` on by default
```

This is the level of detail that the Rust compiler has. It warns us that the variable `name` is assigned a value that is never used because it is overwritten. To avoid embarrassment, we can remove the initial assignment:

```rust
fn main() {
     let mut name;
     name = "Rust";
     println!("Hello, {}!", name);
}
```

But then, the compiler shows us another _warning_: why declare something that does not change mutable?: You are right.

```bash
warning: variable does not need to be mutable
  --> src\main.rs:2:9
   |
2 | let mut name;
   | ----^^^^^^
   | |
   | help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default

warning: `hello` (bin "hello") generated 1 warning (run `cargo fix --bin "hello"` to apply 1 suggestion)
```

Fortunately, the fix is simple. We can remove the `mut` keyword, and the program will compile without errors or warnings. In addition to doing it by hand, you can use the IDE's online help or even by executing the command suggested by the compiler:

```bash
cargo fix --bin "hello"
```

> This is similar to `eslint --fix` in JavaScript.

The final result is the following:

```rust
fn main() {
     let name;
     name = "Rust";
     println!("Hello, {}!", name);
}
```

It is a syntax that separates the declaration from the assignment. This is useful for declaring variables that are initialized later, although it is also not the most common. But back to the case of mutability. How could we have used that feature appropriately?

```rust
fn main() {
     let mut name = "Rust";
     println!("Hello, {}!", name);
     name = "Alberto Basalo";
     print!("By {}.", name)
}
```

In this case, the variable `name` is declared mutable because its value changes during the execution of the program (first the language, then the programmer). Finally, it compiles without any errors or warnings, and the result is the following:

```bash
Hello Rust!
By Alberto Basalo.
```

## Data types

So far, we have only worked with text strings, and we haven't even declared the data type. How does the compiler know that `name` is a text string?

We have already seen that Rust automatically infers the data type, like TypeScript. Your editor may show it to you embedded as a hint, but you can also declare the data type explicitly:

```rust
fn main() {
     let mut name: &str;
     name = "Rust";
     println!("Hello, {}!", name);
     name = "Alberto Basalo";
     print!("By {}.", name)
}
```

> What the hell is this syntax? Don't worry; the `&str` data type is a primitive data type that represents a reference to a character string. We will see the chains in depth later.

And what types of data can we use? For Rust, data types fall into two categories: scalars and composites. In this article, we are going to look at scalar types, especially numerical ones. This is how we become familiar with how detailed Rust is about everything.

![Rust Data Types](1-rust_data_types.png)

### Numerical scalar types

Numeric types are treated based on their sign, their size, and their decimal precision. Sometimes, a picture is worth a thousand words, so I'll leave you with the family photo.

Family tree of numerical scalar types:

```bash
numerical
├── integers
│ ├── with sign
│ │ ├── i8
│ │ ├── i16
│ │ ├── i32
│ │ ├── i64
│ │ ├── i128
│ │ └── isize
│ └── without sign
│ ├── u8
│ ├── u16
│ ├── u32
│ ├── u64
│ ├── u128
│ └── usize
└── floating point
     ├── f32
     └── f64
```

Again, everything is clearer with an example to give you an idea of the limits and uses of each numeric type in Rust:

```rust
     // maximum unsigned integer u128
     let max_u128: u128 = 340282366920938463463374607431768211455;
     println!("The maximum unsigned integer is {}.", max_u128);
     // maximum signed integer i128
     let max_i128: i128 = 170141183460469231731687303715884105727;
     println!("The maximum + signed integer is {}.", max_i128);
     // minimum signed integer i128
     let min_i128: i128 = -170141183460469231731687303715884105728;
     println!("The minimum - signed integer is {}.", min_i128);
     // to float min/max
     let min_f64: f64 = min_i128 as f64 / max_u128 as f64;
     println!("The ratio min/max is {}.", min_f64);
     // square root of 2
     let sqrt2: f64 = 2.0_f64.sqrt();
     println!("The square root of 2 is {}.", sqrt2);
     // square root of -1, no imagination
     let sqrt_1: f64 = (-1.0_f64).sqrt();
     println!("The square root of -1 is {}.", sqrt_1);
     // division by zero, don't panic
     let div0: f64 = 1.0_f64 / 0.0_f64;
     println!("The division by zero is {}.", div0);
     // is even or odd, a boolean question
     let is_even: bool = max_u128 % 2 == 0;
     println!("Is the maximum unsigned integer even? : {}.", is_even);
```

A couple of things to highlight from the example:

- You cannot mix types happily in an expression.

- Types are not converted implicitly.

So, we have to do the conversions explicitly with the operator. Or use values with explicit types since a `_` suffix can be added to any number to force its specific type. For example, `2.0_f64` is the number 2 but in 64-bit floating point.

As you can see, Rust is very strict with data types and the operations that can be performed with them. However, this is an advantage because it helps us avoid programming errors and efficiently manage resource and memory consumption if you choose the right type, of course.

### Strings

This is another world. And today, we're just going to scratch the surface. Text strings are declared with double quotes `"` and can contain any Unicode character. So far as expected.

As for types, things get complicated because values can be typed as `&str` or `String`. And the steps from one to the other are a bit confusing. But don't worry because it is simpler than it seems in practice. Let's see it with an example to get used to reading more Rust code:

```rust
fn main() {
     // working with strings
     let bye: String = String::from("Bye");
     let see_you: &str = "See you soon!";
     let mut bye_see_you: String = bye;
     bye_see_you.push_str(", ");
     bye_see_you.push_str(see_you);
     println!("{}", bye_see_you);
}
```

It's a little weird to start with, but the differences in declarations and assignments come because `&str` is a reference while `String` is a composite data type. We will see all of this later, and it will make much more sense.

So far, in the first lesson, we have seen how to install Rust, create a new project with Cargo, and write our first program. Also, it has an approach to variables and scalar data types. I leave you a link to the lab I use for this series in case you want to review the code or clone it and run it on your machine:

> [Rust laboratory](https://github.com/AlbertoBasalo/rs-lab)

I hope you liked it and were curious to learn more about Rust. We will see control structures and compound data types in future posts. Algorithms and data structures, here we come!


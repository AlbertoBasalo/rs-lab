# Rust Programs

I begin this article with a tribute to Niklaus Wirth, recently deceased, author of the book "Algorithms + Data Structures = Programs" with which I learned to program in Pascal. On the cover of the first edition of the book it reads:

> "This book is for those who want to write programs that are efficient and make good use of computer resources.
> Niklaus Wirth"

I think it is a quote that is most applicable to Rust, a programming language that has been designed to be efficient and secure. Let's see what data structures and flow control Rust offers us as tools so that code artisans can write programs.

![Rust structs](2-rust-structs.png)

## Pre-requirements

Obviously, I know that you know how to program and that this content is the foundation of any language, also in Rust, everything will be familiar to you. But, I hope it takes you back to your first steps in programming when you learned how to use data structures and control flow. And a certain nostalgia for the simple.

In case it is your first time with Rust, I offer you my first article, [Hello Rust](https://www.linkedin.com/pulse/hola-rust-alberto-basalo-qrbbf/?trackingId=UlQBak8uQv6cns776U4acA%3D%3D), where I explain how to install Rust and write your first program by familiarizing yourself with its primitive types, you know, string and numbers, many types of numbers.

## Data structures.

After these primitive data types, the next thing you learn in any language is to create complex structures or types. In Rust, we have several ways to define data structures, each with its own characteristics and uses.

### Tuples

- Tuples are a sequence of values of different types.
- The values of a tuple are accessed through its index, which starts at zero.
- They do not have a declaration per se, just a series of data in parentheses.

They are useful when you need to group values quickly, for example as a result of function calls.

```rust
let tuple_book = ("Algorithms + Data Structures = Programs", "Niklaus Wirth", 382);
let pages = tuple_book.2;
```

### Structures

- Structures are improved tuples as they indicate the purpose of each value in a named property.
- The fields of a structure are declared and used in any order since they are accessed by name, not by index.
- The syntax includes definition using `{}` and accessing fields using the `.` operator and the field name.

```rust
struct Book {
     title:String,
     author:String,
     pages: u32,
}
let struct_book = Book {
     author: "Niklaus Wirth".to_string(),
     title: "Algorithms + Data Structures = Programs".to_string(),
     pages: 382,
};
let title = struct_book.title;
```

They are the most common data type in Rust and are used to represent more complex data.

### Arrays

- Arrays are lists of elements of the same type.
- They have a fixed size that is determined at compile time.
- Just as in tuples, the elements of an array are accessed through their index, which starts at zero.
- The syntax includes declaration and access using `[]`.

```rust
let array_of_books = ["Algorithms + Data Structures = Programs", "The Pascal User Manual and Report", "The Art of Computer Programming"];
let first_book = array_of_books[0];
```

They are useful when you need to store a fixed amount of data. The content can be modified, but not its size, which results in significant memory optimization.

### Vectors (Vec)

- Vectors are similar to arrays, but their size can change during program execution.
- Syntax includes declaration and access via `Vec::new()`, `push()`, `pop()` and `len()`.

```rust
let mut vector_of_books = Vec::new();
vector_of_books.push("Algorithms + Data Structures = Programs");
vector_of_books.push("The Pascal User Manual and Report");
vector_of_books.push("The Art of Computer Programming");
```

Vectors are useful when you need to store a variable amount of data. Especially when you depend on external data sources, such as files, databases, the network... or users, that is, almost always.

### Enums

- Enums are a data type that offers values from a finite range or domain.
- Each value is considered a variant of the enumeration.
- Variants do not have to be of the same type and can mix simple types with structures.

```rust
enum State {
     WishList,
     Purchased,
     Reading,
     Read,
}

struct Book {
     title:String,
     author:String,
     pages: u32,
     state: State,
}

let struct_book = Book {
     title: "Algorithms + Data Structures = Programs".to_string(),
     author: "Niklaus Wirth".to_string(),
     pages: 382,
     state: State::Read,
};
```

They are useful when you need to represent a finite set of options in your program. The standard library defines two enumerations widely used in Rus, `Result` and `Option`. Combined with the 'match' control structure, they are one of the most powerful aspects of Rust that we will see in this and future articles.

## Flow control structures

We come to the algorithms section. Control flow structures allow us to make decisions and repeat tasks in our programs. In Rust, as in any language, we have the two basics, conditionals and repetitive, fundamental to writing programs that do something useful.

### Conditionals

#### if

- The `if` structure allows us to execute a block of code if a condition is met.

```rust
letpages = 382;
if pages >= 300 {
println!("It is a great book");
} else {
println!("It is a short book");
}

```

#### match

This structure is characteristic of Rust, it is similar to `switch` in other programming languages. With the particularity that it is typically executed against an `enum`, necessarily going through each variant.

```rust
enum State {
     WishList,
     Purchased,
     Reading,
     Read,
}

let state = State::Read;
match state {
     State::WishList => println!("Book is in the wish list"),
     State::Purchased => println!("Book is purchased"),
     State::Reading => println!("Book is being read"),
     State::Read => println!("Book has been read"),
}
```

Beyond simple comparison of values, this control flow structure is widely used to handle errors, using the `Result` type, which is an enumeration with two variants, `Ok` and `Err`, and with the `Option` enumeration, which has two variants, `Some` and `None` to deal with optional values.

These cases will be seen in future articles, but I can tell you that it is one of the most powerful features of Rust.

### Repetitive

In this case, they will be familiar to you since they are very similar to those of any other language. Let's have a quick review.

#### for

The `for` structure allows us to iterate over a sequence of elements, usually an _array_ or a _vector_.

```rust
let library = ["Algorithms + Data Structures = Programs", "The Pascal User Manual and Report", "The Art of Computer Programming"];

for book in library.iter() {
println!("{}", book);
}

```

In addition to looping through the elements sequentially, we can also iterate over an array using an index. Note the syntax `0..library.len()`, which is a range of numbers from 0 to the size of the array.

```rust
let library = ["Algorithms + Data Structures = Programs", "The Pascal User Manual and Report", "The Art of Computer Programming"];
for i in 0..library.len() {
     println!("{}", library[i]);
}
```

#### while

The `while` structure allows us to execute a block of code as long as a condition is met. In a way, they are repetitive conditionals.

```rust
// my library
let mut library = Vec::new();
// fill library
library.push("Algorithms + Data Structures = Programs");
library.push("The Pascal User Manual and Report");
library.push("The Art of Computer Programming");
// read library
let mut i = 0;
while i < library.len() {
  println!("{}", library[i]);
  i += 1;
}
```

#### loop

The `loop` structure allows us to execute a block of code indefinitely. The way to break out of the loop is with the `break` statement.

```rust
let mut i = 0;
letpages = 382;
loop {
     println!("Reading page: {}", i);
     i += 1;
     if i == pages {
         println!("Finished reading");
         break;
     }
}
```

### Programs

Obviously, we cannot finish this article without a complete program. Also, as a reminder of the time when we learned to program, I present to you the famous and simple program **ATM**, for its acronym in English, "Automated Teller Machine", that is, an automatic teller machine.

![Rust programs](2-rust-programs.jpg)

In it, I have used almost all the data and flow control structures we have seen. I have also defined a `struct`, suitable for representing a _wad_ (wad) of bills of a certain face value. And the wallet, which is a `Vec` of _wads_.

I hope you like it. You have the code in my [Rust lab on GitHub](https://github.com/AlbertoBasalo/rs-lab/blob/main/2-programs/src/main.rs).

```rust
use std::env;
// ATM machine
fn main() {
    // Get the command line arguments as a vector of strings
    let args: Vec<String> = env::args().collect();

    // Early return if no arguments are provided
    if args.len() < 2 {
        println!("🚧 Please provide the amount to withdraw.");
        return;
    }

    // Get the amount to withdraw by index position from the vector of arguments
    let typed_amount: &String = &args[1];

    // Returns an enum to represent the result of parsing a string to a number, either Ok or Err
    let parse_result: Result<u16, std::num::ParseIntError> = typed_amount.parse();

    // Match the parse result enum to either Ok or Err
    let amount_to_withdraw: u16 = match parse_result {
        Ok(n) => n, // Return the number if it's valid
        Err(_) => {
            println!("🚧 Please provide a valid amount number to withdraw.");
            return;
        }
    };

    // Early return if amount_to_withdraw is zero or is greater than MAX_AMOUNT_TO_WITHDRAW
    if amount_to_withdraw == 0 {
        println!("🕳️ Nothing to withdraw.");
        return;
    }
    const MAX_AMOUNT_TO_WITHDRAW: u16 = 1000;
    if amount_to_withdraw > MAX_AMOUNT_TO_WITHDRAW as u16 {
        println!("🚧 Amount to withdraw is greater than maximum allowed.");
        return;
    }

    // Array of available notes values
    const NUM_DISTINCT_NOTE_VALUES: usize = 6;
    let available_note_values: [u8; NUM_DISTINCT_NOTE_VALUES] = [200, 100, 50, 20, 10, 5];

    // Early return if amount_to_withdraw is not multiple of the minimum note value
    let min_note_value: u8 = available_note_values[NUM_DISTINCT_NOTE_VALUES - 1];
    if amount_to_withdraw % (min_note_value as u16) != 0 {
        println!("🚧 Amount to withdraw is not multiple of the minimum note value.");
        return;
    }

    // Struct to store a wad of notes of a given value
    struct WadOfNotes {
        value: u8,
        quantity: u8,
    }
    // Vector of wads of notes to keep in your wallet
    let mut wallet: Vec<WadOfNotes> = Vec::new();

    let mut pending_amount: u16 = amount_to_withdraw;

    // Iterate over available_note_values
    for &note_value in available_note_values.iter() {
        // Calculate the number of notes to withdraw of the current note value
        let quantity: u8 = (pending_amount / note_value as u16) as u8;
        // early return if nothing to withdraw
        if quantity == 0 {
            continue;
        }
        // Create a wad of notes of the current note value and quantity
        let wad: WadOfNotes = WadOfNotes {
            value: note_value,
            quantity,
        };
        // Push the wad to the wallet
        wallet.push(wad);
        // Update pending_amount
        let wad_value: u16 = (quantity as u16) * (note_value as u16);
        pending_amount -= wad_value;
    }
    if pending_amount != 0 {
        println!("🔥 Error: pending_amount is not zero.");
        return;
    }

    // traverse wallet and print each wad with note details
    let mut index: usize = 0;
    println!("💼 Save {} in to your wallet", amount_to_withdraw);
    while index < wallet.len() {
        let wad: &WadOfNotes = &wallet[index];
        println!("💸 A wad of {} notes of {}.", wad.quantity, wad.value);
        index += 1;
    }
}
```

With this second article in the series, we have already seen the fundamentals of programming with Rust. If you have any questions or suggestions, don't hesitate to leave a comment.

We will continue with the world of functions and something very typical of Rust: the concept of property and the lifetime of its variables. Until next time!

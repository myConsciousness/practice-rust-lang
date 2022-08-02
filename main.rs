// This is the entry point of the program.
//
// See more https://doc.rust-lang.org/rust-by-example/index.html
fn main() {
    println!("Hello, World!");
    // Possible to bind value with "{}".
    println!("Hello, {}!", "world");

    // Possible to call function like this.
    saySomething();

    // Output n from 1 to 100.
    for n in 1..101 {
        println!("{}", n);
    }

    // The same output with above.
    for n in 1..=100 {
        println!("{}", n);
    }

    advancedForEach();

    tryIfElse();

    tryWhileLoop();
}

// Possible to define function like this.
fn saySomething() {
    println!("Hey what's up?");
}

fn advancedForEach() {
    let mut names = vec!["Bob", "Frank", "Ferris"];
    println!("names: {:?}", names);

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}

fn tryIfElse() {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");

        10 * n
    } else {
        println!(", and is a big number, halve the number");

        n / 2
    };

    println!("{} -> {}", n, big_n);
}

fn tryWhileLoop() {
    // Make `optional` of type `Option<i32>`
    let mut optional = Some(0);

    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
    }
}

// Silence some warnings that could distract from the exercise
#![allow(unused)]

fn inspect(value: Option<&str>) {
    if let Some(val) = value {
        println!("You passed in {}", val);
    }
}

fn main() {
    // 1. If `maybe_fruit` below is a `Some` variant, then print out the string it wraps. Use the
    // Option type's `is_some` and `unwrap` methods with an `if` expression to implement the logic.
    // Then run your code. You should get the output "apple".

    let maybe_fruit: Option<&str> = Some("apple");
    if maybe_fruit.is_some() {
        println!("{}", maybe_fruit.unwrap());
    }

    // 2. Write a function `inspect` that accepts an `Option<&str>` as an argument and does not
    // return anything. Use an `if let` expression inside the function to get the value wrapped by
    // the `Some` variant and print out "You passed in a {}" with the value of the string. The
    // function should do nothing if the value is a `None` variant.
    //
    // Call the function once for `maybe_plant` and once for `maybe_food`.
    //
    // Then run the code. You should get one line of output about cake.

    let maybe_plant: Option<&str> = None;
    let maybe_food: Option<&str> = Some("cake");
    inspect(maybe_plant);
    inspect(maybe_food);

    // 3.  Write a loop that passes each number in the `numbers` vector to the `do_math` function
    // and then checks the result using a `match` expression.
    //
    // The `do_math` function (see bottom of this file) takes an i32 as input and returns a
    // `Result` whose `Ok` variant wraps a new i32, or an `Err` variant that wraps a `String`
    // which is a message about the error which occurred.
    //
    // - If the result is an `Ok`, then print out "The result was {}", with the value of the
    //   wrapped number
    // - If the result is an `Err`, then print out the wrapped error message string
    //
    // You should get one error message with a sad face, and one line with the number 100.

    let numbers = vec![0, 1];
    for num in numbers {
        let res = do_math(num);
        match res {
            Ok(val) => println!("The result was {}", val),
            Err(err) => println!("The error is {}", err),
        }
    }

    // 4. Define an enum named `Snack` with the following variants:
    //
    // - Apple - which contains no data
    // - Cookies - which contains an unnamed tuple with a single `u8`
    // - Sandwich - which contains an unnamed struct with fields `lettuce` and `cheese`. Both fields
    //   are the type `bool`.
    //
    // Then uncomment and run the code below. If you defined the enum correctly, you should get
    // output about three snacks.

    enum Snack {
        Apple,
        Cookies(u8),
        Sandwich { lettuce: bool, cheese: bool },
    }
    let healthy_snack = Snack::Apple;
    let sugary_snack = Snack::Cookies(18);
    let lunch = Snack::Sandwich {
        lettuce: false,
        cheese: true,
    };

    let snackies = vec![&healthy_snack, &sugary_snack, &lunch];

    for snack in snackies {
        match snack {
            Snack::Apple => println!("The healthy snack is an apple."),
            Snack::Cookies(num_cookies) => println!("The sugary snack is {} cookies", num_cookies),
            Snack::Sandwich { lettuce, cheese } => {
                let lettuce_msg = if *lettuce { "does" } else { "does not" };
                let cheese_msg = if *cheese { "does" } else { "does not" };
                println!(
                    "The sandwich {} have lettuce and {} have cheese.",
                    lettuce_msg, cheese_msg
                );
            }
        }
    }

    // 5. Create an `impl` block for the `Snack` enum and implement a method named `price` which
    // takes ownership of a Snack and returns a u8 representing the price of the snack according to
    // the following rules:
    //
    // - The price of an apple is always 5
    // - The price of cookies is 2 times the number of cookies
    // - A sandwich's price starts at 10, plus 1 if lettuce is true, plus 2 if cheese is true
    //
    // Hint: The signature of the method is `fn price(self) -> u8`
    //
    // Then uncomment and run the code below. You should see three lines ending with the costs of
    // $5, $36, and $12.

    impl Snack {
        fn price(self) -> u8 {
            match self {
                Self::Apple => 5,
                Self::Cookies(num_cookies) => 2 * num_cookies,
                Self::Sandwich { lettuce, cheese } => {
                    let mut price = 10;
                    if lettuce {
                        price += 1;
                    }
                    if cheese {
                        price += 2;
                    }

                    price
                }
            }
        }

        fn is_apple(&self) -> bool {
            match self {
                Self::Apple => true,
                _ => false,
            }
        }
    }

    println!("An apple costs ${}", healthy_snack.price());
    if let Snack::Cookies(number) = sugary_snack {
        println!("{} cookies costs ${}", number, sugary_snack.price());
    }
    if let Snack::Sandwich { lettuce, cheese } = lunch {
        let lettuce_message = if lettuce { " with lettuce" } else { "" };
        let cheese_message = if cheese { " with cheese" } else { "" };
        println!(
            "A sandwich{}{} costs ${}",
            lettuce_message,
            cheese_message,
            lunch.price()
        );
    }

    // Challenge 1: Implement an `is_apple` method for Snack that return a bool. Return `true` if
    // the value is an `Apple` variant, and `false` otherwise. Then uncomment and run the code
    // below.

    let snacks = vec![Snack::Apple, Snack::Cookies(5), Snack::Apple];
    for (index, snack) in snacks.iter().enumerate() {
        if snack.is_apple() {
            println!("Snack {} is an apple.", index)
        } else {
            println!("Snack {} is NOT an apple.", index)
        }
    }

    // Challenge 2: Refactor the code from (4) to put all off the variables into a vector, then loop
    // through the vector and use a `match` expression instead of `if let` statements. The output
    // should remain the same.
}

fn do_math(x: i32) -> Result<i32, String> {
    if x == 1 {
        return Ok(100);
    }
    return Err(format!("I wanted the number 1 and you gave me a {} 🥺", x));
}

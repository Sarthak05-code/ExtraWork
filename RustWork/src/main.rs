

// 1. Add #[derive(Debug)] so Rust knows how to print your errors using `{:?}`
#[derive(Debug)]
enum Error {
    TooManyNegativeNumberError,
    TooManyPositiveNumberError,
    TooManyOddNumberError,
    TooManyEvenNumberError,
    NaturalNumberSequenceError,
    EmptyArrayError,
}

// Changed `&Vec<i32>` to `&[i32]` (Idiomatic Rust)
fn error_checker(number: &[i32]) -> Result<(), Vec<Error>> {
    let mut positive = 0;
    let mut negative = 0;
    let mut even = 0;
    let mut odd = 0;
    let mut sequence = 0;
    let mut errors = Vec::new();

    if number.is_empty() {
        // 2. Wrap this in `vec![]` because your function returns a `Vec<Error>`
        return Err(vec![Error::EmptyArrayError]);
    }

    for num in number {
        if *num > 0 {
            positive += 1;
        } else if *num < 0 {
            negative += 1;
        }

        if *num % 2 == 0 {
            even += 1;
        } else {
            odd += 1;
        }
    }

    for i in 0..number.len() - 1 {
        if number[i] + 1 == number[i + 1] {
            sequence += 1;
        }
    }

    if positive > 3 {
        errors.push(Error::TooManyPositiveNumberError);
    }
    if negative > 3 {
        errors.push(Error::TooManyNegativeNumberError);
    }
    if even > 3 {
        errors.push(Error::TooManyEvenNumberError);
    }
    if odd > 3 {
        errors.push(Error::TooManyOddNumberError);
    }
    if sequence > 3 {
        errors.push(Error::NaturalNumberSequenceError);
    }

    // 3. Your logic here was perfectly fine! Cleaned it up slightly for readability:
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn main() {
    let array = vec![1, 2, 3, 4, 5];

    match error_checker(&array) {
        Ok(()) => println!("This array is fine."),
        Err(errors) => {
            // Updated 'error' to 'errors' since it's a vector
            println!("{:?}", errors);
        }
    }
}
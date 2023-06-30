fn main() {
    // for i in 1..=1000 {
    //     println!("{i:>4}: {}", to_words(i));
    // }

    println!("{:?}", (1..=1000).map(|i| to_words(i).replace(" ", "").len()).sum::<usize>());
}

fn to_words(n: u32) -> String {
    let val = match n {
        1..=99 => return tens(n),

        100 => "one hundred",
        200 => "two hundred",
        300 => "three hundred",
        400 => "four hundred",
        500 => "five hundred",
        600 => "six hundred",
        700 => "seven hundred",
        800 => "eight hundred",
        900 => "nine hundred",

        101..=199 => return format!("one hundred and {}", tens(n - 100)),
        201..=299 => return format!("two hundred and {}", tens(n - 200)),
        301..=399 => return format!("three hundred and {}", tens(n - 300)),
        401..=499 => return format!("four hundred and {}", tens(n - 400)),
        501..=599 => return format!("five hundred and {}", tens(n - 500)),
        601..=699 => return format!("six hundred and {}", tens(n - 600)),
        701..=799 => return format!("seven hundred and {}", tens(n - 700)),
        801..=899 => return format!("eight hundred and {}", tens(n - 800)),
        901..=999 => return format!("nine hundred and {}", tens(n - 900)),

        1000 => "one thousand",
        _    => "",
    };
    String::from(val)
}

fn tens(n: u32) -> String {
    let val = match n {
        1..=9 => return digits(n),

        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        20 => "twenty",
        30 => "thirty",
        40 => "forty",
        50 => "fifty",
        60 => "sixty",
        70 => "seventy",
        80 => "eighty",
        90 => "ninety",

        21..=29 => return format!("twenty {}", digits(n - 20)),
        31..=39 => return format!("thirty {}", digits(n - 30)),
        41..=49 => return format!("forty {}", digits(n - 40)),
        51..=59 => return format!("fifty {}", digits(n - 50)),
        61..=69 => return format!("sixty {}", digits(n - 60)),
        71..=79 => return format!("seventy {}", digits(n - 70)),
        81..=89 => return format!("eighty {}", digits(n - 80)),
        91..=99 => return format!("ninety {}", digits(n - 90)),

        _ => "",
    };
    String::from(val)
}

fn digits(n: u32) -> String {
    let val = match n {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        _ => "",
    };
    String::from(val)
}


const DAYS_OF_XMAS: [&str; 12] = [
    "partridge in a pear tree",
    "Two turtle doves,",
    "Three French hens,",
    "Four calling birds,",
    "Five golden rings,",
    "Six geese a-laying,",
    "Seven swans a-swimming,",
    "Eight maids a-milking,",
    "Nine ladies dancing,",
    "Ten lords a-leaping,",
    "Eleven pipers piping,",
    "Twelve drummers drumming,",
];

const WORD_FOR_NTH_DAY: [&str; 12] = [
    "first",
    "second",
    "third",
    "fourth",
    "fifth",
    "sixth",
    "seventh",
    "eighth",
    "ninth",
    "tenth",
    "eleventh",
    "twelfth",
];

const MONTHS_PER_YEAR: usize = 12;

fn main() {
    for i in 1..MONTHS_PER_YEAR + 1 {
        print_day_of_xmas(i);
    }
}

fn print_day_of_xmas(day_number: usize) {
    print_song_prefix(day_number);
    print_lines_for_day(day_number);
    print_blank_line();
}

fn print_lines_for_day(day_number: usize) {
    if day_number == 1 {
        print_first_day_only_line();
        return;
    }

    for current_day in (1..day_number + 1).rev() {
        print_single_day_line(current_day);
    }
}

fn print_blank_line() {
    println!("");
}

fn print_song_prefix(day_number: usize) {
    let day_word = WORD_FOR_NTH_DAY[day_number - 1];
    println!("On the {day_word} day of Christmas,");
    println!("my true love gave to me");
}


fn print_single_day_line(day_number: usize) {
    if day_number == 1 {
        println!("And a {}", DAYS_OF_XMAS[0]);
    } else {
        println!("{}", DAYS_OF_XMAS[day_number - 1]);
    }
}

fn print_first_day_only_line() {
    println!("A {}", DAYS_OF_XMAS[0]);
}
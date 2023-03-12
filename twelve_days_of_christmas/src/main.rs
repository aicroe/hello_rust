fn main() {
    const CHRISTMAS_DAYS: usize = 12;
    let presents: [&str; CHRISTMAS_DAYS] = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];
    let ordinal_numbers: [&str; CHRISTMAS_DAYS] = [
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

    for current_day in 0..CHRISTMAS_DAYS {
        println!("[Verse {}]", current_day + 1);
        println!(
            "On the {} day of Christmas, my true love sent to me",
            ordinal_numbers[current_day]
        );
        for day in (0..current_day + 1).rev() {
            println!("{}", presents[day]);
        }
        println!("");
    }
}

fn main() {
    
    /*
    A cute little Christmas carol :).
    */

    let sentences = ["A partridge in a pear tree",
    "Two turtle doves and",
    "Three french hens",
    "Four calling birds",
    "Five golden rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming"];

    for i in 1..=12 {
     
        // English grammar is funny, so we have special cases for the first three ordinal numbers.
        let suffix = match i{
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th"
        }; 
     
        println!("On the {i}{suffix} day of Christmas, my true love sent to me:");

        for j in {1..=i}.rev() {
            println!("{}", sentences[j-1]);
        }
        println!("");
    }
}

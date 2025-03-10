fn main() {
    let presents = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five golden rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];
    let mut index = 0;

    loop {
        if index < 12 {
            println!("On the {}. day of Christmas my true love gave to me:", index + 1);

            let mut i = index;
            loop {
                if i > 0 {
                    println!("{}, ", presents[i].to_string());
                    i = i - 1;
                } else if i == 0 && index != 0 {
                    println!("and {}", presents[i].to_string());
                    break
                } else {
                    println!("{}", presents[i].to_string());
                    break
                }
            }
            
            index = index + 1;
        } else {
            break
        }
    }
}

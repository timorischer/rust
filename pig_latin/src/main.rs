fn starts_with_vowel(mut x: String) -> bool {
    x = x.to_ascii_lowercase();
    if x.starts_with('a')
        || x.starts_with('ä')
        || x.starts_with('Ä')
        || x.starts_with('e')
        || x.starts_with('i')
        || x.starts_with('o')
        || x.starts_with('u')
    {
        true
    } else {
        false
    }
}

fn consonant(word: &str) -> String {
    let initial = word.chars().next().unwrap();
    let mut rest: String = String::from("");
    let mut position: u8 = 0;
    for c in word.chars() {
        position += 1;
        if position > 1 {
            rest.push(c)
        }
    }
    let mut output: String = String::from(rest);
    output.push('-');
    output.push(initial);
    output = output + "ay";
    output
}

fn vowel(word: &str) -> String {
    let mut output: String = String::from(word);
    output = output + "-fay";
    output
}

fn main() {
    let original_string: String =
        String::from("This program converts words into pig latin and works with UTF8 like Äpfel Körner");
    let mut final_word: String = String::from("");
    let mut i: i8 = 0;
    for word in original_string.split_whitespace() {
        i += 1;
        if starts_with_vowel(word.to_string().clone()) {
            if i > 1 {
                final_word.push(' ');
            }
            final_word += &String::from(vowel(word));
        } else {
            if i > 1 {
                final_word.push(' ');
            }
            final_word += &String::from(consonant(word));
        }
    }
    println!("{}", final_word);
}

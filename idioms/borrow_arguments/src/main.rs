// You should always prefer using the borrowed type over borrowing the owned type.
// Such as &str over &String, &[T] over &Vec<T>, or &T over &Box<T>.

fn three_vowels(word: &str) -> bool {
    let mut vowel_count = 0;
    for c in word.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vowel_count += 1;
                if vowel_count >= 3 {
                    return true;
                }
            }
            _ => vowel_count = 0
        }
    }
    false
}


fn main() {
    let ferris = "Ferris".to_string();
    let curious = "Curious".to_string();
    println!("{}: {}", ferris, three_vowels(&ferris));
    println!("{}: {}", curious, three_vowels(&curious));

    // This works fine, but the following two lines would fail:
    println!("Ferris: {}", three_vowels("Ferris"));
    println!("Curious: {}", three_vowels("Curious"));
}

//The str type, also called a ‘string slice’, is the most primitive string type.
//It is usually seen in its borrowed form, &str. It is also the type of string literals, &'static str.

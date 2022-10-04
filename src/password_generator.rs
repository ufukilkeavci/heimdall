use rand::Rng;

pub fn generate<'a>(
    length: Option<u8>,
    includes_numbers: Option<bool>,
    include_letters: Option<bool>,
    include_capitals: Option<bool>,
    include_symbols: Option<bool>,
    not_allowed_chars: Option<Box<Vec<char>>>,
) -> Option<Box<String>> {
    let _length = length.unwrap_or(16);
    let _includes_numbers = includes_numbers.unwrap_or(true);
    let _include_letters = include_letters.unwrap_or(true);
    let _include_capitals = include_capitals.unwrap_or(true);
    let _include_symbols = include_symbols.unwrap_or(false);
    let _not_allowed_chars = not_allowed_chars.unwrap_or(Box::new(vec!['\0']));

    let letters = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    let capitals = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    let numbers = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    let symbols = [
        '!', '#', '$', '%', '&', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '>', '<', '?',
        '@', '[', ']', '^', '_', '{', '}', '~',
    ];

    let mut allowed_chars: Vec<char> = vec![];

    if _include_capitals {
        for i in 0..capitals.len() {
            if !_not_allowed_chars.contains(&capitals[i]) {
                allowed_chars.push(capitals[i]);
            }
        }
    }

    if _includes_numbers {
        for i in 0..numbers.len() {
            if !_not_allowed_chars.contains(&numbers[i]) {
                allowed_chars.push(numbers[i]);
            }
        }
    }

    if _include_symbols {
        for i in 0..symbols.len() {
            if !_not_allowed_chars.contains(&symbols[i]) {
                allowed_chars.push(symbols[i]);
            }
        }
    }

    if _include_letters {
        for i in 0..letters.len() {
            if !_not_allowed_chars.contains(&letters[i]) {
                allowed_chars.push(letters[i]);
            }
        }
    }

    let char_space_size = allowed_chars.len();

    let mut password_array: Vec<char> = Vec::new();

    let mut rng = rand::thread_rng();
    for _ in 0.._length {
        password_array.push(allowed_chars[rng.gen_range(0..char_space_size) as usize]);
    }

    //println!("{:?}", password_array);

    let password: &str = &password_array.iter().collect::<String>();
    Some(Box::new(password.to_string()))
}

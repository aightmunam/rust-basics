#[allow(unused_variables)]
fn main() {
    // TWO DIFFERENT TYPES TO REPRESENT STRINGS - `&str`, `String`
    // ------

    // Type 1: &str is a reference to an immutable storage of string. This can be a reference to some
    // memory location in Heap, Stack, or directly in the binary.

    // String literals are stored in the binary, a reference to that memory location is saved as
    // &str in the variable.
    let s1: &str = "foo";

    // Fixed sized array of utf8 are stored in the stack. In this case, a reference to the stack
    // location is saved as the &str in the variable.
    let s1: &str= std::str::from_utf8(&[b'f', b'o', b'o']).expect("Error!");

    // String data is stored on the heap. Rust automatically coerces type &String to &str
    // where needed. Here, s3 stores a reference to the heap location where "foo" is stored.
    let s1: &str = &String::from("foo");

    // Type 2: String is a mutable collection that is stored on the heap.
    let string1: String = String::from("foo");


    // CONVERTING STRING TYPES
    // ------

    // Converting string slice to String
    let string2: String = "foo".to_string();
    let string2: String = s1.to_string();
    let string2: String = String::from("foo");
    let string2: String = String::from(s1);


    // Converting String to a slice
    // Rust automatically coerces the &String type to &str
    let s2: &str = &string2;
    let s2: &str = &string2[..];


    // SUBSTRINGS
    // ------

    // Get a substring of a String
    // &String[start..end] means get a slice starting at `start` upto but not including `end`
    let s3: &str = &string2[0..]; // Get a slice from 0 to last index
    let s3: &str = &string2[..2]; // Get a slice from start to 1 index (right index is excluded)
    let s3: &str = &string2[0..3]; // Get a slice from 0 to 2 index


    // COMBINING STRINGS
    // ------

    // Combining multiple string slices
    let string3 = ["foo", "bar"].concat(); // Results in a String not in a string slice i.e `&str`
    let string3 = format!("{}{}", "foo", "bar"); // Results in a String not a string slice

    // If we want to combine a String and a string slice, we can add them simply
    // However, this will only work if the String type is added first and then the slice(s)
    let string3 = string3 + s1 + s2 + s3 + &string2;

    // Combining multiple String types
    // The first value should be a String and the rest should be String references i.e &String
    let string3 = String::from("foo") + &String::from("bar") + &String::from("foobar");

    // Adding to a String
    let mut string4 = String::new();
    string4.push_str(s3);
    string4.push_str("bar");
    string4.push_str(&string3);

    // We can also add chars to a String
    string4.push('f');
    string4.push('o');
    string4.push('o');


    // GETTING CHARS FROM STRING
    // ------

    // We cannot get a char like &string[3] as a char could be more than a single letter.
    // Gets the third character. This is not equivalent to string4[3]
    let c = string4.chars().nth(3); // Returns an Option
    let c = match c {
        Some(c) => c,
        None => {}
    };
}

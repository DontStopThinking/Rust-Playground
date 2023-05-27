use std::{println, format, vec};

fn main() {
    let eye = 'ಠ';
    println!("{}_{}", eye, eye);

    let eye_unicode = '\u{CA0}';
    println!("{}_{}", eye_unicode, eye_unicode);

    assert_eq!('*' as i32, 42);
    assert_eq!('*'.is_alphabetic(), false);

    let eigen_text = "I see the eigenvalue in thine eye";
    let (head, tail) = eigen_text.split_at(21);
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");

    let single_value_tuple = ("lonely hearts",);
    println!("single_value_tuple: {:?}", single_value_tuple);

    let eggs = (12, "eggs");
    let eggs_box = Box::new(eggs); // Allocated a tuple in the heap.
    println!("eggs_box: {:?}", eggs_box);

    let lazy = [1, 2, 4, 7, 11, 16];
    println!("lazy: {:?}", lazy);

    let taxonomy = ["Animalia", "Arthoropoda", "Insecta"];
    assert_eq!(taxonomy.len(), 3);

    let sieve = [true; 5];  // Array of 5 elements, all true.
    for i in 0..sieve.len() {
        println!("sieve {}", sieve[i]);
    }

    let mut chaos = [3, 5, 1, 10, 15];
    chaos.sort();
    assert_eq!(chaos, [1, 3, 5, 10, 15]);

    let mut primes = vec![2, 3, 5, 7];  // Shorthand syntax to create vector. Similar to array.
    assert_eq!(primes.iter().product::<i32>(), 210);

    primes.push(11);
    primes.push(13);
    assert_eq!(primes.iter().product::<i32>(), 30030);

    let rows = 640;
    let cols = 480;
    let new_pixel_buffer = vec![0; rows * cols];    // Shorthand for vector of specific length,
                                                    // all set to same value. Similar to array.
    assert_eq!(new_pixel_buffer.len(), rows * cols);

    let mut pal = Vec::new();
    pal.push("step");
    pal.push("on");
    pal.push("no");
    pal.push("pets");
    assert_eq!(pal, vec!["step", "on", "no", "pets"]);

    let v: Vec<i32> = (0..5).collect(); // Build a vector from values produced by an iterator.
    assert_eq!(v, [0, 1, 2, 3, 4]);

    let mut palindrome = vec!["a man", "a plan", "a canal", "panama"];
    palindrome.reverse();   // reverse() is a slice method. Can be used on both arrays and vectors.
    assert_eq!(palindrome, vec!["panama", "a canal", "a plan", "a man"]);

    let mut v = vec!["Snow Puff", "Glass Gem"];
    assert_eq!(v.pop(), Some("Glass Gem"));
    assert_eq!(v.pop(), Some("Snow Puff"));
    assert_eq!(v.pop(), None);

    let languages = vec!["lisp", "scheme", "c", "c++", "fortran"];
    for l in languages {
        println!(
            "{}: {}",
            l,
            if l.len() % 2 == 0 {
                "functional"
            } else {
                "imperative"
            },
        );
    }

    let v: Vec<f64> = vec![0.0,  0.707,  1.0,  0.707];  // Vector shorthand syntax
    let a: [f64; 4] =     [0.0, -0.707, -1.0, -0.707];  // Slice. It's a 'fat pointer' - contains a
                                                        // pointer to 1st element and the number of elements.

    let sv: &[f64] = &v;
    let sa: &[f64] = &a;

    println!("slice of vector: {:?}", sv);
    println!("slice of array: {:?}", sa);

    print_slice(&a);    // Works on arrays.
    print_slice(&v);    // Works on vectors.
    print_slice(&v[0..2]);  // Print the first two elements of v.
    print_slice(&a[2..]);   // Print elements of a starting with a[2].
    print_slice(&sv[1..3]);   // Print v[1] and v[2].

    let speech = "\"Ouch!\" said the well.\n";
    print!("{}", speech);

    // A string may span multiple lines. Newline character and spaces are included.
    println!("In the room the women come and go,
        Singing of Mount Abora");

    // Newline and leading spaces dropped.
    println!("It was a bright, cold day in April, and \
        there were four of us-\
        more or less.");

    let default_win_install_path = r"C:\Program Files\Gorillas";    // Raw string
    println!("{}", default_win_install_path);

    // when we want to include double quotes in the string
    println!(r###"
        This raw string started with 'r###'.
        Therefore it does not end until we reach a quote mark('"')
        followed immediately by three pound signs ('###'):
        "###);

    let method = b"GET";    // Byte string. It's actually a slice &[u8; 3]
    assert_eq!(method, &[b'G', b'E', b'T']);

    let noodles = "noodles".to_string();    // noodles is a String
    let oodles = &noodles[1..]; // _oodles is a &str reference to the text belonging to noodles
    let poodles = "ಠ_ಠ";

    println!("{}", noodles);
    println!("{}", oodles);
    println!("{}", poodles);

    assert_eq!(poodles.len(), 7);   // len() is measure in bytes, not characters
    assert_eq!(poodles.chars().count(), 3);

    // String

    assert_eq!(format!("{}°{:02}′{02}″N", 24, 5, 23), "24°05′23″N".to_string());

    let bits = vec!["veni", "vidi", "vici"];
    assert_eq!(bits.concat(), "venividivici");
    assert_eq!(bits.join(", "), "veni, vidi, vici");

    assert!("ONE".to_lowercase() == "one"); // 2 strings are equal is they contain the same characters
                                            // in the same order.
    assert!("peanut".contains("nut"));

    for word in "veni, vidi, vici".split(", ") {
        assert!(word.starts_with("v"));
    }

    // Type aliases
    type Bytes = Vec<u8>;
    let post: Bytes = vec![b'P', b'O', b'S', b'T'];
    println!("{:?}", post);
}

// Takes in a slice reference so it'll work arrays as well as vectors
fn print_slice(n: &[f64]) {
    for elt in n {
        print!("{} ", elt);
    }
    println!();
}

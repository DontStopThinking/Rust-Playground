use std::rc::Rc;

fn print_padovan() {
    let mut padovan = vec![1, 1, 1]; // allocated here
    for i in 3..10 {
        let next = padovan[i - 3] + padovan[i - 2];
        padovan.push(next);
    }
    println!("P(1..10) = {:?}", padovan);
} // dropped here

struct Person {
    name: String,
    birth: i32,
}

#[derive(Clone, Copy)] // this explicitly defines the struct as Copy type
struct Label {
    number: u32,
}

fn print_label(l: Label) {
    println!("Stamp: {}", l.number);
}

fn main() {
    print_padovan();

    {
        let point = Box::new((0.625, 0.5)); // Tuple allocated on the heap
        println!("point = {:?}", point);
        let label = format!("{:?}", point); // label allocated here
        assert_eq!(label, "(0.625, 0.5)");
    } // both dropped here

    {
        let mut composers = Vec::new();
        composers.push(Person {
            name: "Palestrina".to_string(),
            birth: 1525,
        });
        composers.push(Person {
            name: "Dowland".to_string(),
            birth: 1563,
        });
        composers.push(Person {
            name: "Lully".to_string(),
            birth: 1632,
        });

        for composer in &composers {
            println!("{}, born {}", composer.name, composer.birth);
        }
    }

    {
        let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
        let t = s.clone();
        let u = s.clone();
        println!("s = {:?}", s);
        println!("t = {:?}", t);
        println!("u = {:?}", u);
    }

    {
        let mut s = "Govinda".to_string();
        let t = s; // "Govinda" moved to t
        s = "Siddhartha".to_string(); // now s has a new value
        assert_eq!(t, "Govinda");
        assert_eq!(s, "Siddhartha");
    }

    // Moving elements of vector
    {
        let mut v = Vec::new();
        for i in 101..106 {
            v.push(i.to_string());
        }

        // 1. Pop a value off the end of the vector:
        let fifth = v.pop().expect("vector empty!");
        assert_eq!(fifth, "105");

        // 2. Move a value out of a given index in the vector,
        // and move the last element into it's spot:
        let second = v.swap_remove(1);
        assert_eq!(second, "102");
        println!("{:?}", v);

        // 3. Swap in another value for the one we're taking out:
        let third = std::mem::replace(&mut v[2], "substitute".to_string());
        assert_eq!(third, "103");

        assert_eq!(v, vec!["101", "104", "substitute"]);
    }

    {
        let l = Label { number: 3 };
        print_label(l);
        println!("My label number is {}", l.number); // l will not be moved since we've explicitly
                                                     // defined Label as Copy type
    }
    
    {
        // Rc and Arc: Shared ownership
        let s: Rc<String> = Rc::new("shirataki".to_string());
        let t: Rc<String> = s.clone();
        let u: Rc<String> = s.clone();

        println!("s: {}", s);
        println!("t: {}", t);
        println!("u: {}", u);
    }
}

// fn main() {
//     for i in range(1i, 101) {
//         if i % 15 == 0 {
//             println!("FizzBuzz");
//         } else if i % 5 == 0 {
//             println!("Buzz");
//         } else if i % 3 == 0 {
//             println!("Fizz");
//         } else {
//             println!("{}", i);
//         }
//     }
// }




// use std::rc::Rc;

// fn test(n: int, d: int, s: &str, prev: String) -> String { 
//     if n % d == 0{
//         s.to_string() + prev
//     } else {
//         prev
//     }
// }

/// Implements the FizzBuzz algorithm.
/// If n is:
///     * divisible by 3, returns "fizz"
///     * divisible by 5, returns "buzz"
///     * divisible by 3 and 5, returns "fizzbuzz"
///     * anything else, returns n as a String
fn fizzbuzz(n: int) -> String {

    fn test(n: int, d: int, s: &str, prev: String) -> String { 
        if n % d == 0{
            s.to_string() + prev
        } else {
            prev
        }
    }

    // fn test(d: int, s: &str, prev: String) -> String { 
    //     if n % d == 0{
    //         s.to_string() + prev
    //     } else {
    //         prev
    //     }
    // }

    // let test = |d, s: &str, prev: String| -> String
    //                      if n % d == 0 {
    //                          s.to_string() + prev
    //                      } else {
    //                          prev
    //                      };

    // let x = test(3, "fizz", test(5, "buzz", "".to_string()) );


    // let testRc = Rc::new( |d, s: &str, prev: String| -> String
    //                      if n % d == 0 {
    //                          s.to_string() + prev
    //                      } else {
    //                          prev
    //                      }
    //                      );
    // let test3 = testRc.clone();
    // let test5 = testRc.clone();
    // let x = (*test3)(3, "fizz", (*test5)(5, "buzz", "".to_string()) );

    // let x2 = test(5, "buzz", "".to_string());
    // let x = test(3, "fizz", x2);

    // let x = test(3, "fizz", test(5, "buzz", "".to_string()) );

    let x = test(n, 3, "fizz", test(n, 5, "buzz", "".to_string()));

    if x == "".to_string() {
        n.to_string()
    } else {
        x
    }

}

fn main() {
    // assert!(fizzbuzz(3).as_slice() == "fizz");
    // assert!(fizzbuzz(5).as_slice() == "buzz");
    // assert!(fizzbuzz(15).as_slice() == "fizzbuzz");
    // assert!(fizzbuzz(7).as_slice() == "7");
    // assert!(fizzbuzz(8).as_slice() == "lulz");

    // for i in range(1, 16) {
        // println!("{}", fizzbuzz(i));
    // }

    for f in range(1, 10001).map(fizzbuzz) {
        println!("{}", f);
    }

}

//rustc --pretty expanded test_fizzbuzz.rs
//rustc test_fizzbuzz.rs && ./test_fizzbuzz

#![feature(macro_rules)]
macro_rules! fizzbuzz(
    ($n:ident $(($d:expr, $str:expr)).+ ) => (
        {
            let n: int = $n;
            let test = |d: int, s: &str, prev: String| -> String {
                if n % d == 0 {
                    s.to_string() + if prev == "".to_string() { prev } else { "_".to_string() + prev }
                } else {
                    prev
                }
            };
            let x = "".to_string();
            $(
                let x = test($d, $str, x);
            )+
            if x == "".to_string() {
                n.to_string()
            } else {
                x
            }
        }
    );
)

fn fizzbuzz(n: int) -> String {
    fizzbuzz!(n (11,"pappa").(8,"lulz").(5,"buzz").(3,"fizz"))
}

fn main() {
    assert!(fizzbuzz(3).as_slice() == "fizz");
    assert!(fizzbuzz(5).as_slice() == "buzz");
    assert!(fizzbuzz(15).as_slice() == "fizz_buzz");
    assert!(fizzbuzz(7).as_slice() == "7");

    // for i in range(1, 16) {
    //     println!("{}", fizzbuzz(i));
    // }

    // for i in range(1, 101) {
    //     println!("{}", fizzbuzz(i));
    // }

    // for f in range(1, 10001).map(fizzbuzz) {
        // println!("{}", f);
    // }

}

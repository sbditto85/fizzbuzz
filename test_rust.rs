fn main() {
    let nums = [1, 2];
    let noms = ["Tim", "Eston", "Aaron", "Ben"];
 
    let mut odds = nums.iter().map(|&x| x * 2 - 1);

    for num in odds {
        println!("{:s}", num.to_string());
    }

    for num in nums.iter() {
        println!("{}", num.to_string());
    }

    for num in odds {
        spawn(proc() {
            println!("{:s} says hello from a lightweight thread!", noms[num]);
        });
    }
}

use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn  main() {
    
    let rndm = rand::thread_rng() .gen_range(1..=100);

    println! ("can you guess the number between 1 and 10 ?\n go ahead :");
    
    loop {
        let mut nbr = String::new();
        io::stdin()
        .read_line(&mut nbr).expect("failed !");
    
        let nbr: u32 = match nbr.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        println! ("you really chose : {nbr}");
    
        match nbr.cmp(&rndm) {
            Ordering::Less => println! ("a bit low innit !"),
            Ordering::Greater => println! ("a bit high innit !"),
            Ordering::Equal => { println! ("that's it mate"); break;}
        }
    }

}


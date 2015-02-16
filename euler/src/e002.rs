fn main(){

    println!("HELLO");

    let sum = fibs_upto( 4_000_000 )
        .iter()
        .filter( | &&x | x%2 == 0)
        .fold(0, | a,&x | a + x);

    println!("{}", sum)
    
}

fn fibs_upto(n: i32) -> Vec<i32>{

    let mut fibs = vec![0, 1, 1];

    loop {
        let y = fibs[fibs.len()-1] + fibs[fibs.len()-2];
        if y>n { break; }
        fibs.push(y);
    }
    
    return fibs

}

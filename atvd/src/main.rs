use std::io;


fn main() {
    println!("Insira o N em fibonacci: ");
    let mut N = String::new();

    io::stdin().read_line(&mut N).expect(" ");
            
    let N: i32 = N.trim().parse().expect(" ");
    println!("Fibonacci({}) = {}", N, fibonigga(N));
}


fn fibonigga(f: i32) -> i32 {
    if f <= 1 {
        return f;
    }
    fibonigga(f - 1) + fibonigga(f - 2)

}

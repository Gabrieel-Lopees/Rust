fn main() {
    let s1 =String::from("texto");

    let tamanho = calcula_tamanho(&s1);

    println!("O tamanho de '{}' eh {}", s1, tamanho);
}

fn calcula_tamanho(s: &String) -> usize {
    s.len()
}

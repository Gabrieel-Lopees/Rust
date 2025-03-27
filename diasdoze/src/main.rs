fn main() {
    let dias = [
        "primeiro", "segundo", "terceiro", "quarto", "quinto", "sexto",
        "sétimo", "oitavo", "nono", "décimo", "décimo primeiro", "décimo segundo",
    ];

    let presentes = [
        "Um pombo no parreiral",
        "Dois pombos da paz",
        "Três galos cantando",
        "Quatro pássaros chamando",
        "Cinco anéis dourados",
        "Seis gansos botando",
        "Sete cisnes nadando",
        "Oito criadas ordenhando",
        "Nove damas dançando",
        "Dez senhores saltando",
        "Onze flautistas tocando",
        "Doze tambores rufando",
    ];

    for i in 0..12 {
        println!("No {} dia de Natal, meu amor me deu", dias[i]);

        for j in (0..=i).rev() {
            if i > 0 && j == 0 {
                println!("E {}", presentes[j]);
            } else {
                println!("{}", presentes[j]);
            }
        }

        println!(); // Linha em branco entre os versos
    }
}


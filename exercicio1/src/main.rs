use std::collections::VecDeque;

fn main() {
    let mut fila: VecDeque<i32> = VecDeque::new();

    println! ("Adicionando");
    fila.push_back(1);
    println! ("Adicionando o número 1");
    fila.push_back(2);
    println! ("Adicionando o número 2");
    fila.push_back(3);
    println! ("Adicionando o número 3");

    println! ("\nTamanho da fila atual: {}", fila.len());

    println!("\nRemovendo elementos da fila:");
    while let Some(elemento) = fila.pop_front() {
        println!("Removido: {}", elemento);
    }

    println!("\nA fila está vazia? {}", fila.is_empty());
}

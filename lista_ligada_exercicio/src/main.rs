use std::collections::LinkedList;

fn main() {
    // Crie uma lista ligada vazia
    let mut frases = LinkedList::new();
    
    // Adicione 10 frases motivacionais à lista
    frases.push_back(String::from("A persistência é o caminho do êxito."));
    frases.push_back(String::from("O sucesso é a soma de pequenos esforços repetidos dia após dia."));
    frases.push_back(String::from("Acredite em você mesmo e todo o resto virá naturalmente."));
    frases.push_back(String::from("O único lugar onde o sucesso vem antes do trabalho é no dicionário."));
    frases.push_back(String::from("Não espere por oportunidades, crie você mesmo as suas."));
    frases.push_back(String::from("O pessimista vê dificuldade em cada oportunidade. O otimista vê oportunidade em cada dificuldade."));
    frases.push_back(String::from("O sucesso normalmente vem para quem está ocupado demais para procurar por ele."));
    frases.push_back(String::from("Se você quer algo que nunca teve, precisa fazer algo que nunca fez."));
    frases.push_back(String::from("Não é sobre ter tempo, é sobre fazer tempo."));
    frases.push_back(String::from("A jornada de mil quilômetros começa com um único passo."));
    
    // Acesse e imprima o terceiro elemento
    let terceiro_elemento = frases.iter().nth(2).unwrap();
    println!("O terceiro elemento da lista é: {}", terceiro_elemento);
    
    // Imprima o tamanho total da lista
    println!("A lista contém {} elementos.", frases.len());
}
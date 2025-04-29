use std::collections::HashMap;

fn main() {
    // criação do hashmap
    let mut estoque = HashMap::new();

    // 1. Inserção de valores
    estoque.insert("banana", 100);
    estoque.insert("pepino", 50);
    estoque.insert("maçã", 2);
    estoque.insert("caqui", 20);

    //2. Acessar de forma segura os valores do hashmap
    if let Some (qtde) = estoque.get("maçã") {
        println!("Temos {:?} maçãs em estoque!", qtde);
    }

    //3. Atualização estoque com entry()
    estoque.entry("pepino").and_modify(|qtde| *qtde += 100);
    if let Some (qtde) = estoque.get("pepino") {
        println!("Temos {:?} pepinos em estoque!", qtde);
    }

    // 4. Remover item "caqui"
    estoque.remove("caqui");

    // 5. Filtrar todas as frutas acima de 100 unidades
    estoque.retain(|fruta, &mut qtde| qtde > 100);
    println!("{:?}", estoque);

    // 6. Limpeza total
    estoque.clear();
    println!("{:?}", estoque);
}
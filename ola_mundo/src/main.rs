//modulo com as operacoes basicas
mod operacoes;

fn main() {
    let n2 = 20;
    let n1 = 50;

    println!("{} + {} = {}", n1, n2, operacoes::soma(n1, n2));
    println!("{} - {} = {}", n1, n2, operacoes::subtrai(n1, n2));
    println!("{} * {} = {}", n1, n2, operacoes::multiplica(n1, n2));

    let maior = operacoes::verifica_maior(n1, n2);
    println!("O maior entre {} e {} = {}", n1, n2, maior);
}

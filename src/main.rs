// Disciplina : Linguagem e Lógica de Programação
// Professor : Alan Furukawa
// Descrição : Aqui você descreve o que o programa faz! (função)
// Autor(a) : Gabriel Aguiar Rocha
// Data atual : 04/10/2021
use std::io;
fn ler()->i32{
let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().parse().unwrap()
}

 fn main() {
     let mut i = 0;
     println!("digite um lado {}º",i+1);
     let lado1 = ler();
     println!("digite um lado {}º",i+2);
     let lado2 = ler();
     println!("digite um lado {}º",i+3);
     let lado3 = ler();

     verficatamanho(lado1,lado2,lado3);
 }
fn verificaTriangulo(lado1:i32, lado2:i32, lado3:i32){
    if lado1 == lado2 && lado1 == lado3 || lado2 == lado1 && lado2 == lado3 || lado3 == lado1 && lado3 == lado2{
        print!("È um triangulo elquilatero todos os lados sao iguais")
    }

    if lado1 == lado2 && lado1 != lado3 || lado1 == lado3 && lado2 != lado3 || lado2 == lado3 && lado1 != lado1{
        println!("Isósceles: dois lados são iguais.")
    }
    if lado1 != lado2 && lado1 != lado3{
        println!("Escaleno: todos os lados são diferentes.")
    }
}
fn verficatamanho(lado1:i32, lado2:i32, lado3:i32){
    if (lado1+lado2) > lado3{
        println!("lado {} + lado {} é maior que {}",lado1,lado2,lado3);
        verificaTriangulo(lado1,lado2,lado3);
    }else if (lado1+lado3) > lado2 {
        println!("lado {} + lado {} é maior que {}",lado1,lado3,lado2);
        verificaTriangulo(lado1,lado2,lado3);
    }else if (lado3+lado2) > lado1 {
        println!("lado {} + lado {} é maior que {}",lado3,lado2,lado1);
        verificaTriangulo(lado1,lado2,lado3);
    }
}
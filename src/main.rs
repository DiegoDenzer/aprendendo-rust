mod soma_inteiros_consecutivos;

use std::io;
use num::complex::ComplexFloat;
use num::pow;

fn matrizes() {
    let mut matriz: [i8; 10] = [42; 10];
    matriz[5] = 0;
    println!("matriz: {:?}", matriz);
}

fn tuplas() {
    let tupla: (i8, bool) = (7, true);
    println!("1º índice: {}", tupla.0);
    println!("2º índice: {}", tupla.1);
}

fn referencia() {
    let mut valor_x: i32 = 10;
    let referencia_x: &mut i32 = &mut valor_x;
    *referencia_x = 20;
    println!("valor_x: {valor_x}");
}

fn slice() {
    let principal: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("Principal: {principal:?}");
    let fatia: &[i32] = &principal[2..4];
    println!("fatia: {fatia:?}");
}

fn soma_simple() {
    let mut a = String::new();
    let mut b = String::new();
    io::stdin().read_line(&mut a).expect("Falha ao ler entrada");
    io::stdin().read_line(&mut b).expect("Falha ao ler entrada");
    println!("SOMA: {}", (a.trim().parse::<i32>().unwrap() + b.trim().parse::<i32>().unwrap()))
}

fn area_circulo() {
    // area = π . raio 2.
    // 2.00 - A=12.5664
    // 100.64 -  A=31819.3103
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Falha ao ler entrada");
    let pi =  3.14159; // std::f64::consts::PI;
    let inicio = pi * a.trim().parse::<f64>().unwrap().powf(2.00);
    println!("A = {:.1$}", inicio, 4);
}


fn main() {
    area_circulo();
}


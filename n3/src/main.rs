use serde_json::from_str;
use std::fs;

fn main() {
    let data = fs::read_to_string("dados.json").expect("Incapaz de ler o arquivo.");
    let json: serde_json::Value = from_str(&data).expect("JSON não está bem formatado.");
    let json = json.as_array().unwrap();
    let mut maior: f64 = 0.0;
    let mut menor: f64 = f64::MAX;
    let mut media: f64 = 0.0;
    for dia in json.iter() {
        let valor: f64 = dia["valor"].as_f64().unwrap();
        media += valor / 30.0;
        if valor < menor {
            menor = valor;
        }
        if valor > maior {
            maior = valor;
        }
    }
    println!("O menor faturamento foi de {}.", menor);
    println!("O maior faturamento foi de {}.", maior);
    let mut soma = 0;
    for dia in json.iter() {
        let valor: f64 = dia["valor"].as_f64().unwrap();
        if valor > media {
            soma += 1;
        }
    }
    println!(
        "Foram {} dias que superaram a media ({:.4}) de faturamento do mês.",
        soma, media
    );
}

fn main() {
    let sp = 67_836.43;
    let rj = 36_678.66;
    let mg = 29_229.88;
    let es = 27_165.48;
    let outros = 19_849.53;
    let total = sp+rj+mg+es+outros;
    println!("SP tem {:.2}% do total.", (sp/total)*100.0);
    println!("RJ tem {:.2}% do total.", (rj/total)*100.0);
    println!("MG tem {:.2}% do total.", (mg/total)*100.0);
    println!("ES tem {:.2}% do total.", (mg/total)*100.0);
    println!("Outros tem {:.2}% do total.", (outros/total)*100.0);
}

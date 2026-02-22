
fn bwt( text : &str) -> String{
  let text = text.to_string();
    let text = text.chars();
    let mut text: Vec<char> = text.collect();
    text.push('$'); // 终止符
    let n = text.len();

    let mut rotations = (0..n).map(|i| {
        let mut rot = String::new();
        for j in 0..n {
            rot.push(text[(i + j) % n]);
        }
        rot
            
    }).collect::<Vec<String>>();
    rotations.sort();

    // 取最后一列
    rotations
        .iter()
        .map(|r| r.chars().last().unwrap())
        .collect()

}

fn main(){
    let text = "Hello!";
    let text = bwt(text);
    print!("{}", text);
    let n = text.len();
    let mut table = vec![String::new(); n];
    for _ in 0..n{
        for (i,c) in text.chars().enumerate(){
            table[i] = format!("{} {}",c,table[i]);
        }
        // table.sort();
        println!("Table after iteration:");
        for row in &table{
            println!("{}", row);    
        }
        
    }
    // for row in table{
    //     println!("{}", row);
    // }


}
use columns::Columns;

fn main(){
    println!(
    "{}",
    Columns::from(vec![
        vec!["line1", "line2", "line3"],
        vec!["should", "be", "displayed", "side by side"],
    ])
    .base_tabsize_in(0) 

    );    


}

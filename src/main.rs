fn main() {
    let args = env::args().collect::<Vec<String>>();
    let command: &str;
    if args.len() < 2 {
        println!("You did not specify a source file.");
    } else {
    
    }

}
fn int_f(file: String) -> (i32,Vec<Vec<Vec<String>>>){
    let mut total: <Vec<Vec<Vec<String>>>;
    let sp = file.split("|");
    let pre = sp[0];
    let sec = sp[1];
    let sp2 = sec.split("-");
    for s in sp2 {
        for l in s.lines() {
        
        }
    }
    (sp.trim(), 
}

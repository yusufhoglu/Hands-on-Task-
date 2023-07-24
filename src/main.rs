
fn main() {
    let first_string = String::from("Şanlı ");
    let second_string: String = String::from("Göztepe!");
    let concatenated_string = concatenate_strings(&first_string, &second_string);

    println!("{}",concatenated_string)
}
fn concatenate_strings(s1:&str,s2:&str) -> String {
    let mut new_string:String = String::from(s1);
    new_string.push_str(s2);
    new_string
}
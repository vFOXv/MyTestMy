use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Write, Read};
use std::io::stdin;

fn main(){
  let path = "changeText.txt";
  /*let mut my_file = File::open(path).expect("Error opening my file!!!");  
  //my_file.write_all(b"HHHHH!!!").expect("Error writing in my file!!!");
  let mut file_date = String::new();
  my_file.read_to_string(&mut file_date).expect("My Error rear to string from file!!!");
  println!("{}", file_date);*/

  let mut my_file = OpenOptions::new()
    .write(true)
    .read(true)
    .create(true)
    .open(path)
    .expect("MyError create/open!");

    let mut file_date = String::new();
    my_file.read_to_string(&mut file_date).expect("My Error riding file!!!");
    println!("{}", file_date);

    /*let mut my_input = String::new();
    stdin().read_line(&mut my_input).expect("My Error input string!!!");
    my_file.write_all(my_input.as_bytes()).expect("My Error writing in file!!!");*/
    std::fs::remove_file(path).expect("Error delet file!")
}





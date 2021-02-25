


fn main() {
   println!("Hello World");
   let buf1 = [3; 5];

   println!("Buffer Length: {}", buf1.len());

   for x in &buf1 {
       println!("{}", x);
   }
   
}
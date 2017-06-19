pub fn run_me() {

   let v = vec![3,5,11,9];

   println!("Length: {}", v.len());

   for elem in v {
       println!("Value: {}", elem)
   }


   let mut stack = Vec::new();

   stack.push(1);
   stack.push(2);
   stack.push(3);

   while let Some(top) = stack.pop() {
   	 // Prints 3, 2, 1
	 println!("{}", top);
    }

}

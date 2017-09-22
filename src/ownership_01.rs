

fn func1(vec: &Vec<i32>) -> i32 {
	// the referenced value is not allowed to change --> this does not work
	// vec[0] = 4;
	let mut sum = 0;
	for el in vec {
		sum = sum + el
	}
	sum
}

fn func2(vec: &mut Vec<i32>) -> i32 {
	vec[0] = 4;
	0
}

pub fn run_me() {

   //Immutable Stuff

   let v = vec![1, 2, 3];
   // that's not possible as v is imutable per default
   // v[0] = 4;
   // we transfer the ownership to v2
   let v2 = v;

   // now v2 has the ownership, therefore v can no longer be used --> this does not work
   // println!("Vectorelement 0: {}", v[0])
   // we can use the new owner --> this works
   println!("v2 Vectorelement 0: {}", v2[0]);

   // create a reference to v2
   let v3 = &v2;
   // creating a reference to v does not work (v has already been transferred to v2)
   //let vx = &v;
   // we can use the reference like the normal variable
   println!("v3 Vectorelement 0: {}", v3[0]);
   // but everything is imutable -> this does not work
   //v3[0] = 4;

   // while we have imutable references, we can still use the object the reference points to...
   println!("v2 Vectorelement 0: {}", v2[0]);

   // we can have as many imutable references to v2 as we want
   let v4 = &v2;
   println!("v4 Vectorelement 0: {}", v4[0]);

   // we can pass the reference to a function which can use it read-only
   let sum = func1(&v2);
   println!("Sum of v is: {}", sum);
   // and can still ue the vector afterwards...
   println!("{:?}", v2);

   // Mutable Stuff
   // create a mutable variable
   let mut v3 = vec![1,2,3];
   // and pass a mutable reference to this variable to the function --> now the 
   // function is allowed to change the ref and therefore the value the ref points to
   let x = func2(&mut v3);
   println!("v3 Vectorelement 0: {}", v3[0]);

   // we create a mutable reference to v3
   let r1 = &mut v3;
   // which allows changing v3
   r1[0] = 25;
   println!("r1 Vectorelement 0: {}", r1[0]);
   // but since r1 is still in scope, we are no longer allowed to use v3 as there is only one mutable reference allowed
   // this does not work
   //println!("v3 Vectorelement 0: {}", v3[0]);

   let mut x = 5;
   println!("x = {}", x);
   x = x + 1;
   println!("x = {}", x);

   { // borrow y mutably
     let y = &mut x;
     println!("y = {}", y);
     // change it
     *y += 1;
     println!("y = {}", y);
     // this is not allowed as x is already (mutably) borrowed
     //println!("x = {}", x);
   }
   // here the mutable ref y is out of scope -> we are allowed to use x again
   println!("x = {}", x);
}


fn main() {
    // println!("Hello, world!");

       let mut list:[i32;6] = [8, 4, 3, 2, 5,7];

      

       for i in 0..list.len(){

         for j in 0..list.len(){

            let b :i32 = list[i];
            let n :i32 = list[j];

            if b<n{
                list[j] = b;
                list[i] = n;
            }

            println!("{}>>>{}", b ,n)
         }
       }


  for y in 0..list.len(){
    println!("{}", list[y])

  }
}

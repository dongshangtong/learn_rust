

#[derive(Debug)]
struct Dog {
    name: String,
    weight: f32,
    height: f32,
}

impl Dog {
    fn ge_name(&self)-> &str {
        &(self.name[..])
    }

    fn get_weight(&self) -> f32 {
        self.weight
    }

    fn get_height(&self) -> f32 {
        self.height
    }
}



fn main() {
    let dog = Dog {
        name: String::from("erha"),
        weight: 100.0,
        height: 80.5,
    };

    println!("dog = {:#?}",dog);

    println!("name ={}",dog.ge_name());

    vacFun();

  
   


}



fn vacFun () {


    let mut v2: Vec<i32> = Vec::new();
    v2.push(1);
    v2.push(2);
    v2.push(4);


    for  i  in &v2 {
        println!("i={}",i)
    }

    for j in &mut v2 {
        *j += 3;
        println!("i={}",j);
    }

}


fn  enumfun () {

      // Option s是标准定义的一个枚举，形式：
    //   enum Option<T> {
    //    Some(T),
    //      None,    
    // }


    let some_number = Some(5);
    let some_string = Some(String:: from("a string"));
    let absent_number: Option<i32> = None;

    let x : i32 = 8;
    let y : Option<i32> = Some(5);
    let mut temp = 0;

    match y {
        Some(i) => {temp = i }
        None => {println!("do nothing");}
    }

    let sum = x + temp;

    println!("sum = {}",sum);
}

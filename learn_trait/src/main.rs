
pub trait GetInformation {
    fn get_name(&self)->&String;
    fn get_age(&self)->u32;
}

trait SchoolName {
    fn get_school_name(&self) -> String {
        String::from("Hong Xing School")
    }
}

pub struct Student {
    pub name:String,
    pub age:u32,
}

impl SchoolName for Student {}

impl GetInformation for Student {
    fn get_name(&self)->&String{
        &self.name
    }
    fn get_age(&self)->u32 {
        self.age
    }
}



fn print_inforMation(item: impl GetInformation) {
    println!("name= {}",item.get_name());
}

fn get_inforMation<T:GetInformation>(item:T){
    println!("name= {}",item.get_name());
}


trait GetName {
    fn get_myname(&self) -> &String;
}

trait GetAge {
    fn get_myage(&self) -> u32;
}



// 写法一
// fn getinfo<T:GetAge+GetName> (item:T){
//     println!("name = {}",item.get_myname());
//     println!("age = {}",item.get_myage());
// }

// 写法二
fn getinfo<T>(item:T) where T:GetAge+GetName{
       println!("name = {}",item.get_myname());
       println!("age = {}",item.get_myage()); 
}

pub struct Dog {
   pub name:String,
   pub age:u32,
}

impl GetName for Dog {
    fn get_myname(&self) ->&String {
        &self.name
    }
}

impl GetAge for Dog {
    fn get_myage(&self) -> u32 {
        self.age
    }
}


fn main() {

    let ss = Student{name:"hehhe".to_string(),age:10};

   let cc= ss.get_school_name();
    println!("school={}",cc);
   // println!("name = {}",ss.get_name());
  // print_inforMation(ss);
   get_inforMation(ss);


   let dd = Dog{name:"erha".to_string(),age:12};

   getinfo(dd);

  
    println!("Hello, world!");
}

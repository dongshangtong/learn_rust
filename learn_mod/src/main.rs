


mod mod1 {
  pub  const MESSAGE:&str = "holle me Joy";

        const MSD:u32 = 89;
  pub mod mod2 {
    pub const MESSAGE2:&str = "hello word";

   pub fn pridn() {
        println!("{}", super::MSD);
    }
  }

  pub(crate) enum crateEnum {
    Item = 4,
    Item2 = 5,
  }



}

fn main() {
//   println!("{}", );
mod1::mod2::pridn()
   
}







// fn templeOrdelate(){

// }

fn main() {
  //  orderofabulle()

  
  println!("KKKK:{}",bubble_ord(0.9,1.4));

// let mut list:[i32;6] = [8, 4, 3, 2, 5,7];

//    bubble_sort(&list,bubble_ord);

}



fn bubble_ord<T: std::cmp::PartialOrd>(a:T, b:T) -> bool {

   let mut lls = false;
     
    if a > b {
        lls = true;
    }

    return lls;
}



// 定义模板参数 T，表示排序的数据类型
// 定义模板参数 F，表示排序时使用的比较函数
fn bubble_sort<T, F>(arr: &mut [T], cmp: F)
where
    T: Copy,
    F: Fn(&T, &T) -> bool,
{
    // 获取序列的长度
    let len = arr.len();
    // 循环进行冒泡排序
    for i in 0..len {
        for j in 0..len - i - 1 {
            // 如果当前元素比后面的元素大，则交换两个元素的位置
            if cmp(&arr[j], &arr[j + 1]) {
                arr.swap(j, j + 1);
            }
        }
    }
}






// fn orderofabulle() {


//        let mut list:[i32;6] = [8, 4, 3, 2, 5,7];
//        for i in 0..list.len(){
//          for j in 0..list.len(){

//             let b :i32 = list[i];
//             let n :i32 = list[j];

//             if b<n{
//                 list[j] = b;
//                 list[i] = n;
//             }

//             println!("{}>>>{}", b ,n)
//          }
//        }


//   for y in 0..list.len(){
//     println!("{}", list[y])
//   }
    
// }
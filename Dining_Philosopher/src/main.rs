use std::string::String;
use std::thread;
use std::sync::{Mutex,Arc}; // 引入Mutex和Arc


struct Ph1{  // 以结构体代表线程
    name:String,
    left: usize,
    right: usize,
}
struct Table {
    forks: Vec<Mutex<()>>,
}

impl Ph1{
    fn new(name: &str, left: usize, right:usize) -> Ph1{   // 定义结构体的关联函数
        Ph1{
            name:name.to_string(),
            left: left, 
            right: right,
        }
    }
    fn eat(&self,table: &Table) { // 定义结构体的方法
        // 先锁住左边的叉子,再右边
        let _left = table.forks[self.left].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} is eating", self.name);
        thread::sleep_ms(1000);
        println!("{} finished eating", self.name);
        thread::sleep_ms(1000);
    }
}

fn main() {
    let table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});
    let mut Phs=vec![
        Ph1 {name: String::from("Karl Marx"), left: 0, right: 1}, 
        Ph1::new("President Mao", 1, 2),
        Ph1::new("Hegel", 2, 3),
        Ph1::new("Lenin", 3, 4), 
        Ph1::new("Hongwen Wang", 4, 0),
        ]; // 创建一个线程的向量

    let handles: Vec<_> = Phs.into_iter().map(|p| {
        let table = table.clone();

        thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();
    for h in handles {
        h.join().unwrap();
    }
    // let mut i=0;
    // loop{
    //     if i==Phs.len(){
    //         break;
    //     }
    //     Phs[i].eat();
    //     i+=1;
    // }
}

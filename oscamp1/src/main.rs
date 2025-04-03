#[derive(Debug)]
enum PokerSuit {
    Clubs(u8),
    Spades(u32),
    Is(bool),
    Diamonds(String),
    Hearts(Drink),
}
enum easy {
    A=1,
    B=2,
    C=3,
}
#[derive(Debug)]
struct Drink{
    name: String,
    price: f32,
}
fn main() {
    let heart = PokerSuit::Hearts(Drink {
        name: String::from("Coke"),
        price: 3.5,
    });
    let diamond = PokerSuit::Diamonds(format!("Diamond"));
    let club = PokerSuit::Clubs(1);
 //   let club2 = PokerSuit::Clubs2;
    let spade = PokerSuit::Spades(2);

    let club2 = easy::A; 
    print_suit(&heart);
    print_suit(&diamond);
    print_suit(&club);
    println!("the value of club is {:?}", club2 as u8); //  value borrowed here after move
}

fn print_suit(card: &PokerSuit) {
    // 需要在定义 enum PokerSuit 的上面添加上 #[derive(Debug)]，否则会报 card 没有实现 Debug
    // 如果参数不是引用，那么当一个被绑定的变量从一个作用域到了一个作用域
    // 其所有权亦会转移到新的所有者，原本的所有者就不能再使用这个变量了
    println!("{:?}",card);
}

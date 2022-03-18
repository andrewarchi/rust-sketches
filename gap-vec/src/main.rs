mod gap_vec;
use gap_vec::GapVec;

fn main() {
    let mut g = GapVec::new();
    println!("{:?}", g);

    g.push("Hello,".to_string());
    assert_eq!(g.len(), 1);
    assert_eq!(unsafe { g.get(0) }, &"Hello,".to_string());
    println!("{:?}", g);

    g.push("world".to_string());
    assert_eq!(g.len(), 2);
    assert_eq!(unsafe { g.get(1) }, &"world".to_string());
    println!("{:?}", g);

    g.push("!".to_string());
    assert_eq!(unsafe { g.get(2) }, &"!".to_string());
    assert_eq!(g.len(), 3);
    println!("{:?}", g);

    unsafe { println!("{} {}{}", g.get(0), g.get(1), g.get(2)) };

    unsafe { g.drop(1) };
    assert_eq!(g.len(), 2);
    println!("{:?}", g);

    unsafe { g.drop(0) };
    assert_eq!(g.len(), 1);
    println!("{:?}", g);

    g.push("Grüezi".to_string());
    assert_eq!(g.len(), 2);
    assert_eq!(unsafe { g.get(0) }, &"Grüezi".to_string());
    println!("{:?}", g);

    g.push("miteinand".to_string());
    assert_eq!(g.len(), 3);
    assert_eq!(unsafe { g.get(1) }, &"miteinand".to_string());
    println!("{:?}", g);

    unsafe { println!("{} {}{}", g.get(0), g.get(1), g.get(2)) };
}

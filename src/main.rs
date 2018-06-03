fn main() {
    // test1();
    // test2();
    // test3();
    // test4();
    // test5();
    // test6();
    // test10();

    let mut rect = Rectangle {
        width: 10,
        length: 20,
    };
    rect.area();
    rect.can_hold(&rect);
    Rectangle::square(10).area(); // 关联方法(类方法)
}

fn test1 () {
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{}", s);
}

// 测试传值
fn test2() {
    let s1 = String::from("hell0");
    let s3 = s1.clone(); // 拷贝
    let s2 = s1;  // 指针传递，s1 被移除

    // println!("{}", s1);

    let s4 = 5;
    let s5 = s4;  // 合法，因为是值传递  
}

// ownership转移 
/*
 * 除了所有的integer类型是值拷贝外，还有
 * - bool
 * - float
 * - tuples(只有当所有的元素都是值拷贝的)
 */
fn test3() {
    let s = String::from("hello");

    test3_1(s); // s的值转移进了方法
    // println!(s); // 非法，不可以再使用

    let x = 5;  // x不是地址空间，而是编译成一个固定长度  
    test3_2(x); 
    println!("{}", x); // x还可以使用
}

fn test3_1(sth: String) {
    println!("{}", sth);
}

fn test3_2(si: i32) {
    println!("{}", si);
}

// 传指调用
fn test4() {
    let s1 = String::from("hello");
    let len = test4_1(&s1);

    println!("the length of {} is {}", s1, len);
}

fn test4_1(s: &String) -> usize {
    s.len()
}

fn test5() {
    let mut s = String::from("hello");
    test5_1(&mut s);
}

fn test5_1(s: &mut String) {
    s.push_str(", world");
}

fn test6() {
    let mut s = String::from("hello");
    let mut s2 = String::from("world");

    let r1 = &mut s;
    // let r10 = &s; // 只要不是可改的，就可以随便引用
    // let r2 = &mut s; // 这里会报 second mutable borrow occurs here
    { // 加个scope
        let r3 = &s2;
    }
    let r4 = &mut &s2;
}

// string slice
// slice 当borrow发生时，
fn test7() {
    let s = String::from("hello world!");

    let hello = &s[0..5];
    let world = &s[6..];
    println!("${}$", hello);
    println!("${}$", world);
}

fn test8() {
    let mut s = String::from("hello world");
    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            println!("{}", &s[0..i]);
            break;
        }
    }
    println!("{}", &s[..]);
}

fn test9() {
    let a = [1, 2, 3, 4];
    let slice = &a[1..3];

    println!("{}", slice[1]);
}


// strcut
#[derive(Debug)]  // 很关键
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn test10() {
    let user1 = User {
        email: String::from("someone@mail.com"),
        username: String::from("somename"),
        active: true,
        sign_in_count: 1,
    };

    println!("{:#?}", user1); // '#'format的意思
}

// method syntax
// p1.distance(&p2); 等同于 (&p1).distance(&p2);
#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&mut self) {
        self.width *= 3;
        println!("{}", self.length * self.width);
    }

    fn can_hold(&self, other: &Rectangle) {
        println!("{}", self.length > other.length && self.width > other.width);
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}
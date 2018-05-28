fn main() {
    test1();
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
    println!(s); // 非法，不可以再使用

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



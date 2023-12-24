// 接口
trait Area {
    fn area(&self) -> f64;
}
 
// 具体类
struct Circle {
    r: f64
}
 
// 让【具体类】实现【接口】
impl Area for Circle {
    fn area(&self) -> f64 {
        (3.14 * self.r) // 作为返回值 => 必须使用 () 括起来，并不能写 ;
    }
}
 
fn main()
{
    let r = Circle {r:10.5};
    println!("area = {:?}", r.area());
}

struct User
{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User
{
    User
    {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct Rectangle
{
    width: u32,
    height: u32,
}
impl Rectangle
{
    fn area(&self) -> u32
    {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool
    {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle
    {
        Rectangle { width: size, height: size }
    }
}
fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let width1 = 30;
    let height1 = 50;
    let rect1 = Rectangle { width: width1, height: height1 };
    let rect2 = Rectangle { width: 10, height: 40 };
    println!("rect1 is {:?} pixels wide.", rect1);
    println!("The area of the rectangle is {} square pixels.", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    let sq = Rectangle::square(3);
    println!("sq is {:?} pixels wide.", sq);
}

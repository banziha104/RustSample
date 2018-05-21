# Traite

> 메서드에 대한 설명만 포함하며, 인터페이스와 유사

```rust
trait Student {
    fn new(grade : i32) -> Self;
    fn get_grade(&self) -> i32;
}

struct Elemental {
    grade : i32
}

impl Student for Elemental{
    fn new(grade: i32) -> Self {
        Elemental{grade}
    }

    fn get_grade(&self) -> i32 {
        self.grade
    }
}

fn main() {
    let mut a: Elemental  = Student::new(32);
    println!("{}",a.get_grade());
    println!("{}",Student::get_grade(&a));
}
```



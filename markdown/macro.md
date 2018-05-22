# 매크로

- 내장 매크로 사용시 뒤에 !를 붙임
- 매크로 개발
    - 매크로는 macro_rules 매크로를 통해 개발
    - 매크로는 패턴 매칭 대해 하나 이상의 규칙을 정함 
    - $arg : 메타 변수를 값으로 바인딩함
    
```rust
macro_rules! welcome{
    () => (
        println!("Welcome to the Game")
    );
    ($arg:expr) =>(println!("arg is {}", $arg))
    ($($arg:expr), *) => ({$( print!("{}/",$arg)); *}) // 반복
}

fn main() {
    welcome!("aaaaa",555555)
}

```
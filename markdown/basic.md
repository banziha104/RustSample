# Rust Basic

- !로 끝나는 함수는 매크로 함수

```rust
// 주석
static MAX_HEALTH: i32 = 100; // 상수선언
static GAME_NAME : &'static str = "Monster Attack";

fn main(){
    const PI: f32 = 3.14;
    println!("The game");
    println!("{0},{1}",MAX_HEALTH,GAME_NAME); // 문자열 포매팅
    println!("{point}",point = 10); // 명명된 인수 사용
    
}


```

# 값과 타입

- 초기화된 상수들은 값을 같는다
- 아무 접미사를 붙이지 않으면 i32, f64가 기본타입이됌
- let 키워드로 값 바인딩
- 러스트에서 변수는 기본적으로 변경이 불가능하다. mut 키워드를 통해명시적으로 나타내야함
- 네이밍 컨벤션은 스네이크 케이스를 따름



```rust

static mut A : i32 = 10; // 변경가능한 전역상수, 타입을 지정해주어야함

fn main(){
    type myType = i32 ; // 에일리어싱
    let b : myType = 10;
    
    let a = 5;
    let b = false;
    let empty = ();
    let mut n = 10; // 변경가능한상수

    {   // 스코프 별도 생성
        let c = 30;
    }
    
    let casting = b as i32; //타입 캐스팅

//    print!(c) 에러
}

```

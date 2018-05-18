# 조건문

```rust

fn main(){

    let a = 10;

    if a > 5 {
        print!("5보다큼");
    }else {
        print!("5보다 작음");
    }

    let b = // 변수에 표현식을 담을 수 있음
        if a > 5 {
            "5보다큼"
        }else {
            "5보다 작음"
        };

    print!("{0}",b)
}
```

# 반복문


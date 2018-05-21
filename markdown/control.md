# 조건문

- if 문

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

- match 문

```rust

let loki = ("Loki", true, 800u32); 
    match loki {
        (name, demi, _) if demi => {
                                    print!("This is a demigod ");
                                    println!("called {}", name); },
        (name, _, _) if name == "Thor" =>
                                    println!("This is Thor!"),
        (_, _, pow) if pow <= 1000 =>
                                   rintln!("This is a powerless god"),
        _ => println!("This is something else")
}

```

# 반복문

- loop 문

```rust
loop{   // 무한루프
    if a > 5 { break }
}
```

- while 문

```rust
while a > 5 {

}
```

- for 문


```rust
for n in 1..11{

}
```




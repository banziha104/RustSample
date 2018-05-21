# 반복자

> 컬렉션의 아이템을 처음부터 마지막까지 순서대로 반환하는 객체이며, 다음 아이템을 반환하려면 next() 메서드를 사용

```rust


    for alien in aliens.iter() { print!("{} / ", alien)}
    for alien in &aliens { print!("{} / ", alien)} // 위와 같다
   
          
    let mut rng = 0..7;

    println!("> {:?}", rng.next()); // Some(0)
    println!("> {:?}", rng.next()); // Some(1)

    loop {
    	match rng.next() {
        	Some(x) => {
            	print!("{}", x);
        	},
        	None => { break }
    	}
    }
	println!("");

	// shorter way:
	let mut rng = 0..7;
    for n in rng {
        print!("{} - ", n);
    }
```

# 컨슈머와 어댑터 

```rust
fn main() {
    // collect 컨슈머는 전체를 요소 전체를 수집한 컨테이너를 반환
    let mut rng = 0..1000;
    let rngvec = rng.collect::<Vec<i32>>(); //
    print!("{:?}",rngvec);


    // find 컨슈머는 조건을 참으로 만드는 반복자의 첫번째 값을 얻어 Option 으로 반환
    let forty_two = rng.find(|n| *n >= 42);
    println!("{:?}", forty_two); // Some(42) 출력
    
    
}
```


# 표현식



```rust
	let n1 = {
		let a = 2;
		let b = 5;
		a + b    // <-- 세미콜론이 없으면 리턴됌
	};
	println!("n1 is: {}", n1);  // n1 is 7


	let n2 = {
		let a = 2;
		let b = 5;
		a + b; // 세미콜론을 찍는 경우에는 표현식으로 생각하지안ㅇ흠.
	};
	println!("n2 is: {:?}", n2);  // n2 is ()
```
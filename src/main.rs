fn main() {
    // collect 컨슈머는 전체를 요소 전체를 수집한 컨테이너를 반환
    let mut rng = 0..1000;
    let rngvec = rng.collect::<Vec<i32>>(); //
    print!("{:?}",rngvec);


    // find 컨슈머는 조건을 참으로 만드는 반복자의 첫번째 값을 얻어 Option 으로 반환
    let forty_two = rng.find(|n| *n >= 42);
    println!("{:?}", forty_two); // Some(42) 출력



}
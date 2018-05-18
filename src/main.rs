fn main(){
    let a = 10;
    let mut i = 0;
    while i < a {
        i += 1;
    }

    'outer : loop { // 무한 루프 , outer 라는 라벨을 붙임
        if i < 10 {
            continue;
        }else{
            break 'outer;
        }
    }

    for n in 1..10{ // for문

    }

}

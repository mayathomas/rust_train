fn main() {
   
}

#[allow(unused)]
fn aaa(){
    struct Y<'yy> {
        fix: &'yy dyn Fn(&Y, u32) -> u32,
    }
    let fibonacci = {
        let yc = Y {
            fix: &|y, n| {
                if n <= 1 {
                    n
                } else {
                    (y.fix)(y, n - 1) + (y.fix)(y, n - 2)
                }
            },
        };
        move |i| (yc.fix)(&yc, i)
    };
    let factorial = {
        let yc = Y {
            fix: &|y, n| if n == 0 { 1 } else { n * (y.fix)(y, n - 1) },
        };
        move |i| (yc.fix)(&yc, i)
    };
    assert_eq!(fibonacci(7), 13);
    assert_eq!(factorial(7), 5040);
}

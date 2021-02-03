/// 1. macro usage
// let fib = recurrence![a[n]: u64 = 0, 1, ..., a[n-1] + a[n-2]];
// for e in fib.take(10) {
//     println!("{}", e)
// }

macro_rules! recurrence {
    ( a[n]: $sty: ty = $($inits:expr),+ , ... , $recur:expr ) => {
        struct Recurrence {
            mem: [u64; 2],
            pos: usize,
        }

        impl Itertor for Recurrence {
            type Item = u64;

            fn next(&mut self) -> Option<u64> {
                if self.pos < 2 {
                    let next_val = self.mem[self.pos];
                    self.pos += 1;
                    Some(next_val)
                } else {
                    let n = self.pos;
                    let next_val = a[n-1] + a[n-2];
                    // todo: append to mem
                    self.pos += 1;
                    Some(next_val)
                }
            }
        }
        todo!();
    };
}

fn main() {
    println!("Hello, world!");
}

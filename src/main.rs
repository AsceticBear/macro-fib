/// 1. macro usage
// let fib = recurrence![a[n]: u64 = 0, 1, ..., a[n-1] + a[n-2]];
// for e in fib.take(10) {
//     println!("{}", e)
// }

// macro_rules! recurrence {
//     ( a[n]: $sty: ty = $($inits:expr),+ , ... , $recur:expr ) => {
//         struct Recurrence {
//             mem: [u64; 2],
//             pos: usize,
//         }

//         impl Itertor for Recurrence {
//             type Item = u64;

//             fn next(&mut self) -> Option<u64> {
//                 if self.pos < 2 {
//                     let next_val = self.mem[self.pos];
//                     self.pos += 1;
//                     Some(next_val)
//                 } else {
//                     let n = self.pos;
//                     let next_val = a[n-1] + a[n-2];
//                     // todo: append to mem
//                     self.pos += 1;
//                     Some(next_val)
//                 }
//             }
//         }
//         todo!();
//     };
// }

fn main() {
    let fib = {
        use std::ops::Index;

        struct Recurrence {
            mem: [u64; 2],
            pos: usize,
        }

        #[derive(Debug)]
        struct IndexOffset<'a> {
            slice: &'a [u64; 2],
            offset: usize,
        }
        impl<'a> Index<usize> for IndexOffset<'a> {
            type Output = u64;

            #[inline(always)]
            fn index<'b>(&'b self, index: usize) -> &'b u64 {
                use std::num::Wrapping;
                let index = Wrapping(index);
                let offset = Wrapping(self.offset);
                let window = Wrapping(2);
                let real_index = index - offset + window;
                &self.slice[real_index.0]
            }
        }

        impl Iterator for Recurrence {
            type Item = u64;
            #[inline]
            fn next(&mut self) -> Option<u64> {
                if self.pos < 2 {
                    let next_val = self.mem[self.pos];
                    self.pos += 1;
                    Some(next_val)
                } else {
                    let n = self.pos;
                    let a = IndexOffset {
                        slice: &self.mem,
                        offset: n,
                    };
                    let next_val = a[n - 1] + a[n - 2];
                    
                    {
                        use std::mem::swap;
                        let mut swap_tmp = next_val;
                        for i in (0..2).rev() { // 1, 0
                            swap(&mut swap_tmp, &mut self.mem[i]);
                        }
                    }
                    
                    self.pos += 1;
                    Some(next_val)
                }
            }
        };

        Recurrence {
            mem: [0, 1],
            pos: 0,
        }
    };

    for e in fib.take(9) {
        println!("{}", e);
    }
}

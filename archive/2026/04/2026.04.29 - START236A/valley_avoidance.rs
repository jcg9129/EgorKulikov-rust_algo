//{"name":"Valley Avoidance","group":"CodeChef - START236A","url":"https://www.codechef.com/START236A/problems/AVVAL","interactive":false,"timeLimit":3000,"tests":[{"input":"4\n4 4\n0 0 0 0\n4 4\n0 3 0 0\n5 3\n0 4 0 3 0\n10 6\n0 0 4 0 1 7 0 0 2 0\n","output":"20\n6\n2\n54\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_utils::PartialSums;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_size_vec(n);

    let mut free = vec![1; n + 1];
    for i in a.copy_iter() {
        free[i] = 0;
    }
    let sf = free.partial_sums();
    let free = Vec::with_gen(n, |i| if a[i] == 0 { 1 } else { 0 });
    let sl = free.partial_sums();
    type Mod = ModIntF;
    let mut pos = vec![n; n + 1];
    for i in 0..n {
        pos[a[i]] = i;
    }
    let less = Vec::with_gen(n + 1, |i| {
        let is_less = Vec::with_gen(n, |j| if a[j] <= i { 1 } else { 0 });
        is_less.partial_sums()
    });
    let mut a1 = Arr2d::new(n, n, Mod::new(0));
    let mut a2 = Arr2d::new(n, n, Mod::new(0));
    let mut a3 = Arr2d::new(n, n, Mod::new(0));
    let mut na1 = Arr2d::new(n, n, Mod::new(0));
    let mut na2 = Arr2d::new(n, n, Mod::new(0));
    let mut na3 = Arr2d::new(n, n, Mod::new(0));
    for at in 0..n {
        na1.fill(Mod::new(0));
        na2.fill(Mod::new(0));
        na3.fill(Mod::new(0));
        let used = sf[n + 1] - sf[at + 1];
        for l in 0..n {
            for r in (l..n).rev() {
                let base = if less[at][r + 1] - less[at][l] > used + k - 3 {
                    Mod::new(0)
                } else if at == 0 {
                    Mod::new(1)
                } else {
                    if pos[at] != n {
                        a1[(l.min(pos[at]), r.max(pos[at]))]
                    } else {
                        let have = sl[r + 1] - sl[l];
                        let mut res = Mod::new(0);
                        if have > used {
                            res += a1[(l, r)] * (have - used);
                        }
                        if l != 0 {
                            res += na2[(l - 1, r)];
                        }
                        if r != n - 1 {
                            res += na3[(l, r + 1)];
                        }
                        // for i in (l.saturating_sub(k - 2)..l).rev() {
                        //     if a[i] == 0 {
                        //         let cand = mem.call(at - 1, i, r);
                        //         if cand == Mod::new(0) {
                        //             break;
                        //         }
                        //         res += cand;
                        //     }
                        // }
                        // for i in r + 1..=(r + k - 2).min(n - 1) {
                        //     if a[i] == 0 {
                        //         let cand = mem.call(at - 1, l, i);
                        //         if cand == Mod::new(0) {
                        //             break;
                        //         }
                        //         res += cand;
                        //     }
                        // }
                        res
                    }
                };
                na1[(l, r)] = base;
                let mut left = if l != 0 { na2[(l - 1, r)] } else { Mod::new(0) };
                if a[l] == 0 && at != 0 {
                    left += a1[(l, r)];
                }
                na2[(l, r)] = left;
                let mut right = if r != n - 1 {
                    na3[(l, r + 1)]
                } else {
                    Mod::new(0)
                };
                if a[r] == 0 && at != 0 {
                    right += a1[(l, r)];
                }
                na3[(l, r)] = right;
            }
        }
        std::mem::swap(&mut a1, &mut na1);
        std::mem::swap(&mut a2, &mut na2);
        std::mem::swap(&mut a3, &mut na3);
    }
    /*let mut mem = Memoization3d::new(n + 1, n, n, |mem, at, l, r| -> (Mod, Mod, Mod) {
        let used = sf[n + 1] - sf[at + 1];
        let base = if less[at][r + 1] - less[at][l] > used + k - 3 {
            Mod::new(0)
        } else if at == 0 {
            Mod::new(1)
        } else {
            if pos[at] != n {
                mem.call(at - 1, l.min(pos[at]), r.max(pos[at])).0
            } else {
                let have = sl[r + 1] - sl[l];
                let mut res = Mod::new(0);
                if have > used {
                    res += mem.call(at - 1, l, r).0 * (have - used);
                }
                if l != 0 {
                    res += mem.call(at, l - 1, r).1;
                }
                if r != n - 1 {
                    res += mem.call(at, l, r + 1).2;
                }
                // for i in (l.saturating_sub(k - 2)..l).rev() {
                //     if a[i] == 0 {
                //         let cand = mem.call(at - 1, i, r);
                //         if cand == Mod::new(0) {
                //             break;
                //         }
                //         res += cand;
                //     }
                // }
                // for i in r + 1..=(r + k - 2).min(n - 1) {
                //     if a[i] == 0 {
                //         let cand = mem.call(at - 1, l, i);
                //         if cand == Mod::new(0) {
                //             break;
                //         }
                //         res += cand;
                //     }
                // }
                res
            }
        };
        let mut left = if l != 0 {
            mem.call(at, l - 1, r).1
        } else {
            Mod::new(0)
        };
        if a[l] == 0 && at != 0 {
            left += mem.call(at - 1, l, r).0;
        }
        let mut right = if r != n - 1 {
            mem.call(at, l, r + 1).2
        } else {
            Mod::new(0)
        };
        if a[r] == 0 && at != 0 {
            right += mem.call(at - 1, l, r).0;
        }
        (base, left, right)
    });*/
    let mut ans = Mod::new(0);
    if pos[n] != n {
        ans = a1[(pos[n], pos[n])];
    } else {
        for i in 0..n {
            if a[i] == 0 {
                ans += a1[(i, i)];
            }
        }
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
        _ => {
            unreachable!();
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
}

#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}

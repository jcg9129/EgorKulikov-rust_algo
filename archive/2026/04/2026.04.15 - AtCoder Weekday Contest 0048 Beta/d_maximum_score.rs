//{"name":"D - Maximum Score","group":"AtCoder - AtCoder Weekday Contest 0048 Beta","url":"https://atcoder.jp/contests/awc0048/tasks/awc0048_d","interactive":false,"timeLimit":2000,"tests":[{"input":"3 4 6\n1234\n5678\n9012\n","output":"30\n"},{"input":"4 5 7\n12345\n67890\n11111\n99999\n","output":"-1\n"},{"input":"5 6 12\n193842\n681937\n274619\n938271\n526184\n","output":"68\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let s = input.read_char_table(n, m);

    let mut t = Arr2d::new(n + 1, m + 1, 0);
    for i in 1..=n {
        for j in 1..=m {
            t[(i, j)] = t[(i - 1, j)] + t[(i, j - 1)] - t[(i - 1, j - 1)]
                + (s[(i - 1, j - 1)] as i32 - b'0' as i32);
        }
    }
    let mut ans = None;
    for x in 1..=n {
        if k % x == 0 && k / x <= m {
            let y = k / x;
            for i in 0..=n - x {
                for j in 0..=m - y {
                    ans.maxim(t[(i, j)] + t[(i + x, j + y)] - t[(i + x, j)] - t[(i, j + y)]);
                }
            }
        }
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::YesNo);

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

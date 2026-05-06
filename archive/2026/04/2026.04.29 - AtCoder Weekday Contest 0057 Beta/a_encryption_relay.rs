//{"name":"A - Encryption Relay","group":"AtCoder - AtCoder Weekday Contest 0057 Beta","url":"https://atcoder.jp/contests/awc0057/tasks/awc0057_a","interactive":false,"timeLimit":2000,"tests":[{"input":"5 10\n1 2 1 3 3\n","output":"10\n"},{"input":"3 5\n7 7 7\n","output":"2\n"},{"input":"8 123\n8 1 8 3 3 5 7 5\n","output":"123\n"},{"input":"20 999999999\n0 1000000000 0 5 5 7 8 7 3 3 3 9 1 9 4 6 4 2 2 2\n","output":"999999998\n"},{"input":"1 0\n0\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::ops::BitXor;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let x = input.read_unsigned();
    let a = input.read_unsigned_vec(n);

    let mut b = a.clone();
    for i in 1..n - 1 {
        if a[i - 1] == a[i + 1] && a[i] != a[i - 1] {
            b[i] = 0;
        }
    }
    out.print_line(x ^ b.copy_fold(0, u32::bitxor));
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

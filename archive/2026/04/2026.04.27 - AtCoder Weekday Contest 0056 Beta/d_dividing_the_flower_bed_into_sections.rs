//{"name":"D - Dividing the Flower Bed into Sections","group":"AtCoder - AtCoder Weekday Contest 0056 Beta","url":"https://atcoder.jp/contests/awc0056/tasks/awc0056_d","interactive":false,"timeLimit":2000,"tests":[{"input":"5 2\n3 1 4 1 5\n","output":"3\n"},{"input":"6 3\n10 1 10 1 10 1\n","output":"9\n"},{"input":"10 4\n5 8 3 7 2 9 1 6 4 10\n","output":"8\n"},{"input":"20 5\n15 3 12 7 20 1 18 5 14 9 2 16 8 11 4 19 6 13 10 17\n","output":"19\n"},{"input":"1 1\n42\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_int_vec(n);

    let mut mem = Memoization2d::new(n + 1, k + 1, |mem, pos, rem| -> i32 {
        if pos == n {
            0
        } else if rem == 0 {
            i32::MAX / 2
        } else {
            let mut res = i32::MAX / 2;
            let mut min = a[pos];
            let mut max = a[pos];
            for i in pos..n {
                min.minim(a[i]);
                max.maxim(a[i]);
                res.minim(max - min + mem.call(i + 1, rem - 1));
            }
            res
        }
    });
    out.print_line(mem.call(0, k));
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

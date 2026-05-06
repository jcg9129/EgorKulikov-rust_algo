//{"name":"A - Flipping Switches","group":"AtCoder - AtCoder Weekday Contest 0048 Beta","url":"https://atcoder.jp/contests/awc0048/tasks/awc0048_a","interactive":false,"timeLimit":2000,"tests":[{"input":"3\nYes 1\nNo 2\nYes 3\n","output":"No\nNo\nNo\n"},{"input":"4\nYes 2\nNo 1\nNo 4\nYes 5\n","output":"Yes\nYes\nNo\nNo\n"},{"input":"6\nYes 100\nNo 99\nYes 1000000\nNo 999999\nYes 42\nNo 7\n","output":"Yes\nYes\nYes\nYes\nYes\nYes\n"},{"input":"10\nYes 1000000000\nNo 1000000000\nYes 999999999\nNo 999999999\nYes 1\nNo 1\nYes 2\nNo 2\nYes 500000000\nNo 500000001\n","output":"Yes\nNo\nNo\nYes\nNo\nYes\nYes\nNo\nYes\nYes\n"},{"input":"1\nNo 1\n","output":"Yes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let on = input.read_str().as_slice() == b"Yes";
    let k = input.read_int();

    out.print_line(on ^ (k % 2 == 1));
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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

//{"name":"A - Arranged Dish from a Recipe","group":"AtCoder - AtCoder Weekday Contest 0056 Beta","url":"https://atcoder.jp/contests/awc0056/tasks/awc0056_a","interactive":false,"timeLimit":2000,"tests":[{"input":"3 2\n10 20 30\n1 5\n3 -10\n","output":"35\n"},{"input":"5 4\n100 200 300 400 500\n2 50\n4 -100\n1 200\n5 0\n","output":"1350\n"},{"input":"10 8\n1000000000 500000000 300000000 700000000 100000000 900000000 200000000 800000000 600000000 400000000\n1 -500000000\n6 1000000000\n3 -300000000\n10 999999999\n5 100000000\n8 -800000000\n2 500000000\n7 0\n","output":"5199999999\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_long_vec(n);
    let bs = input.read_vec::<(usize, i64)>(m);

    let mut ans = 0;
    for i in 0..m {
        ans += a[bs[i].0 - 1] + bs[i].1;
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

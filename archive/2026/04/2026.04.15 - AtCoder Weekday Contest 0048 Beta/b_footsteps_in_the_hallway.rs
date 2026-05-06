//{"name":"B - Footsteps in the Hallway","group":"AtCoder - AtCoder Weekday Contest 0048 Beta","url":"https://atcoder.jp/contests/awc0048/tasks/awc0048_b","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3 0 5 2 4\n1 1 1 1 1\n","output":"3\n"},{"input":"8\n5 0 3 0 7 0 2 4\n2 3 0 1 1 5 1 3\n","output":"2\n"},{"input":"15\n1 1 0 1 0 1 1 0 0 1 1 0 1 1 1\n1 1 1 1 1 1 1 1 1 1 1 1 1 1 1\n","output":"5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let h = input.read_int_vec(n);
    let d = input.read_int_vec(n);

    let mut ans = 0;
    if h[0] > 0 && d[0] > 0 {
        ans += 1;
    }
    if h[n - 1] > 0 && d[n - 1] > 0 {
        ans += 1;
    }
    let mut qty = 0;
    for i in 1..n - 1 {
        if h[i] > 0 && d[i] > 0 {
            qty += 1;
        } else {
            ans += qty / 2;
            qty = 0;
        }
    }
    ans += qty / 2;
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

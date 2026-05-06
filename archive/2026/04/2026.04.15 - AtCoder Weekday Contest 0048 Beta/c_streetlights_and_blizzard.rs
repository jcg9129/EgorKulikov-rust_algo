//{"name":"C - Streetlights and Blizzard","group":"AtCoder - AtCoder Weekday Contest 0048 Beta","url":"https://atcoder.jp/contests/awc0048/tasks/awc0048_c","interactive":false,"timeLimit":2000,"tests":[{"input":"5 2\n10 5 8 3 7\n2 4 3\n1 3 4\n","output":"3\n"},{"input":"3 2\n3 2 1\n1 3 2\n1 1 1\n","output":"0\n"},{"input":"10 5\n100 50 30 80 60 40 90 20 70 10\n1 5 20\n3 8 15\n6 10 25\n1 10 10\n2 4 30\n","output":"5\n"},{"input":"15 6\n1000000000 500 300 800 600 400 900 200 700 100 550 350 750 450 1000000000\n1 15 100\n1 8 200\n5 12 150\n3 10 50\n7 15 100\n1 15 50\n","output":"10\n"},{"input":"1 1\n1\n1 1 1\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let h = input.read_long_vec(n);

    let mut delta = vec![0; n + 1];
    for _ in 0..m {
        let l = input.read_size() - 1;
        let r = input.read_size();
        let d = input.read_long();
        delta[l] += d;
        delta[r] -= d;
    }
    let mut ans = 0;
    let mut cur = 0;
    for i in 0..n {
        cur += delta[i];
        if cur < h[i] {
            ans += 1;
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

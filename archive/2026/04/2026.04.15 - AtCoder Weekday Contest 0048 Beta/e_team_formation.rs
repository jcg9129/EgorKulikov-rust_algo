//{"name":"E - Team Formation","group":"AtCoder - AtCoder Weekday Contest 0048 Beta","url":"https://atcoder.jp/contests/awc0048/tasks/awc0048_e","interactive":false,"timeLimit":2000,"tests":[{"input":"4 2 3\n1 2 3 4\n","output":"2\n"},{"input":"6 3 5\n1 2 3 4 5 6\n","output":"4\n"},{"input":"10 5 7\n1 2 3 4 5 6 7 8 9 10\n","output":"36\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let m = input.read_long();
    let a = input.read_long_vec(n);

    fn build(a: &[i64], m: i64) -> Vec<DefaultHashMap<i64, usize>> {
        let mut res = vec![DefaultHashMap::default(); a.len() + 1];
        for i in usize::iter_all(a.len()) {
            let mut sum = 0;
            for j in a.indices() {
                if i.is_set(j) {
                    sum += a[j];
                }
            }
            sum %= m;
            res[i.count_ones() as usize][sum] += 1;
        }
        res
    }
    let left = build(&a[..n / 2], m);
    let right = build(&a[n / 2..], m);
    let mut ans = 0;
    for i in left.indices().take(k + 1) {
        let r = k - i;
        if r < right.len() {
            for (&sum, &cnt) in left[i].iter() {
                ans += cnt * right[r][(m - sum) % m];
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

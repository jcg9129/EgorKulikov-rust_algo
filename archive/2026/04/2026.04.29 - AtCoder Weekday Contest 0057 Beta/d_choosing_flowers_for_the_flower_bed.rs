//{"name":"D - Choosing Flowers for the Flower Bed","group":"AtCoder - AtCoder Weekday Contest 0057 Beta","url":"https://atcoder.jp/contests/awc0057/tasks/awc0057_d","interactive":false,"timeLimit":2000,"tests":[{"input":"3 2 1\n5 3 -2\n1 2 10\n","output":"18\n"},{"input":"3 1 2\n-10 -10 -10\n1 1 3\n2 3 2\n","output":"0\n"},{"input":"6 3 4\n10 -5 8 -3 7 -1\n1 3 15\n2 5 20\n4 6 10\n1 6 5\n","output":"75\n"},{"input":"10 5 8\n100 -50 80 -30 70 -10 60 -40 90 -20\n1 3 200\n2 5 150\n4 7 180\n6 10 120\n1 5 100\n3 8 90\n5 10 110\n1 10 50\n","output":"1400\n"},{"input":"1 1 1\n-1000\n1 1 1\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
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
    let m = input.read_size();
    let s = input.read_int_vec(n);
    let lrp = input.read_vec::<(usize, usize, i32)>(m).dec();

    let mut ans = 0;
    for i in usize::iter_all(n) {
        if i.count_ones() as usize > k {
            continue;
        }
        let mut cur = 0;
        for j in 0..n {
            if i.is_set(j) {
                cur += s[j];
            }
        }
        for (l, r, p) in lrp.copy_iter() {
            if (i >> l) & usize::all_bits(r - l + 1) != 0 {
                cur += p;
            }
        }
        ans.maxim(cur);
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

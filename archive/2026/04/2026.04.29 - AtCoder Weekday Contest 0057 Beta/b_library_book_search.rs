//{"name":"B - Library Book Search","group":"AtCoder - AtCoder Weekday Contest 0057 Beta","url":"https://atcoder.jp/contests/awc0057/tasks/awc0057_b","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n2 algorithm data\n2 string graph\n3\nag\nri\nzz\n","output":"1\n2\n0\n"},{"input":"3\n1 programming\n2 problem process\n2 apple application\n4\npo\npin\npp\nrm\n","output":"3\n2\n1\n2\n"},{"input":"5\n3 binary search tree\n2 dynamic programming\n1 graph\n2 stack queue\n3 linked list array\n5\nar\nra\ni\nzzz\nee\n","output":"2\n3\n3\n0\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut books = Vec::with_capacity(n);
    for _ in 0..n {
        let k = input.read_size();
        books.push(input.read_str_vec(k));
    }
    let q = input.read_size();

    for _ in 0..q {
        let t = input.read_str();
        let mut ans = 0;
        for i in 0..n {
            for b in books[i].iter() {
                let mut at = 0;
                for c in b.copy_iter() {
                    if c == t[at] {
                        at += 1;
                        if at == t.len() {
                            ans += 1;
                            break;
                        }
                    }
                }
                if at == t.len() {
                    break;
                }
            }
        }
        out.print_line(ans);
    }
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

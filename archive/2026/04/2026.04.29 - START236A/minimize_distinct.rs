//{"name":"Minimize Distinct","group":"CodeChef - START236A","url":"https://www.codechef.com/START236A/problems/MINDSTC","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n4\n1 3 2 4\n5\n2 1 2 1 2\n5\n1 5 4 2 4\n","output":"2 3 4 4\n2 2 2 2 2\n3 3 4 4 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut a = input.read_size_vec(n).sorted();

    a.dedup();
    let mut max2 = 0;
    let mut max1 = 0;
    for i in a.indices() {
        if i + 1 < a.len() && a[i + 1] == a[i] + 2 {
            max1 = a[i];
        }
        if i + 2 < a.len() && a[i + 2] == a[i] + 2 {
            max1 = a[i];
        }
        if i + 3 < a.len() && a[i + 3] == a[i] + 3 {
            max2 = a[i];
        }
    }
    let ans = Vec::with_gen(n, |i| {
        if i < max2 {
            a.len() - 2
        } else if i < max1 {
            a.len() - 1
        } else {
            a.len()
        }
    });
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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

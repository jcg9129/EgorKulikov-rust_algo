//{"name":"E - Delivery Driver's Route","group":"AtCoder - AtCoder Weekday Contest 0056 Beta","url":"https://atcoder.jp/contests/awc0056/tasks/awc0056_e","interactive":false,"timeLimit":2000,"tests":[{"input":"4 5\n1 2 2\n2 3 3\n3 4 4\n1 4 7\n1 3 10\n1 2\n3 4\n","output":"16\n"},{"input":"6 8\n1 2 3\n1 3 5\n2 3 1\n2 4 7\n3 5 4\n4 5 2\n4 6 3\n5 6 6\n1 3\n4 5 6\n","output":"26\n"},{"input":"10 14\n1 2 5\n1 3 12\n2 3 4\n2 4 8\n3 5 3\n4 5 6\n4 6 2\n5 7 7\n6 7 4\n6 8 9\n7 9 3\n8 9 5\n8 10 2\n9 10 6\n1 5\n3 5 7 9 10\n","output":"54\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::distances::Distances;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let edges = input.read_vec::<(usize, usize, i64)>(m).dec();
    let s = input.read_size() - 1;
    let k = input.read_size();
    let mut d = input.read_size_vec(k).dec();

    d.push(s);
    let mut graph = Graph::new(n);
    for (u, v, w) in edges {
        graph.add_edge(BiWeightedEdge::new(u, v, w));
    }
    let dst = Vec::with_gen(k + 1, |i| {
        graph
            .distances_from(d[i])
            .into_iter()
            .map(|x| x.map_or(0, |x| x.0))
            .collect::<Vec<_>>()
    });
    let mut mem = Memoization2d::new(k + 1, 1 << (k + 1), |mem, pos, mask| -> i64 {
        if mask == usize::all_bits(k + 1) && pos == k {
            0
        } else {
            let mut res = i64::MAX / 2;
            for i in 0..=k {
                if !mask.is_set(i) {
                    res.minim(dst[pos][d[i]] + mem.call(i, mask.with_bit(i)));
                }
            }
            res
        }
    });
    let mut ans = i64::MAX;
    for i in 0..k {
        ans.minim(dst[k][d[i]] + mem.call(i, 1 << i));
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

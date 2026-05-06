//{"name":"C. Zhily and Signpost","group":"Codeforces - Codeforces Round 1097 (Div. 1,  Based on Zhili Cup 2026)","url":"https://codeforces.com/contest/2223/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n3 1\n1 1\n10 20\n4\n10 5\n1 2 2 2 1 1 3 4 5\n1 2 3 4 5 6 7 8 9\n1 2 3 4 5\n","output":"2\n6 7 9 6 7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable4, RecursiveFunction4};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::gcd::lcm;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let f = input.read_size_vec(n - 1).dec();
    let l = input.read_size_vec(n - 1);

    let mut graph = Graph::new(n);
    for i in 0..n - 1 {
        graph.add_edge(Edge::new(f[i], i + 1));
    }

    let mut mm = vec![0; n];
    let mut xx = vec![0; n];
    let mut next = vec![Vec::new(); n];
    let mut dfs = RecursiveFunction4::new(|dfs, v: usize, m: i128, x: i128, k: i128| {
        mm[v] = m;
        xx[v] = x;
        if graph[v].len() == 0 {
            next[v] = vec![v];
            return;
        }
        let nm = if m <= 1_000_000_000_000_000_000 {
            lcm(m, graph[v].len() as i128)
        } else {
            m
        };
        for d in 0..nm / m {
            let cur = x + d * m;
            if cur > 1_000_000_000_000_000_000 {
                break;
            }
            let id = graph[v][((cur + k) % (graph[v].len() as i128)) as usize].to();
            dfs.call(id, nm, cur, k + l[id - 1] as i128);
            if next[id].len() == 1 {
                let dest = next[id][0];
                next[v].push(dest);
            } else {
                next[v].push(id);
            }
        }
    });
    dfs.call(0, 1, 0, 0);

    out.print_line(Vec::with_gen(q, |_| {
        let m = input.read_i128();
        let mut v = 0;
        while graph[v].len() > 0 {
            let dest = (m / mm[v]) % (next[v].len() as i128);
            v = next[v][dest as usize];
        }
        v + 1
    }));
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();

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
        TestType::RunTwiceSingle => {
            let mode = input.read_str();
            match mode.as_slice() {
                b"first" => solve(&mut input, &mut output, 1, &mut pre_calc),
                b"second" => solve2(&mut input, &mut output, 1, &mut pre_calc),
                _ => unreachable!(),
            }
        }
        TestType::RunTwiceMultiNumber => {
            let mode = input.read_str();
            let t = input.read();
            for i in 1..=t {
                match mode.as_slice() {
                    b"first" => solve(&mut input, &mut output, i, &mut pre_calc),
                    b"second" => solve2(&mut input, &mut output, i, &mut pre_calc),
                    _ => unreachable!(),
                }
            }
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

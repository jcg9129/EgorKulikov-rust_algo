//{"name":"Numismatics","group":"CodeChef - START155A","url":"https://www.codechef.com/START155A/problems/NUMMAT","interactive":false,"timeLimit":2000,"tests":[{"input":"1\n7 4\n0101100\n2 1\n1 3 6\n1 2 4\n2 1\n","output":"3\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Numismatics"}}}

use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::mem::swap;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let s = input.read_str();

    #[derive(Copy, Clone)]
    enum State {
        AtEmpty,
        AtOccupied,
        Broken,
    }

    #[derive(Copy, Clone)]
    struct Node {
        empty_coins: usize,
        empty_to_state: State,
        occupied_coins: usize,
        occupied_to_state: State,
        rev_empty_coins: usize,
        rev_empty_to_state: State,
        rev_occupied_coins: usize,
        rev_occupied_to_state: State,
        delta: bool,
    }

    impl Node {
        fn new(s: u8) -> Self {
            match s {
                b'0' => Node {
                    empty_coins: 0,
                    empty_to_state: State::Broken,
                    occupied_coins: 0,
                    occupied_to_state: State::AtEmpty,
                    rev_empty_coins: 1,
                    rev_empty_to_state: State::AtOccupied,
                    rev_occupied_coins: 1,
                    rev_occupied_to_state: State::AtOccupied,
                    delta: false,
                },
                b'1' => Node {
                    empty_coins: 1,
                    empty_to_state: State::AtOccupied,
                    occupied_coins: 1,
                    occupied_to_state: State::AtOccupied,
                    rev_empty_coins: 0,
                    rev_empty_to_state: State::Broken,
                    rev_occupied_coins: 0,
                    rev_occupied_to_state: State::AtEmpty,
                    delta: false,
                },
                _ => unreachable!(),
            }
        }
    }

    impl SegmentTreeNode for Node {
        fn new(_left: usize, _right: usize) -> Self {
            Node {
                empty_coins: 0,
                empty_to_state: State::AtOccupied,
                occupied_coins: 0,
                occupied_to_state: State::AtOccupied,
                rev_empty_coins: 0,
                rev_empty_to_state: State::AtOccupied,
                rev_occupied_coins: 0,
                rev_occupied_to_state: State::AtOccupied,
                delta: false,
            }
        }

        fn join(&mut self, left_val: &Self, right_val: &Self) {
            self.empty_coins = left_val.empty_coins
                + match left_val.empty_to_state {
                    State::AtEmpty => right_val.empty_coins,
                    State::AtOccupied => right_val.occupied_coins,
                    State::Broken => 0,
                };
            self.empty_to_state = match left_val.empty_to_state {
                State::AtEmpty => right_val.empty_to_state,
                State::AtOccupied => right_val.occupied_to_state,
                State::Broken => State::Broken,
            };
            self.occupied_coins = left_val.occupied_coins
                + match left_val.occupied_to_state {
                    State::AtEmpty => right_val.empty_coins,
                    State::AtOccupied => right_val.occupied_coins,
                    State::Broken => 0,
                };
            self.occupied_to_state = match left_val.occupied_to_state {
                State::AtEmpty => right_val.empty_to_state,
                State::AtOccupied => right_val.occupied_to_state,
                State::Broken => State::Broken,
            };
            self.rev_empty_coins = left_val.rev_empty_coins
                + match left_val.rev_empty_to_state {
                    State::AtEmpty => right_val.rev_empty_coins,
                    State::AtOccupied => right_val.rev_occupied_coins,
                    State::Broken => 0,
                };
            self.rev_empty_to_state = match left_val.rev_empty_to_state {
                State::AtEmpty => right_val.rev_empty_to_state,
                State::AtOccupied => right_val.rev_occupied_to_state,
                State::Broken => State::Broken,
            };
            self.rev_occupied_coins = left_val.rev_occupied_coins
                + match left_val.rev_occupied_to_state {
                    State::AtEmpty => right_val.rev_empty_coins,
                    State::AtOccupied => right_val.rev_occupied_coins,
                    State::Broken => 0,
                };
            self.rev_occupied_to_state = match left_val.rev_occupied_to_state {
                State::AtEmpty => right_val.rev_empty_to_state,
                State::AtOccupied => right_val.rev_occupied_to_state,
                State::Broken => State::Broken,
            };
        }

        fn accumulate(&mut self, value: &Self) {
            if value.delta {
                swap(&mut self.empty_coins, &mut self.rev_empty_coins);
                swap(&mut self.empty_to_state, &mut self.rev_empty_to_state);
                swap(&mut self.occupied_coins, &mut self.rev_occupied_coins);
                swap(&mut self.occupied_to_state, &mut self.rev_occupied_to_state);
                self.delta = !self.delta;
            }
        }

        fn reset_delta(&mut self) {
            self.delta = false;
        }
    }
    let mut st = SegmentTree::from_generator(n, |i| Node::new(s[i]));
    for _ in 0..q {
        match input.read_int() {
            1 => {
                let l = input.read_size() - 1;
                let r = input.read_size() - 1;
                st.update(
                    l..=r,
                    &Node {
                        empty_coins: 0,
                        empty_to_state: State::Broken,
                        occupied_coins: 0,
                        occupied_to_state: State::AtEmpty,
                        rev_empty_coins: 0,
                        rev_empty_to_state: State::Broken,
                        rev_occupied_coins: 0,
                        rev_occupied_to_state: State::AtEmpty,
                        delta: true,
                    },
                );
            }
            2 => {
                let x = input.read_size() - 1;
                out.print_line(st.query(x..).occupied_coins);
            }
            _ => unreachable!(),
        }
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
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
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
use aoc_runner_derive::{aoc, aoc_generator};
use std::fmt;
use std::iter::Peekable;
use std::str::Chars;

type PeekChar<'a> = Peekable<Chars<'a>>;

#[derive(Debug, Clone)]
enum NodeValue {
    Literal(u32),
    Node(Box<Node>),
}

enum ExplodeResult {
    None,
    BeginExplode(u32, u32),
    Exploded,
}

impl NodeValue {
    fn try_explode(
        &mut self,
        depth: usize,
        left: &mut Option<&mut NodeValue>,
        right: &mut Option<&mut NodeValue>,
    ) -> ExplodeResult {
        match self {
            NodeValue::Literal(_) => ExplodeResult::None,
            NodeValue::Node(v) => v.try_explode(depth, left, right),
        }
    }

    fn try_split(&mut self) -> bool {
        match self {
            NodeValue::Literal(v) => {
                if *v >= 10 {
                    let half = *v / 2; // integer divide (floor)
                    let right = *v % 2; // any remainder for ceil
                    *self = NodeValue::Node(Box::new(Node {
                        left: NodeValue::Literal(half),
                        right: NodeValue::Literal(half + right),
                    }));
                    true
                } else {
                    false
                }
            }
            NodeValue::Node(v) => v.try_split(),
        }
    }

    fn add_moving_right(&mut self, value: u32) {
        match self {
            NodeValue::Literal(v) => *self = NodeValue::Literal(*v + value),
            NodeValue::Node(v) => v.add_moving_right(value),
        }
    }

    fn add_moving_left(&mut self, value: u32) {
        match self {
            NodeValue::Literal(v) => *self = NodeValue::Literal(*v + value),
            NodeValue::Node(v) => v.add_moving_left(value),
        }
    }

    fn get_magnitude(&self) -> u32 {
        match self {
            NodeValue::Literal(v) => *v,
            NodeValue::Node(v) => v.get_magnitude(),
        }
    }
}

impl fmt::Display for NodeValue {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NodeValue::Literal(v) => write!(f, "{}", *v),
            NodeValue::Node(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Debug, Clone)]
struct Node {
    left: NodeValue,
    right: NodeValue,
}

impl Node {
    fn try_explode(
        &mut self,
        depth: usize,
        left_sibling: &mut Option<&mut NodeValue>,
        right_sibling: &mut Option<&mut NodeValue>,
    ) -> ExplodeResult {
        let left_value = match self.left {
            NodeValue::Literal(val) => Some(val),
            _ => None,
        };
        let right_value = match self.right {
            NodeValue::Literal(val) => Some(val),
            _ => None,
        };

        if let (true, Some(left_val), Some(right_val)) = (depth > 3, left_value, right_value) {
            ExplodeResult::BeginExplode(left_val, right_val)
        } else {
            match self
                .left
                .try_explode(depth + 1, left_sibling, &mut Some(&mut self.right))
            {
                ExplodeResult::None => {
                    match self.right.try_explode(
                        depth + 1,
                        &mut Some(&mut self.left),
                        right_sibling,
                    ) {
                        ExplodeResult::None => ExplodeResult::None,
                        ExplodeResult::Exploded => ExplodeResult::Exploded,
                        ExplodeResult::BeginExplode(left_val, right_val) => {
                            if let Some(right_search) = right_sibling.as_mut() {
                                right_search.add_moving_left(right_val);
                            }
                            self.left.add_moving_right(left_val);
                            self.right = NodeValue::Literal(0);
                            ExplodeResult::Exploded
                        }
                    }
                }

                ExplodeResult::Exploded => ExplodeResult::Exploded,

                ExplodeResult::BeginExplode(left_val, right_val) => {
                    if let Some(left_search) = left_sibling.as_mut() {
                        left_search.add_moving_right(left_val);
                    }
                    self.right.add_moving_left(right_val);
                    self.left = NodeValue::Literal(0);
                    ExplodeResult::Exploded
                }
            }
        }
    }

    fn try_split(&mut self) -> bool {
        self.left.try_split() || self.right.try_split()
    }

    fn add_moving_left(&mut self, value: u32) {
        self.left.add_moving_left(value);
    }

    fn add_moving_right(&mut self, value: u32) {
        self.right.add_moving_right(value);
    }

    fn get_magnitude(&self) -> u32 {
        3 * self.left.get_magnitude() + 2 * self.right.get_magnitude()
    }
}

impl fmt::Display for Node {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{},{}]", self.left, self.right)
    }
}

fn parse_node_value(chars: &mut PeekChar) -> NodeValue {
    match chars.peek().unwrap() {
        '[' => {
            assert_eq!(chars.next(), Some('[')); // pop the '['
            let value = NodeValue::Node(Box::new(parse_node(chars)));
            assert_eq!(chars.next(), Some(']')); // pop the ']'
            value
        }
        _ => {
            assert_eq!(chars.peek().unwrap().is_digit(10), true);
            let mut value = 0;
            while chars.peek().unwrap().is_digit(10) {
                value = value * 10 + chars.next().unwrap().to_digit(10).unwrap();
            }
            NodeValue::Literal(value)
        }
    }
}

fn parse_node(chars: &mut PeekChar) -> Node {
    let left = parse_node_value(chars);
    assert_eq!(chars.next(), Some(',')); // pop the ','
    let right = parse_node_value(chars);

    Node {
        left: left,
        right: right,
    }
}

#[aoc_generator(day18)]
fn day18_input(s: &str) -> Vec<NodeValue> {
    s.trim()
        .lines()
        .map(|x| {
            let mut iter = x.chars().peekable();
            parse_node_value(&mut iter)
        })
        .collect()
}

#[aoc(day18, part1)]
fn day18_part1(input: &[NodeValue]) -> u32 {
    let mut previous_value = input[0].clone();
    for v in &input[1..input.len()] {
        let mut node = Node {
            left: previous_value,
            right: v.clone(),
        };

        while match node.try_explode(0, &mut None, &mut None) {
            ExplodeResult::Exploded => true,
            _ => false,
        } || node.try_split()
        {}

        previous_value = NodeValue::Node(Box::new(node.clone()));
    }
    previous_value.get_magnitude()
}

#[aoc(day18, part2)]
fn day18_part2(input: &[NodeValue]) -> u32 {
    let mut max_sum = u32::MIN;
    for i in 0..input.len()-1 {
        for j in (i+1)..input.len() {
            let left = day18_part1(&vec![input[i].clone(),input[j].clone()]);
            if left > max_sum {
                max_sum = left;
            }
            let right = day18_part1(&vec![input[j].clone(),input[i].clone()]);
            if right > max_sum {
                max_sum = right;
            }
        }
    }
    max_sum
}

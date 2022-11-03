use std::collections::HashMap;
use std::mem;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    unreachable!();
} // Two Sum problem

pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let mut reverse = 0;
    let mut temp = x;
    while temp != 0 {
        reverse = (reverse * 10) + (temp % 10);
        temp = temp / 10;
    }
    return reverse == x;
}//Palindrome Number problem

pub fn roman_to_int(s: String) -> i32 {
    let roman_lib: HashMap<char, i32> = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);

    let mut sum = 0;
    let mut prev = 0;

    for char in s.to_uppercase().chars().rev() {
        if let Some(num) = roman_lib.get(&char) {
            if *num < prev {
                sum -= num;
            } else {
                sum += num;
            }
            prev = *num;
        } else {
            println!("Constants not a roman [{}]", char);
        }
    }
    return sum;
}//Roman to Integer problem

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    fn lcp_inplace(mut s1: String, s2: &str) -> String {
        let mut i = 0;
        for (c1, c2) in s1.chars().zip(s2.chars()) {
            if c1 != c2 {
                break;
            }
            i += 1;
        }
        s1.truncate(i);
        s1
    }
    if strs.len() > 0 {
        strs.iter().skip(1).fold(strs[0].clone(), |acc, x| lcp_inplace(acc, &x))
    } else {
        String::from("")
    }
}//Longest Common Prefix problem

fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();

    for c in s.chars() {
        stack.push(match c {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            _ => {
                if Some(c) == stack.pop() {
                    continue;
                } else {
                    return false;
                }
            }
        })
    }

    stack.is_empty()
}//Valid Parentheses problem

pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = None;
    let mut p_next = &mut dummy;

    while list1.is_some() && list2.is_some() {
        let lone = &mut list1;
        let ltwo = &mut list2;
        let l = if lone.as_ref().unwrap().val < ltwo.as_ref().unwrap().val {
            lone
        } else { ltwo };

        mem::swap(p_next, l);
        mem::swap(l, &mut p_next.as_mut().unwrap().next);
        p_next = &mut p_next.as_mut().unwrap().next;
    }

    mem::swap(p_next, if list1.is_none() { &mut list2 } else { &mut list1 });
    dummy
}//Merge Two Sorted Lists problem

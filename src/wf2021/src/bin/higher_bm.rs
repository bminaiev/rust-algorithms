use std::cmp::max;

struct Res {
    ptr: usize,
    order: i32,
}

fn parse(s: &Vec<u8>, from: usize) -> Res {
    let next = *s.get(from).unwrap_or(&0);
    if next == b'(' {
        let left_res = parse(s, from + 1);
        assert_eq!(s[left_res.ptr], b')');
        let after = *s.get(left_res.ptr + 1).unwrap_or(&0);
        if after == b'-' {
            assert_eq!(s[left_res.ptr + 2], b'>');
            let right_res = parse(s, left_res.ptr + 3);
            Res { ptr: right_res.ptr, order: max(left_res.order + 1, right_res.order) }
        } else {
            Res { ptr: left_res.ptr + 1, order: left_res.order }
        }
    } else {
        Res { ptr: from, order: 0 }
    }
}

pub fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let res = parse(&s.into_bytes(), 0);
    println!("{}", res.order);
}

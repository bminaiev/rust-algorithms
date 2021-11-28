use super::lazy_seg_tree3::*;

#[test]
fn max_val() {
    #[derive(Clone, Default, Copy, Debug)]
    struct MaxValNode {
        max_val: i32,
        pos: usize,
    }

    impl LazySegTreeNodeSpec for MaxValNode {
        fn unite(l: &Self, r: &Self) -> Self {
            if l.max_val > r.max_val {
                *l
            } else {
                *r
            }
        }

        fn apply_update(node: &mut Self, update: &Self::Update) {
            node.max_val = *update;
        }

        #[allow(unused)]
        fn join_updates(current: &mut Self::Update, add: &Self::Update) {
            unreachable!()
        }

        type Update = i32;
    }

    let n = 5;
    let mut seg_tree = LazySegTree::new_f(n, &|pos| MaxValNode { max_val: 0, pos });
    seg_tree.modify(2, 3, 123);
    let res = seg_tree.get(0, 5);
    println!("res = {:?}", res);
}

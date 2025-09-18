mod string {
    pub fn find<S: AsRef<str>>(s: S, ns: S) -> Option<usize> {
        let s = s.as_ref();
        let ns = ns.as_ref();
        let ns_len = ns.len();
        let s_len = s.len();
        if ns_len == 0 || s_len == 0 || ns_len > s_len {
            return None;
        }
        let mut left = 0;
        let mut right = 0;
        while left < s_len {
            if s[left..left + 1] != ns[right..right + 1] {
                right = 0;
            } else {
                right += 1;
                if right==ns_len{
                    return Some(left-ns_len+1);
                }
            }
            left += 1;
        }
        None
    }
}
#[test]
fn find_str() {
    use crate::utils::string::find;

    let s = "Hello World";
    let ns = "llo";
    let res = find(s, ns);
    assert_eq!(Some(2_usize), res);
    let s = "(* (min 3,4) 2) ";
    let ns = "min";
    let res = find(s, ns);
    assert_eq!(Some(4_usize), res);
      let s = "(* (min 3,4) 2) ";
    let ns = "mine";
    let res = find(s, ns);
    assert_eq!(None, res);
}

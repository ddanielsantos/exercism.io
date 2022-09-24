#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        return Comparison::Equal;
    }

    let f_len = _first_list.len();
    let s_len = _second_list.len();

    if f_len == 0 && s_len > 0 {
        return Comparison::Sublist;
    } else if s_len == 0 && f_len > 0 {
        return Comparison::Superlist;
    }

    if _first_list
        .iter()
        .all(|f_value| _second_list.contains(f_value))
    {
        // println!("sub");
        return Comparison::Sublist;
    }

    if _second_list
        .iter()
        .all(|s_value| _first_list.contains(s_value))
    {
        // println!("sup");
        return Comparison::Superlist;
    }

    Comparison::Unequal
}

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let first_len = _first_list.len();
    let second_len =  _second_list.len();
    if first_len == second_len { 
        // check equal
        let mut i = 0;
        while i < first_len {
            let first_value = _first_list.get(i);
            let second_value = _second_list.get(i);
            if first_value != second_value {
                return Comparison::Unequal;
            }
            i += 1;
        }
        return Comparison::Equal;
    } else if first_len < second_len {
        if check_array_inclusion(_second_list, _first_list) { 
            return Comparison::Sublist 
        } else { 
            return Comparison::Unequal 
        };
    } else {
        if check_array_inclusion(_first_list, _second_list) { 
            return Comparison::Superlist 
        } else { 
            return Comparison::Unequal 
        }
    }
}


pub fn check_array_inclusion<T: PartialEq>(_inclusive_list: &[T], _included_list: &[T]) -> bool {
    let _included_list_len = _included_list.len();
    if _included_list_len == 0 {
        return true; // everything includes an empty list
    }
    // check sublist
    let mut i = 0;
    let mut j = 0;
    let mut j_offset = 0;
    while j_offset < _included_list_len {
        let first_value = _included_list.get(i);
        let second_value = _inclusive_list.get(j);
        if first_value == second_value {
            if i == _included_list_len - 1 { // end
                return true;
            } else {
                i += 1;
                j += 1;
            }
        } else {
            i = 0;
            j_offset += 1;
            j = j_offset;
        }
    }
    return false;
}
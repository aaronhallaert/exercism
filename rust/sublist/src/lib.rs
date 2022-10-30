#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    //"Determine if the first list is equal to, sublist of, superlist of or unequal to the second list."

    let mut second_list_vec = _second_list.to_vec();

    let mut number_removed: usize = 0;

    // iterate over _second_list
    for i in _first_list {

        if second_list_vec.contains(i) {
            // remove the element from the first list
            second_list_vec.retain(|x| x != i);
            number_removed += 1;
        } else {
            // if the element is not in the first list, then the first list is not a sublist of the second list
            return Comparison::Unequal;
        }
    }

    if second_list_vec.is_empty() {
        if number_removed == _first_list.len() {
            return Comparison::Equal
        }
        else {
            return Comparison::Sublist
        }
    } 
    else {
    }
}

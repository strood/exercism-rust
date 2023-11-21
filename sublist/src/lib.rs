#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
	Equal,
	Sublist,
	Superlist,
	Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
  let first_len = _first_list.len();
  let second_len = _second_list.len();
  if first_len == 0 && second_len == 0 {
    return Comparison::Equal;
  }

  if first_len == 0 {
    return Comparison::Sublist;
  }
  if second_len == 0 {
    return Comparison::Superlist;
  }

  if first_len == second_len {
    return match _first_list == _second_list {
      true => Comparison::Equal,
      false => Comparison::Unequal,
    };
  };

  let mut count = 0;
  match first_len < second_len {
    true => {
      while count <= second_len - first_len {
        if _first_list == &_second_list[count..count + first_len] {
          return Comparison::Sublist;
        } else {
          count += 1;
        }
      }
    }
    false => {
      while count <= first_len - second_len {
        if _second_list == &_first_list[count..count + second_len] {
          return Comparison::Superlist;
        } else {
          count += 1;
        }
      }
    }
  }

  Comparison::Unequal
}

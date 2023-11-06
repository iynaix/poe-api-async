use async_graphql::{Enum, InputType};

#[derive(Enum, Debug, Copy, Clone, PartialEq, Eq)]
pub enum Orderby {
    Asc,
    Desc,
}

pub trait OrderbyInput
where
    Self: Sized + InputType,
{
    type Output: Clone;

    /// sorts the vec by the list of OrderbyInputs
    fn orderby(arr: &mut Vec<Self::Output>, orders: Vec<Self>) -> Vec<Self::Output> {
        arr.sort_by(|a, b| {
            for order in &orders {
                match order.cmp_orderby(a, b) {
                    std::cmp::Ordering::Equal => continue,
                    other => return other,
                }
            }
            std::cmp::Ordering::Equal
        });
        arr.to_vec()
    }

    /// cmp for purposes of orderby from graphql input
    fn cmp_orderby(&self, a: &Self::Output, b: &Self::Output) -> std::cmp::Ordering;
}

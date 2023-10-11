use async_graphql::{Enum, InputType, Value};

#[derive(Enum, Debug, Copy, Clone, PartialEq, Eq)]
pub enum Orderby {
    Asc,
    Desc,
}

/// used for parsing the orderby graphql argument while maintaining order
pub type OrderbyPair = (String, Orderby);

pub trait OrderbyInput
where
    Self: Sized + InputType,
{
    type Output: Clone;

    // TODO: fix for ordered fields?
    fn to_orderby_vec(&self) -> Vec<Self> {
        if let Value::Object(value) = self.to_value() {
            let orderby_pairs: Vec<OrderbyPair> = value
                .iter()
                .filter_map(|(orderby_name, orderby_value)| {
                    if let Value::Enum(v) = orderby_value {
                        Some((
                            orderby_name.to_string(),
                            match v.as_str() {
                                "DESC" => Orderby::Desc,
                                _ => Orderby::Asc,
                            },
                        ))
                    } else {
                        None
                    }
                })
                .collect();
            Self::from_orderbypairs(orderby_pairs);
        }

        Vec::new()
    }

    fn from_orderbypairs(orderby_vec: Vec<OrderbyPair>) -> Vec<Self>;

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

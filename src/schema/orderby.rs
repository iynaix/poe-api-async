use async_graphql::Enum;

#[derive(Enum, Debug, Copy, Clone, PartialEq, Eq)]
pub enum Orderby {
    Asc,
    Desc,
}

/// used for parsing the orderby graphql argument while maintaining order
pub type OrderbyPair = (String, Orderby);

pub trait OrderbyInput
where
    Self: Sized,
{
    type Output: Clone;

    // TODO: implement for async_graphql
    fn to_orderby_vec(&self) -> Vec<Self> {
        // if let Some(order) = &self {
        //     if let Value::Object(value) = order.to_value() {
        //         value.iter().for_each(|(k, v)| {
        //             println!("{}: {:?}", k, v);
        //         });
        //     }
        // };

        Vec::new()
    }

    /*
    fn from_executor<__S: ScalarValue>(executor: &Executor<(), __S>) -> Vec<Self> {
        let look_ahead = executor.look_ahead();

        if let Some(orderby_arg) = look_ahead.argument("orderby") {
            if let LookAheadValue::Object(orderby_arg) = orderby_arg.value() {
                let orderby_pairs: Vec<_> = orderby_arg
                    .iter()
                    .filter_map(|(orderby_name, orderby_value)| {
                        if let LookAheadValue::Enum(orderby_value) = orderby_value {
                            Some((
                                orderby_name.to_string(),
                                match *orderby_value {
                                    "DESC" => Orderby::Desc,
                                    _ => Orderby::Asc,
                                },
                            ))
                        } else {
                            None
                        }
                    })
                    .collect();
                return Self::from_orderbypairs(orderby_pairs);
            } else {
                return Vec::new();
            }
        }

        Vec::new()
    }
    */

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

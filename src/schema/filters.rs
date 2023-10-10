use async_graphql::InputObject;
use regex::Regex;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

use super::{
    ninja_currency::CurrencyEndpoint,
    ninja_item::{ItemEndpoint, Modifier},
};

#[derive(Debug, InputObject)]
pub struct StringFilter {
    pub _eq: Option<String>,
    pub _ieq: Option<String>,
    pub _ne: Option<String>,
    pub _ine: Option<String>,
    pub _contains: Option<String>,
    pub _icontains: Option<String>,
    pub _startswith: Option<String>,
    pub _istartswith: Option<String>,
    pub _endswith: Option<String>,
    pub _iendswith: Option<String>,
    pub _regex: Option<String>,
    pub _iregex: Option<String>,
    pub _in: Option<Vec<String>>,
    pub _nin: Option<Vec<String>>,
}

#[derive(Debug, InputObject)]
pub struct BooleanFilter {
    pub _eq: Option<bool>,
    pub _ne: Option<bool>,
}

#[derive(Debug, InputObject)]
pub struct IntFilter {
    pub _eq: Option<i32>,
    pub _ne: Option<i32>,
    pub _gt: Option<i32>,
    pub _gte: Option<i32>,
    pub _lt: Option<i32>,
    pub _lte: Option<i32>,
    pub _in: Option<Vec<i32>>,
    pub _nin: Option<Vec<i32>>,
}

#[derive(Debug, InputObject)]
pub struct FloatFilter {
    pub _eq: Option<f64>,
    pub _ne: Option<f64>,
    pub _gt: Option<f64>,
    pub _gte: Option<f64>,
    pub _lt: Option<f64>,
    pub _lte: Option<f64>,
    pub _in: Option<Vec<f64>>,
    pub _nin: Option<Vec<f64>>,
}

// same as StringFilter
#[derive(Debug, InputObject)]
pub struct ModifierFilter {
    pub _eq: Option<String>,
    pub _ieq: Option<String>,
    pub _ne: Option<String>,
    pub _ine: Option<String>,
    pub _contains: Option<String>,
    pub _icontains: Option<String>,
    pub _startswith: Option<String>,
    pub _istartswith: Option<String>,
    pub _endswith: Option<String>,
    pub _iendswith: Option<String>,
    pub _regex: Option<String>,
    pub _iregex: Option<String>,
    pub _in: Option<Vec<String>>,
    pub _nin: Option<Vec<String>>,
}

pub trait FilterInput {
    type Item;

    fn filter_fn(&self, value: Self::Item) -> bool;
}

impl FilterInput for BooleanFilter {
    type Item = bool;

    fn filter_fn(&self, s: Self::Item) -> bool {
        match &self {
            Self { _eq: Some(v), .. } if &s != v => false,
            Self { _ne: Some(v), .. } if &s == v => false,
            _ => true,
        }
    }
}

impl FilterInput for StringFilter {
    type Item = String;

    fn filter_fn(&self, s: Self::Item) -> bool {
        let sl = s.to_lowercase();

        // note if statements are for the failure case
        match self {
            Self { _eq: Some(v), .. } if &s != v => false,
            Self { _ieq: Some(v), .. } if sl != v.to_lowercase() => false,
            Self { _ne: Some(v), .. } if &s == v => false,
            Self { _ine: Some(v), .. } if sl == v.to_lowercase() => false,
            Self {
                _contains: Some(v), ..
            } if !s.contains(v) => false,
            Self {
                _icontains: Some(v),
                ..
            } if !sl.contains(&v.to_lowercase()) => false,
            Self {
                _startswith: Some(v),
                ..
            } if !s.starts_with(v) => false,
            Self {
                _istartswith: Some(v),
                ..
            } if !sl.starts_with(&v.to_lowercase()) => false,
            Self {
                _endswith: Some(v), ..
            } if !s.ends_with(v) => false,
            Self {
                _iendswith: Some(v),
                ..
            } if !sl.ends_with(v) => false,
            Self {
                _regex: Some(v), ..
            } if !Regex::new(v).unwrap().is_match(&s) => false,
            Self {
                _iregex: Some(v), ..
            } if !Regex::new(&format!("(?i){}", v)).unwrap().is_match(&s) => false,
            Self { _in: Some(v), .. } if !v.contains(&s) => false,
            Self { _nin: Some(v), .. } if v.contains(&s) => false,
            _ => true,
        }
    }
}

impl FilterInput for IntFilter {
    type Item = i32;

    fn filter_fn(&self, s: Self::Item) -> bool {
        match &self {
            Self { _eq: Some(v), .. } if &s != v => false,
            Self { _ne: Some(v), .. } if &s == v => false,
            Self { _gt: Some(v), .. } if s <= *v => false,
            Self { _gte: Some(v), .. } if s < *v => false,
            Self { _lt: Some(v), .. } if s >= *v => false,
            Self { _lte: Some(v), .. } if s > *v => false,
            Self { _in: Some(v), .. } if !v.contains(&s) => false,
            Self { _nin: Some(v), .. } if v.contains(&s) => false,
            _ => true,
        }
    }
}

impl FilterInput for FloatFilter {
    type Item = f64;

    fn filter_fn(&self, s: Self::Item) -> bool {
        match &self {
            Self { _eq: Some(v), .. } if &s != v => false,
            Self { _ne: Some(v), .. } if &s == v => false,
            Self { _gt: Some(v), .. } if s <= *v => false,
            Self { _gte: Some(v), .. } if s < *v => false,
            Self { _lt: Some(v), .. } if s >= *v => false,
            Self { _lte: Some(v), .. } if s > *v => false,
            Self { _in: Some(v), .. } if !v.contains(&s) => false,
            Self { _nin: Some(v), .. } if v.contains(&s) => false,
            _ => true,
        }
    }
}

impl FilterInput for ModifierFilter {
    type Item = Modifier;

    fn filter_fn(&self, s: Self::Item) -> bool {
        let string_fitler = StringFilter {
            _eq: self._eq.clone(),
            _ieq: self._ieq.clone(),
            _ne: self._ne.clone(),
            _ine: self._ine.clone(),
            _contains: self._contains.clone(),
            _icontains: self._icontains.clone(),
            _startswith: self._startswith.clone(),
            _istartswith: self._istartswith.clone(),
            _endswith: self._endswith.clone(),
            _iendswith: self._iendswith.clone(),
            _regex: self._regex.clone(),
            _iregex: self._iregex.clone(),
            _in: self._in.clone(),
            _nin: self._nin.clone(),
        };

        string_fitler.filter_fn(s.text)
    }
}

pub trait WhereInput
where
    Self: Sized,
{
    type Output: Clone + Hash + Eq;

    // boilerplate required to access where and, or, not struct fields
    fn and(&self) -> Option<&Vec<Self>>;
    fn or(&self) -> Option<&Vec<Self>>;
    fn not(&self) -> Option<&Vec<Self>>;

    fn filter(&self, arr: Vec<Self::Output>) -> Vec<Self::Output>;

    fn filter_recursive(&self, arr: &[Self::Output]) -> Vec<Self::Output> {
        let mut filtered = self.filter(arr.to_vec());

        if let Some(and) = self.and() {
            filtered = and
                .iter()
                .fold(filtered, |acc, inner| inner.filter_recursive(&acc))
        }

        if let Some(or) = self.or() {
            filtered = or
                .iter()
                .fold(HashSet::new(), |mut acc, inner| {
                    acc.extend(inner.filter_recursive(&filtered));
                    acc
                })
                .into_iter()
                .collect();
        }

        if let Some(not) = self.not() {
            let all_matching = not.iter().fold(HashSet::new(), |mut acc, inner| {
                acc.extend(inner.filter_recursive(&filtered));
                acc
            });
            let all_matching: HashSet<_> = all_matching.iter().collect();

            filtered.retain(|item| !all_matching.contains(item))
        }

        filtered
    }
}

#[derive(Debug, InputObject)]
pub struct ItemEndpointFilter {
    _eq: Option<ItemEndpoint>,
    _ne: Option<ItemEndpoint>,
    _in: Option<Vec<ItemEndpoint>>,
    _nin: Option<Vec<ItemEndpoint>>,
}

impl FilterInput for ItemEndpointFilter {
    type Item = ItemEndpoint;

    fn filter_fn(&self, s: Self::Item) -> bool {
        match &self {
            Self { _eq: Some(v), .. } if &s != v => false,
            Self { _ne: Some(v), .. } if &s == v => false,
            Self { _in: Some(v), .. } if !v.contains(&s) => false,
            Self { _nin: Some(v), .. } if v.contains(&s) => false,
            _ => true,
        }
    }
}

#[derive(Debug, InputObject)]
pub struct CurrencyEndpointFilter {
    _eq: Option<CurrencyEndpoint>,
    _ne: Option<CurrencyEndpoint>,
    _in: Option<Vec<CurrencyEndpoint>>,
    _nin: Option<Vec<CurrencyEndpoint>>,
}

impl FilterInput for CurrencyEndpointFilter {
    type Item = CurrencyEndpoint;

    fn filter_fn(&self, s: Self::Item) -> bool {
        match &self {
            Self { _eq: Some(v), .. } if &s != v => false,
            Self { _ne: Some(v), .. } if &s == v => false,
            Self { _in: Some(v), .. } if !v.contains(&s) => false,
            Self { _nin: Some(v), .. } if v.contains(&s) => false,
            _ => true,
        }
    }
}

use std::{collections::HashMap, str::FromStr};

use anyhow::anyhow;
use diff::{Diff, OptionDiff};
use helpers::to_monitor_name;
use tokio::sync::RwLock;
use types::{traits::Busy, Build};

#[macro_export]
macro_rules! response {
    ($x:expr) => {
        Ok::<_, (axum::http::StatusCode, String)>($x)
    };
}

pub fn option_diff_is_some<T: Diff>(diff: &OptionDiff<T>) -> bool
where
    <T as Diff>::Repr: PartialEq,
{
    diff != &OptionDiff::NoChange && diff != &OptionDiff::None
}

pub fn any_option_diff_is_some<T: Diff>(diffs: &[&OptionDiff<T>]) -> bool
where
    <T as Diff>::Repr: PartialEq,
{
    for diff in diffs {
        if diff != &&OptionDiff::NoChange && diff != &&OptionDiff::None {
            return true;
        }
    }
    false
}

pub fn parse_comma_seperated_list<T: FromStr>(comma_sep_list: &str) -> anyhow::Result<Vec<T>> {
    comma_sep_list
        .split(',')
        .filter(|item| !item.is_empty())
        .map(|item| {
            let item = item
                .parse()
                .map_err(|_| anyhow!("error parsing string {item} into type T"))?;
            Ok::<T, anyhow::Error>(item)
        })
        .collect()
}

pub fn get_image_name(build: &Build) -> String {
    let name = to_monitor_name(&build.name);
    match &build.docker_organization {
        Some(org) => format!("{org}/{name}"),
        None => match &build.docker_account {
            Some(acct) => format!("{acct}/{name}"),
            None => name,
        },
    }
}

pub fn empty_or_only_spaces(word: &str) -> bool {
    if word.is_empty() {
        return true;
    }
    for char in word.chars() {
        if char != ' ' {
            return false;
        }
    }
    true
}

#[derive(Default)]
pub struct Cache<T: Clone + Default> {
    cache: RwLock<HashMap<String, T>>,
}

impl<T: Clone + Default> Cache<T> {
    pub async fn get(&self, key: &str) -> Option<T> {
        self.cache.read().await.get(key).cloned()
    }

    pub async fn get_or_default(&self, key: String) -> T {
        let mut cache = self.cache.write().await;
        cache.entry(key).or_default().clone()
    }

    pub async fn _get_list(&self, filter: Option<impl Fn(&String, &T) -> bool>) -> Vec<T> {
        let cache = self.cache.read().await;
        match filter {
            Some(filter) => cache
                .iter()
                .filter(|(k, v)| filter(k, v))
                .map(|(_, e)| e.clone())
                .collect(),
            None => cache.iter().map(|(_, e)| e.clone()).collect(),
        }
    }

    pub async fn insert(&self, key: String, val: T) {
        self.cache.write().await.insert(key, val);
    }

    pub async fn update_entry(&self, key: String, handler: impl Fn(&mut T)) {
        let mut cache = self.cache.write().await;
        handler(cache.entry(key).or_default());
    }

    pub async fn clear(&self) {
        self.cache.write().await.clear();
    }

    pub async fn remove(&self, key: &str) {
        self.cache.write().await.remove(key);
    }
}

impl<T: Clone + Default + Busy> Cache<T> {
    pub async fn busy(&self, id: &str) -> bool {
        match self.get(id).await {
            Some(state) => state.busy(),
            None => false,
        }
    }
}

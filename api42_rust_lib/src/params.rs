/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   params.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: dpotsch <poetschdavid@gmail.com>           +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/06/29 17:44:57 by dpotsch           #+#    #+#             */
/*   Updated: 2025/06/29 20:19:34 by dpotsch          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::collections::HashMap;
use std::fmt;

#[derive(Clone)]
pub struct Params {
  data: HashMap<String, String>,
}

impl Params {
  pub fn new(items: &[(&str, &str)]) -> Self {
    let data = items
      .iter()
      .map(|(k, v)| (k.to_string(), v.to_string()))
      .collect();
    Self { data }
  }

  pub fn empty() -> Self {
    Self {
      data: HashMap::new(),
    }
  }

  pub fn default() -> Self {
    let mut new_params = Self {
      data: HashMap::new(),
    };

    new_params.add("filter[campus]", "53");
    new_params
  }

  pub fn as_query(&self) -> Vec<(&str, &str)> {
    self
      .data
      .iter()
      .map(|(k, v)| (k.as_str(), v.as_str()))
      .collect()
  }

  pub fn add(&mut self, key: &str, value: &str) {
    self.data.insert(key.to_string(), value.to_string());
  }
}

impl fmt::Display for Params {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let param_string = self
      .data
      .iter()
      .map(|(k, v)| format!("{}={}", k, v))
      .collect::<Vec<_>>()
      .join("&");
    write!(f, "{}", param_string)
  }
}

impl fmt::Debug for Params {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let param_string = self
      .data
      .iter()
      .map(|(k, v)| format!("{}={}", k, v))
      .collect::<Vec<_>>()
      .join("&");
    write!(f, "{}", param_string)
  }
}

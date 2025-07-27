/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   utils.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: dpotsch <poetschdavid@gmail.com>           +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/07/23 10:41:38 by dpotsch           #+#    #+#             */
/*   Updated: 2025/07/27 12:15:49 by dpotsch          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use serde_json::Value;
use std::fs::File;
use std::io::Write;

/// ### Should turn the json pages array into one json value
pub fn flatten_json_arrays(data: &[Value]) -> Value {
  let mut combined = Vec::new();

  for val in data {
    if let Value::Array(items) = val {
      combined.extend(items.clone());
    }
  }
  Value::Array(combined)
}

pub fn write_json_to_file(file_name: &str, data: &Value) {
  let file_raw = File::create(file_name);
  if file_raw.is_err() {
    return;
  }
  let mut file_raw = file_raw.unwrap();

  let str = format!("{}\n", serde_json::to_string_pretty(data).unwrap());
  let res = file_raw.write_all(str.as_bytes());

  if res.is_err() {
    println!("error writing file {}", file_name);
    return;
  }
}

pub fn write_string_to_file(file_name: &str, data: &str) {
  let file_raw = File::create(file_name);
  if file_raw.is_err() {
    return;
  }
  let mut file_raw = file_raw.unwrap();
  let res = file_raw.write_all(data.as_bytes());

  if res.is_err() {
    println!("error writing file {}", file_name);
    return;
  }
}

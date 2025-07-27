/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   json_parsing.rs                                    :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: dpotsch <poetschdavid@gmail.com>           +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/07/24 15:59:59 by dpotsch           #+#    #+#             */
/*   Updated: 2025/07/27 11:56:08 by dpotsch          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use serde_json::Value;

pub fn find_str_entry(entry: &str, item: &Value) -> String {
  let mut current = item;

  for key in entry.split('/') {
    match current.get(key) {
      Some(val) => current = val,
      None => return "unknown".to_string(),
    }
  }
  current.as_str().unwrap_or("unknown").to_string()
}

pub fn find_int_entry(entry: &str, item: &Value) -> i64 {
  let mut current = item;

  for key in entry.split('/') {
    match current.get(key) {
      Some(val) => current = val,
      None => return 0,
    }
  }
  current.as_i64().unwrap_or(0)
}

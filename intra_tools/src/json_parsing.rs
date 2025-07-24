/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   json_parsing.rs                                    :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: dpotsch <poetschdavid@gmail.com>           +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/07/24 15:59:59 by dpotsch           #+#    #+#             */
/*   Updated: 2025/07/24 16:40:43 by dpotsch          ###   ########.fr       */
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

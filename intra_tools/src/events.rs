/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   events.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: dpotsch <poetschdavid@gmail.com>           +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/06/28 18:25:18 by dpotsch           #+#    #+#             */
/*   Updated: 2025/07/27 12:01:48 by dpotsch          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use api42_rust_lib::{params::Params, Intra42Api};
use serde_json::Value;

use crate::{
  json_parsing::{find_int_entry, find_str_entry},
  utils::{write_json_to_file, write_string_to_file},
};

fn parse_json_data(data: &Value) {
  write_json_to_file("raw_events.json", &data);

  let mut result = "".to_string();
  let json_data = data.as_array().unwrap();

  for item in json_data {
    let id = find_int_entry("id", &item);
    let name = find_str_entry("name", &item);
    let begin_at = find_str_entry("begin_at", &item);
    println!("id: {}, {:15} ({})", id, name, begin_at);

    // create string for file
    let res = format!("id: {}, {:15} ({})", id, name, begin_at);
    result.push_str(res.as_str());
    result.push('\n');
  }
  write_string_to_file("events.txt", &result.as_str());
}

pub async fn get_events(api: &mut Intra42Api) {
  let mut params = Params::new(&[
    ("per_page", "100"),
    ("page", "1"),
    (
      "range[begin_at]",
      "2025-07-01T00:00:00.000Z,2025-08-01T00:00:00.000Z",
    ),
  ]);

  let endpoint = format!("campus/{}/events", "53");
  match api.get_data_simple(&endpoint, &mut params).await {
    Ok(data) => {
      parse_json_data(&data);
    }
    Err(e) => println!("Error: {}", e),
  }
}

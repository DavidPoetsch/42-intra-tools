/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   event_users.rs                                     :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: dpotsch <poetschdavid@gmail.com>           +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/06/28 18:25:18 by dpotsch           #+#    #+#             */
/*   Updated: 2025/07/24 16:53:39 by dpotsch          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use api42_rust_lib::{params::Params, Intra42Api};
use serde_json::Value;

use crate::{
  json_parsing::find_str_entry,
  utils::{write_json_to_file, write_string_to_file},
};

fn parse_json_data(data: &Value) {
  write_json_to_file("raw_event_users.json", &data);
  let mut result = "".to_string();
  let json_data = data.as_array().unwrap();

  for item in json_data {
    let first_name = find_str_entry("user/first_name", &item);
    let login = find_str_entry("user/login", &item);
    println!("User first_name: {:15} ({})", first_name, login);

    // create string for file
    let res = format!("User first_name: {:15} ({})", first_name, login);
    result.push_str(res.as_str());
    result.push('\n');
  }
  write_string_to_file("event_users.txt", &result.as_str());
}

pub async fn get_event_users(api: &mut Intra42Api, event_id: u32) {
  let mut params = Params::new(&[
    ("filter[campus_id]", "53"),
    ("per_page", "100"),
    ("page", "1"),
  ]);

  let endpoint = format!("events/{}/events_users", event_id);
  match api.get_data_simple(&endpoint, &mut params).await {
    Ok(data) => {
      parse_json_data(&data);
    }
    Err(e) => println!("Error: {}", e),
  }
}

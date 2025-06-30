/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   event_users.rs                                     :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: dpotsch <poetschdavid@gmail.com>           +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/06/28 18:25:18 by dpotsch           #+#    #+#             */
/*   Updated: 2025/06/29 19:53:20 by dpotsch          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use api42_rust_lib::{params::Params, Intra42Api};
use serde_json::Value;

fn parse_json_data(data: &Value) {
  // println!("{}", serde_json::to_string_pretty(&event).unwrap());
  let json_data = data.as_array().unwrap();
  for item in json_data {
    if let Some(user) = item.get("user") {
      if let Some(login) = user.get("login") {
        println!("User login: {}", login);
      }
    }
  }
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

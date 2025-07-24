/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   projects.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: dpotsch <poetschdavid@gmail.com>           +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/06/28 15:22:08 by dpotsch           #+#    #+#             */
/*   Updated: 2025/07/24 16:59:28 by dpotsch          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use api42_rust_lib::{params::Params, Intra42Api};
use serde_json::Value;

use crate::json_parsing::find_str_entry;

pub fn print_projects_data(data: &[Value]) {
  for _item in data {
    if let Some(array) = _item.as_array() {
      for item in array {
        let name = find_str_entry("name", &item);
        let id = item.get("id").and_then(|n| n.as_i64()).unwrap_or(-1);
        println!("id: {:5}, name: {}", id, name);
      }
    }
  }
}

pub async fn projects_data(api: &mut Intra42Api) {
  let mut params = Params::new(&[]);

  match api
    .get_all_pages(&"cursus/21/projects".to_string(), &mut params)
    .await
  {
    Ok(data_vec) => {
      print_projects_data(&data_vec);
    }
    Err(e) => println!("Error: {}", e),
  }
}

/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   project_user_data.rs                               :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: dpotsch <poetschdavid@gmail.com>           +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/06/28 15:22:08 by dpotsch           #+#    #+#             */
/*   Updated: 2025/07/24 17:05:47 by dpotsch          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::json_parsing::find_str_entry;
use crate::utils::flatten_json_arrays;
use crate::utils::write_json_to_file;
use api42_rust_lib::{params::Params, Intra42Api};
use serde_json::Value;
use std::fs::File;
use std::io::Write;

fn sort_by_project(data: &mut Vec<Value>) {
  data.sort_by(|a, b| {
    let a_project = a
      .get("project")
      .and_then(|i| i.get("name"))
      .and_then(|i| i.as_str())
      .unwrap_or("");
    let b_project = b
      .get("project")
      .and_then(|i| i.get("name"))
      .and_then(|i| i.as_str())
      .unwrap_or("");
    a_project.cmp(b_project)
  });
}

async fn parse_json_data(data: &[Value]) {
  let mut data = flatten_json_arrays(data);
  write_json_to_file("raw_project_user_data.json", &data);

  let file = File::create("project_user_data.txt");
  if file.is_err() {
    return;
  }
  let mut file = file.unwrap();

  if let Some(array) = data.as_array_mut() {
    sort_by_project(array);

    for item in array {
      let first_name = find_str_entry("user/first_name", &item);
      let login = find_str_entry("user/login", &item);
      let project = find_str_entry("project/name", &item);
      let empty_vec = Vec::new();
      let cursus_ids = item
        .get("cursus_ids")
        .and_then(|i| i.as_array())
        .unwrap_or(&empty_vec);

      let target_id = Value::Number(serde_json::Number::from(21));
      if cursus_ids.contains(&target_id) {
        let str = format!(
          "First name: {:25}, Login: {:15}, Project: {:40}\n",
          first_name, login, project
        );
        let res = file.write_all(str.as_bytes());
        if res.is_err() {
          println!("write error");
          return;
        }
      }
    }
  } else {
    println!("error at parsing");
  }
}

/*
  campus 53 = vienna
  cursus 21 = common core
  possible status: creating_group, in_progress, waiting_for_correction
*/

pub async fn project_user_data(api: &mut Intra42Api) {
  let mut params = Params::new(&[
    ("filter[campus]", "53"),
    ("filter[cursus]", "21"),
    ("filter[status]", "in_progress"),
    (
      "range[updated_at]",
      "2025-01-01T00:00:00.000Z,2026-01-01T00:00:00.000Z",
    ),
  ]);

  let endpoint = String::from("projects_users");
  match api.get_all_pages(&endpoint, &mut params).await {
    Ok(data_vec) => {
      parse_json_data(&data_vec).await;
    }
    Err(e) => println!("Error: {}", e),
  }
}

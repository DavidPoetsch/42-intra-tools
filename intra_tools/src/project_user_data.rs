/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   project_user_data.rs                               :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: dpotsch <poetschdavid@gmail.com>           +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/06/28 15:22:08 by dpotsch           #+#    #+#             */
/*   Updated: 2025/06/29 20:08:23 by dpotsch          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

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

fn flatten_json_arrays(data: &[Value]) -> Value {
  let mut combined = Vec::new();

  for val in data {
    if let Value::Array(items) = val {
      combined.extend(items.clone());
    }
  }
  Value::Array(combined)
}

fn write_raw_data(data: &Value) {
  let file_raw = File::create("rawout.json");
  if file_raw.is_err() {
    return;
  }
  let mut file_raw = file_raw.unwrap();

  let str = format!("{}\n", serde_json::to_string_pretty(data).unwrap());
  let res = file_raw.write_all(str.as_bytes());

  if res.is_err() {
    println!("error writing rawout file");
    return;
  }
}

async fn parse_json_data(data: &[Value]) {
  let mut data = flatten_json_arrays(data);
  write_raw_data(&data);

  let file = File::create("out.json");
  if file.is_err() {
    return;
  }
  let mut file = file.unwrap();

  if let Some(array) = data.as_array_mut() {
    sort_by_project(array);

    for item in array {
      let first_name = item
        .get("user")
        .and_then(|u| u.get("first_name"))
        .and_then(|n| n.as_str())
        .unwrap_or("<unknown>");
      let login = item
        .get("user")
        .and_then(|u| u.get("login"))
        .and_then(|n| n.as_str())
        .unwrap_or("<unknown>");
      let project = item
        .get("project")
        .and_then(|i| i.get("name"))
        .and_then(|i| i.as_str())
        .unwrap_or("<unknown>");

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

pub async fn project_user_data(api: &mut Intra42Api) {
  let mut params = Params::new(&[
    ("filter[campus]", "53"),
    ("filter[cursus]", "21"),
    ("filter[status]", "in_progress"),
    (
      "range[updated_at]",
      "2024-01-01T00:00:00.000Z,2026-01-01T00:00:00.000Z",
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

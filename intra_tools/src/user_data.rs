/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   user_data.rs                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: dpotsch <poetschdavid@gmail.com>           +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/07/24 12:34:20 by dpotsch           #+#    #+#             */
/*   Updated: 2025/07/24 15:42:39 by dpotsch          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use api42_rust_lib::{params::Params, Intra42Api};
use crate::utils::{flatten_json_arrays, write_json_to_file};
use serde_json::Value;

async fn parse_json_data(data: &[Value]) {
  let data = flatten_json_arrays(data);
  write_json_to_file("rawout.json", &data);
}

pub async fn user_data(api: &mut Intra42Api)
{
  // let mut params = Params::new(&[
  //   ("filter[campus]", "53"),
  //   ("filter[cursus]", "21"),
  //   (
  //     "range[updated_at]",
  //     "2025-05-01T00:00:00.000Z,2025-07-23T00:00:00.000Z",
  //   ),
  // ]);
  let mut params = Params::empty();
  // params.add("filter[primary_campus_id]", "53");
  // params.add("filter[login]", "dpotsch");
  let endpoint = String::from("users/193289/projects_users");
  match api.get_all_pages(&endpoint, &mut params).await {
    Ok(data_vec) => {
      parse_json_data(&data_vec).await;
    }
    Err(e) => println!("Error: {}", e),
  }
}

/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: dpotsch <poetschdavid@gmail.com>           +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/06/26 19:22:16 by dpotsch           #+#    #+#             */
/*   Updated: 2025/06/30 15:08:00 by dpotsch          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod event_users;
mod project_user_data;
mod projects;

use crate::event_users::get_event_users;
use crate::project_user_data::project_user_data;
use crate::projects::projects_data;
use api42_rust_lib::Intra42Api;
use std::env;

#[tokio::main]
async fn main() {
  let args: Vec<String> = env::args().skip(1).collect();
  let mut api = Intra42Api::new();

  match api.request_access_token().await {
    Ok(()) => println!("Token requested successfully."),
    Err(e) => {
      eprintln!("Error requesting token: {}", e);
      return;
    }
  }

  if args.is_empty() {
    return;
  }

  let arg1 = args.first().unwrap();
  if arg1 == "1" {
    project_user_data(&mut api).await;
  } else if arg1 == "2" {
    get_event_users(&mut api, 32965).await;
  } else if arg1 == "3" {
    projects_data(&mut api).await;
  }
}

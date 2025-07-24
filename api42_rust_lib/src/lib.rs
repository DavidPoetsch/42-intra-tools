/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: dpotsch <poetschdavid@gmail.com>           +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/06/26 19:22:16 by dpotsch           #+#    #+#             */
/*   Updated: 2025/07/24 15:42:51 by dpotsch          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub mod errors;
pub mod params;

use dotenvy::dotenv;
use errors::Intra42ApiError;
use params::Params;
use reqwest::header::HeaderMap;
use reqwest::{Client, Response};
use serde_json::Value;
use std::env;
use std::result::Result;
use tokio::task;
use tokio::time::{Duration, sleep};

pub struct Intra42Api {
  client: Client,
  access_token: Option<String>,
  client_id: Option<String>,
  client_secret: Option<String>,
}

impl Intra42Api {
  pub fn new() -> Self {
    Self {
      client: Client::new(),
      access_token: None,
      client_id: None,
      client_secret: None,
    }
  }

  pub fn read_env_vars(&mut self) -> Result<(), Intra42ApiError> {
    if dotenv().is_err() {
      return Err(Intra42ApiError::DotEnvNotFound);
    }

    self.client_id = match env::var("CLIENT_ID") {
      Ok(val) => Some(val),
      Err(_) => {
        return Err(Intra42ApiError::InvalidClientId);
      }
    };

    self.client_secret = match env::var("CLIENT_SECRET") {
      Ok(val) => Some(val),
      Err(_) => {
        return Err(Intra42ApiError::InvalidClientSecret);
      }
    };

    Ok(())
  }

  async fn get_request(
    client: &Client,
    token: &str,
    uri: &str,
    params: &Params,
  ) -> Result<Response, Intra42ApiError> {
    // println!("uri: {}, params: {}", uri, params);//!for debuging
    // println!("{:?}", params.as_query());

    for attempt in 1..=5 {
      let response = client
        .get(uri)
        .query(&params.as_query())
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e| Intra42ApiError::RequestError(e.to_string()))?;

      if response.status().as_u16() == 429 {
        println!("Rate limited. Attempt {attempt}. Retrying...");
        sleep(Duration::from_secs(2u64)).await;
        continue;
      }

      if !response.status().is_success() {
        return Err(Intra42ApiError::RequestBadStatus(response.status()));
      }

      return Ok(response);
    }
    Err(Intra42ApiError::RequestTooManyRetries)
  }

  async fn post_request(
    client: &Client,
    token: &str,
    uri: &str,
    params: &Params,
  ) -> Result<Response, Intra42ApiError> {
    for attempt in 1..=5 {
      let response = client
        .post(uri)
        .query(&params.as_query())
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e| Intra42ApiError::RequestError(e.to_string()))?;

      if response.status().as_u16() == 429 {
        println!("Rate limited. Attempt {attempt}. Retrying...");
        sleep(Duration::from_secs(2u64)).await;
        continue;
      }

      if !response.status().is_success() {
        return Err(Intra42ApiError::RequestBadStatus(response.status()));
      }

      return Ok(response);
    }
    Err(Intra42ApiError::RequestTooManyRetries)
  }

  /**
   * requests a new token
   */
  pub async fn request_access_token(&mut self) -> Result<(), Intra42ApiError> {
    self.access_token = None;
    if self.client_id.is_none() || self.client_secret.is_none() {
      self.read_env_vars()?;
    }
    // Unwrap should be safe because read_env_vars would return early
    let client_id: &str = self.client_id.as_ref().unwrap();
    let client_secret: &str = self.client_secret.as_ref().unwrap();

    let params = Params::new(&[
      ("client_id", client_id),
      ("client_secret", client_secret),
      ("grant_type", "client_credentials"),
      ("scope", ""),
    ]);

    let response = Self::post_request(
      &self.client,
      &"dummy_token",
      "https://api.intra.42.fr/v2/oauth/token",
      &params,
    )
    .await?;

    let json = Self::response_to_json(response).await?;
    let access_token = json["access_token"]
      .as_str()
      .ok_or(Intra42ApiError::MissingAccessToken)?;

    self.access_token = Some(access_token.to_string());
    Ok(())
  }

  /**
   * used to reuse the requested token
   */
  async fn get_access_token(&mut self) -> Result<String, Intra42ApiError> {
    match &self.access_token {
      Some(token) => Ok(token.clone()),
      None => match self.request_access_token().await {
        Ok(_) => Ok(self.access_token.clone().unwrap()),
        Err(e) => {
          eprintln!("request_access_token failed: {e}");
          return Err(e);
        }
      },
    }
  }

  fn build_uri(endpoint: &str) -> String {
    format!("https://api.intra.42.fr/v2/{}", endpoint)
  }

  async fn response_to_json(
    response: Response,
  ) -> Result<Value, Intra42ApiError> {
    response
      .json::<Value>()
      .await
      .map_err(|_| Intra42ApiError::JsonError)
  }

  /**
   * Check the response header if it contains information about the total items
   * of and endpoint.
   * Calculates how many pages neccessary to get all items
   */
  async fn get_ammount_of_pages(
    &mut self,
    endpoint: &String,
    params: &Params,
  ) -> Result<i32, Intra42ApiError> {
    let access_token = self.get_access_token().await?;
    let uri = Self::build_uri(endpoint);
    let response =
      Self::get_request(&self.client, &access_token, &uri, params).await?;

    let headers: &HeaderMap = response.headers();
    // println!("{:?}", headers);
    // match response.text().await
    // {
    //     Ok(body) => {println!("{}", body)},
    //     Err(_) => todo!(),
    // };
    
    let total = headers
      .get("X-Total")
      .ok_or(Intra42ApiError::FailedToParseXTotal)?
      .to_str()
      .map_err(|_| Intra42ApiError::FailedToParseXTotal)?
      .parse::<i32>()
      .map_err(|_| Intra42ApiError::FailedToParseXTotal)?;

    let per_page = headers
      .get("X-Per-Page")
      .ok_or(Intra42ApiError::FailedToParseXPerPage)?
      .to_str()
      .map_err(|_| Intra42ApiError::FailedToParseXPerPage)?
      .parse::<i32>()
      .map_err(|_| Intra42ApiError::FailedToParseXPerPage)?;

    let total_pages = (total + per_page - 1) / per_page;

    println!("X-Total: {total}, X-Per-Page: {per_page}, = {total_pages} pages");
    Ok(total_pages)
  }

  pub async fn get_all_pages(
    &mut self,
    endpoint: &String,
    params: &mut Params,
  ) -> Result<Vec<Value>, Intra42ApiError> {
    let token = self.get_access_token().await?;
    params.add("page", "1");
    params.add("per_page", "100");
    let total_pages = self.get_ammount_of_pages(endpoint, params).await?;

    // println!("total pages {}", total_pages);
    let mut data = Vec::new();
    let mut page = 1;
    let mut pages_done = 0;
    while page <= total_pages {
      let mut tasks = Vec::new();

      // Spawn up to 5 tasks
      for _ in 0..5 {
        if page > total_pages {
          break;
        }

        let mut owned_params = params.clone();
        owned_params.add("page", page.to_string().as_str());
        let client_clone = self.client.clone();
        let token_clone = token.clone();
        let endpoint_clone = Self::build_uri(endpoint);

        let task = task::spawn(async move {
          let response = Self::get_request(
            &client_clone,
            &token_clone,
            &endpoint_clone,
            &owned_params,
          )
          .await?;
          Self::response_to_json(response).await
        });

        tasks.push(task);
        page += 1;
      }

      // Wait for the current batch of tasks
      let results = futures::future::join_all(tasks).await;

      for result in results {
        pages_done += 1;
        match result {
          Ok(Ok(value)) => data.push(value),
          Ok(Err(e)) => println!("Request error: {}", e),
          Err(join_err) => println!("Join error: {}", join_err),
        }
      }
      println!("{}/{} pages done", pages_done, total_pages);
    }
    Ok(data)
  }

  pub async fn get_data_simple(
    &mut self,
    endpoint: &str,
    params: &mut Params,
  ) -> Result<Value, Intra42ApiError> {
    let access_token = self.get_access_token().await?;
    let uri = Self::build_uri(endpoint);

    let response =
      Self::get_request(&self.client, &access_token, &uri, params).await?;

    Self::response_to_json(response).await
  }
}

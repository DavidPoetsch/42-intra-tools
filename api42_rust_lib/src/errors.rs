/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   errors.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: dpotsch <poetschdavid@gmail.com>           +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/06/29 15:00:19 by dpotsch           #+#    #+#             */
/*   Updated: 2025/06/29 17:07:32 by dpotsch          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::fmt;

pub enum Intra42ApiError {
  DotEnvNotFound,
  InvalidClientId,
  InvalidClientSecret,
  MissingAccessToken,
  RequestError(String),
  RequestBadStatus(reqwest::StatusCode),
  RequestTooManyRetries,
  JsonError,
  FailedToParseXTotal,
  FailedToParseXPerPage,
}

impl fmt::Display for Intra42ApiError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::DotEnvNotFound => write!(f, "failed to parse .env file"),
      Self::InvalidClientId => {
        write!(f, "failed to parse CLIENT_ID from .env file")
      }
      Self::InvalidClientSecret => {
        write!(f, "failed to parse CLIENT_SECRET from .env file")
      }
      Self::RequestError(e) => write!(f, "request failed {}", e),
      Self::RequestBadStatus(status) => {
        write!(f, "HTTP request returned bad status: {}", status)
      }
      Self::RequestTooManyRetries => {
        write!(f, "too many retries after 429 rate limit")
      }
      Self::MissingAccessToken => write!(f, "missing access_token in response"),
      Self::JsonError => write!(f, "failed to convert response into json"),
      Self::FailedToParseXTotal => {
        write!(
          f,
          "failed to get ammount of pages, X-Total not found in header"
        )
      }
      Self::FailedToParseXPerPage => write!(
        f,
        "failed to get ammount of pages, X-Per-Page not found in header"
      ),
    }
  }
}

impl fmt::Debug for Intra42ApiError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::DotEnvNotFound => write!(f, "failed to parse .env file"),
      Self::InvalidClientId => {
        write!(f, "failed to parse CLIENT_ID from .env file")
      }
      Self::InvalidClientSecret => {
        write!(f, "failed to parse CLIENT_SECRET from .env file")
      }
      Self::RequestError(e) => write!(f, "request failed {}", e),
      Self::RequestBadStatus(status) => {
        write!(f, "HTTP request returned bad status: {}", status)
      }
      Self::RequestTooManyRetries => {
        write!(f, "too many retries after 429 rate limit")
      }
      Self::MissingAccessToken => write!(f, "missing access_token in response"),
      Self::JsonError => write!(f, "failed to convert response into json"),
      Self::FailedToParseXTotal => {
        write!(
          f,
          "failed to get ammount of pages, X-Total not found in header"
        )
      }
      Self::FailedToParseXPerPage => write!(
        f,
        "failed to get ammount of pages, X-Per-Page not found in header"
      ),
    }
  }
}

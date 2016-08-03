// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use hyper::client::Client as HyperClient;
use std::io::Read;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use rpc::{ClientTransport, Context};
use rpc::{CodecBase, Codec, Message};

#[derive(Clone)]
pub struct HttpClientTransport {
    client: Arc<HyperClient>,
    url: String,
    current_id: Arc<AtomicU64>
}

impl Default for HttpClientTransport {
    fn default() -> HttpClientTransport {
        HttpClientTransport {
            client: Arc::new(HyperClient::new()),
            url: "127.0.0.1:8000".to_string(),
            current_id: Arc::new(AtomicU64::new(1)),
        }
    }
}

impl ClientTransport for HttpClientTransport {
    fn new(url: String) -> HttpClientTransport {
        use std::time::Duration;
        let mut client = HyperClient::new();
        client.set_read_timeout(Some(Duration::new(2, 0)));
        client.set_write_timeout(Some(Duration::new(2,0)));
        HttpClientTransport {
            client: Arc::new(client),
            url: url.clone(),
            current_id: Arc::new(AtomicU64::new(1)),
        }
    }

    fn call<Request, Response, C>(&self,ctx: &Context, endpoint: &str, req: &Request)
        -> Result<Response, String>
        where C: CodecBase + Codec<Request> + Codec<Response>,
        Request: Clone, Response: Clone {
        let id = self.current_id.clone().fetch_add(1, Ordering::SeqCst);

        let codec = C::default();
        let message = match <C as Codec<Request>>::encode_message(&codec, req, endpoint, id) {
            Ok(m) => m,
            Err(_) => unreachable!()
        };
        let cc = self.client.clone();
        let mut res = cc.post(&self.url)
            .header(codec.content_type())
            .body(&message)
            .send();

        match res {
            Ok(ref mut ok_res) => {
                let mut s = String::new();
                let _ = ok_res.read_to_string(&mut s);
                info!("response: {}", s);
                let concrete: &Response = match <C as Codec<Response>>::decode_message(&codec, &s) {
                    Ok(concrete) => unsafe {::std::mem::transmute(concrete.get_body())},
                    Err(e) => return Err(e)
                };
                return Ok((*concrete).clone());
            },
            Err(e) => {
                return Err(format!("{}", e));
            },
        }
    }
}

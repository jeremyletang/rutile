// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use hyper::client::Client as HyperClient;
use hyper::header::{Headers, ContentType, ContentLength};
use std::io::Read;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use rpc::{ClientTransport, Context};
use rpc::{CodecBase, Codec, Message};

#[derive(Clone)]
pub struct HttpClient {
    client: Arc<HyperClient>,
    url: String,
    current_id: Arc<AtomicU64>
}

impl Default for HttpClient {
    fn default() -> HttpClient {
        HttpClient {
            client: Arc::new(HyperClient::new()),
            url: "127.0.0.1:8000".to_string(),
            current_id: Arc::new(AtomicU64::new(1)),
        }
    }
}

impl ClientTransport for HttpClient {
    fn new(url: String) -> HttpClient {
        use std::time::Duration;
        let mut client = HyperClient::new();
        client.set_read_timeout(Some(Duration::new(2, 0)));
        client.set_write_timeout(Some(Duration::new(2,0)));
        HttpClient {
            client: Arc::new(client),
            url: url.clone(),
            current_id: Arc::new(AtomicU64::new(1)),
        }
    }

    fn call<Request, Response, C>(&self, ctx: Context, endpoint: &str, req: &Request)
        -> Result<Response, String>
        where C: CodecBase + Codec<Request> + Codec<Response>,
        Request: Clone, Response: Clone {
        let id = self.current_id.clone().fetch_add(1, Ordering::SeqCst);

        let codec = C::default();
        let message = match <C as Codec<Request>>::encode(&codec, req, endpoint, id) {
            Ok(m) => m,
            Err(e) => {
                println!("error from encode: {}", e);
                unreachable!()
            }
        };

        // set headers from ctx.metas
        let mut hds = Headers::new();
        for (ref k, ref v) in ctx.metas {
            hds.set_raw(k.clone(), vec![v.clone().into_bytes()])
        }

        // create the client
        let cc = self.client.clone();
        let mut res = cc.post(&self.url)
            .headers(hds)
            .header(ContentType(codec.content_type()))
            .header(ContentLength(message.len() as u64))
            .body(&*message)
            .send();

        match res {
            Ok(ref mut ok_res) => {
                let mut buf = Vec::new();
                let _ = ok_res.read_to_end(&mut buf);
                // info!("response: {}", buf);
                let concrete: &Response = match <C as Codec<Response>>::decode(&codec, &buf) {
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

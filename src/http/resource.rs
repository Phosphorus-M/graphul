use async_trait::async_trait;
use axum::response::{Response, IntoResponse};
use hyper::StatusCode;

use crate::http::Request;

/*

take from : https://rust-lang.github.io/async-book/07_workarounds/05_async_in_traits.html
async in Traits
Currently, async fn cannot be used in traits. The reasons for this are somewhat complex, but there are plans to remove this restriction in the future.

In the meantime, however, this can be worked around using the async-trait crate from crates.io.

Note that using these trait methods will result in a heap allocation per-function-call.
This is not a significant cost for the vast majority of applications, but should be considered
when deciding whether to use this functionality in the public API of a low-level function that is
expected to be called millions of times a second.

*/

#[async_trait]
pub trait Resource {

    async fn get(_request: Request) -> Response {
        (StatusCode::NOT_IMPLEMENTED, "Method Not Allowed").into_response()
    }

    async fn post(_request: Request) -> Response {
        (StatusCode::NOT_IMPLEMENTED, "Method Not Allowed").into_response()
    }

    async fn put(_request: Request) -> Response {
        (StatusCode::NOT_IMPLEMENTED, "Method Not Allowed").into_response()
    }

    async fn delete(_request: Request) -> Response {
        (StatusCode::NOT_IMPLEMENTED, "Method Not Allowed").into_response()
    }

    async fn patch(_request: Request) -> Response {
        (StatusCode::NOT_IMPLEMENTED, "Method Not Allowed").into_response()
    }

    async fn options(_request: Request) -> Response {
        (StatusCode::NOT_IMPLEMENTED, "Method Not Allowed").into_response()
    }

    async fn trace(_request: Request) -> Response {
        (StatusCode::NOT_IMPLEMENTED, "Method Not Allowed").into_response()
    }

    async fn head(_request: Request) -> Response {
        (StatusCode::NOT_IMPLEMENTED, "Method Not Allowed").into_response()
    }
}

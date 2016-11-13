mod index;

use router::Router;

pub fn build() -> Router {
    router!{
        index: get "/" => index::handle
    }
}
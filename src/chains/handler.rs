#![allow(dead_code)]
#![allow(non_snake_case)]

use crate::chains::chains::chains;

// one chain handler to rule them all
pub struct ChainHandler {
    Chain: chains, // gotta be a valid chain
}

pub struct HTTPHandler {}

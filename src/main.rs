use mini_redis::{client, Result as redis_Result};
use color_eyre::eyre::Result;
use tracing::info;
use std::collections::HashMap;
use rayon::prelude::*;
use chrono::prelude::{DateTime, Local};

fn main() {
    println!("Hello, world!");
};

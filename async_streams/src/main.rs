use futures::{stream, Future, Stream, StreamExt};
use lazy_static::lazy_static;
use rand::distributions::{Distribution, Uniform};
use std::{time::Duration, usize};
use tokio::time::{sleep, Instant};

lazy_static! {
    static ref START_TIME: Instant = Instant::now();
}

#[tokio::main]
async fn main() {
    println!(
        "IDs from first 5 pages:\n{:?}",
        get_ids_n_pages(5).collect::<Vec<usize>>().await
    );
    println!(
        "IDs from first 10 pages, buffered by 5:\n{:?}",
        get_ids_n_pages_buffered(10, 5)
            .collect::<Vec<usize>>()
            .await
    );
    println!(
        "IDs from first 10 pages, buffered, unordered by 5:\n{:?}",
        get_ids_n_pages_buffered(10, 5)
            .collect::<Vec<usize>>()
            .await
    );
}

fn get_ids_n_pages(n: usize) -> impl Stream<Item = usize> {
    get_pages().take(n).flat_map(|page| stream::iter(page))
}

fn get_ids_n_pages_buffered(n: usize, buf_factor: usize) -> impl Stream<Item = usize> {
    get_pages_futures()
        .take(n)
        .buffered(buf_factor)
        .flat_map(|page: Vec<usize>| stream::iter(page))
}

fn get_ids_n_pages_buffered_unordered(n: usize, buf_factor: usize) -> impl Stream<Item = usize> {
    get_pages_futures()
        .take(n)
        .buffer_unordered(buf_factor)
        .flat_map(|page| stream::iter(page))
}
async fn get_n_pages(n: usize) -> Vec<Vec<usize>> {
    get_pages().take(n).collect().await
}

async fn get_n_pages_buffered(n: usize, buf_factor: usize) -> Vec<Vec<usize>> {
    get_pages_futures()
        .buffer_unordered(buf_factor)
        .take(n)
        .collect()
        .await
}

fn get_pages_buffered(buf_factor: usize) -> impl Stream<Item = Vec<usize>> {
    get_pages_futures().buffered(buf_factor)
}

fn get_pages_futures() -> impl Stream<Item = impl Future<Output = Vec<usize>>> {
    stream::iter(0..).map(|i| get_page(i))
}
fn get_pages() -> impl Stream<Item = Vec<usize>> {
    stream::iter(0..).then(get_page)
}

async fn get_page(i: usize) -> Vec<usize> {
    let millis = Uniform::from(0..10).sample(&mut rand::thread_rng());

    println!(
        "[{}] # get_page({}) will complete in {} ms",
        START_TIME.elapsed().as_millis(),
        i,
        millis
    );

    sleep(Duration::from_millis(millis)).await;
    println!(
        "[{}] # get_page({}) completed",
        START_TIME.elapsed().as_millis(),
        i
    );

    (10 * i..10 * (i + 1)).collect()
}

#[derive(Clone, Copy)]
struct Resource(usize);

impl std::fmt::Debug for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("r:{}", self.0))
    }
}

async fn fetch_resource(i: usize) -> Resource {
    let millis = Uniform::from(0..10).sample(&mut rand::thread_rng());
    println!(
        "[{}] ## fetch_resource({}) will complete in {} ms",
        START_TIME.elapsed().as_millis(),
        i,
        millis
    );

    sleep(Duration::from_millis(millis)).await;

    println!(
        "[{}] ## fetch_resource({}) completed",
        START_TIME.elapsed().as_millis(),
        i
    );

    Resource(i)
}

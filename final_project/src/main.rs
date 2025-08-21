use std::time::{Duration, Instant};
use std::sync::{mpsc::{channel, Sender, Receiver}, Arc, Mutex};
use std::thread;
use ureq;
use chrono::{Utc, DateTime};

#[derive(Debug)]
pub struct WebsiteStatus {
    pub url: String,
    pub status: Result<u16, String>,
    pub response_time: Duration,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct MonitorConfig {
    pub worker_threads: usize,
    pub request_timeout: Duration,
    pub max_retries: usize,
}

fn worker(
    id: usize,
    receiver: Arc<Mutex<Receiver<String>>>,
    sender: Sender<WebsiteStatus>,
    config: MonitorConfig,
) {
    let agent = ureq::AgentBuilder::new()
        .timeout(config.request_timeout)
        .build();

    loop {
        let url = {
            let locked_receiver = receiver.lock().unwrap();
            match locked_receiver.recv() {
                Ok(url) => url,
                Err(_) => break,
            }
        };

        let start_time = Instant::now();
        let mut status_result: Result<u16, String> = Err("Initial state".to_string());
        let mut retries = 0;

        while retries <= config.max_retries {
            match agent.get(&url).call() {
                Ok(response) => {
                    status_result = Ok(response.status());
                    break;
                }
                Err(e) => {
                    status_result = Err(format!("Error: {}", e));
                    retries += 1;
                    if retries <= config.max_retries {
                        thread::sleep(Duration::from_millis(100));
                    }
                }
            }
        }

        let response_time = start_time.elapsed();
        let timestamp = Utc::now();

        let website_status = WebsiteStatus {
            url,
            status: status_result,
            response_time,
            timestamp,
        };
        sender.send(website_status).unwrap();
    }
}

fn main() {
    let (url_sender, url_receiver) = channel::<String>();
    let (status_sender, status_receiver) = channel::<WebsiteStatus>();

    let config = MonitorConfig {
        worker_threads: 4,
        request_timeout: Duration::from_secs(5),
        max_retries: 2,
    };

    let shared_url_receiver = Arc::new(Mutex::new(url_receiver));

    for i in 0..config.worker_threads {
        let worker_url_receiver = Arc::clone(&shared_url_receiver);
        let worker_status_sender = status_sender.clone();
        let worker_config = config.clone();

        thread::spawn(move || {
            worker(i, worker_url_receiver, worker_status_sender, worker_config);
        });
    }

    url_sender.send("https://www.google.com".to_string()).unwrap();
    url_sender.send("https://www.youtube.com".to_string()).unwrap();
    url_sender.send("https://www.facebook.com".to_string()).unwrap();
    url_sender.send("https://www.instagram.com".to_string()).unwrap();
    url_sender.send("https://www.x.com".to_string()).unwrap();
    url_sender.send("https://www.reddit.com".to_string()).unwrap();
    url_sender.send("https://www.whatsapp.com".to_string()).unwrap();
    url_sender.send("https://www.bing.com".to_string()).unwrap();
    url_sender.send("https://www.wikipedia.org".to_string()).unwrap();
    url_sender.send("https://www.yahoo.co.jp".to_string()).unwrap();
    url_sender.send("https://www.yahoo.com".to_string()).unwrap();
    url_sender.send("https://www.yandex.ru".to_string()).unwrap();
    url_sender.send("https://www.amazon.com".to_string()).unwrap();
    url_sender.send("https://www.tiktok.com".to_string()).unwrap();
    url_sender.send("https://www.baidu.com".to_string()).unwrap();
    url_sender.send("https://www.linkedin.com".to_string()).unwrap();
    url_sender.send("https://www.temu.com".to_string()).unwrap();
    url_sender.send("https://www.netflix.com".to_string()).unwrap();
    url_sender.send("https://www.naver.com".to_string()).unwrap();
    url_sender.send("https://www.live.com".to_string()).unwrap();
    url_sender.send("https://www.dzen.ru".to_string()).unwrap();
    url_sender.send("https://www.pinterest.com".to_string()).unwrap();
    url_sender.send("https://www.bilibili.com".to_string()).unwrap();
    url_sender.send("https://www.office.com".to_string()).unwrap();
    url_sender.send("https://www.twitch.tv".to_string()).unwrap();
    url_sender.send("https://www.microsoft.com".to_string()).unwrap();
    url_sender.send("https://www.weather.com".to_string()).unwrap();
    url_sender.send("https://www.vk.com".to_string()).unwrap();
    url_sender.send("https://www.news.yahoo.co.jp".to_string()).unwrap();
    url_sender.send("https://www.fandom.com".to_string()).unwrap();
    url_sender.send("https://www.mail.ru".to_string()).unwrap();
    url_sender.send("https://www.samsung.com".to_string()).unwrap();
    url_sender.send("https://www.sharepoint.com".to_string()).unwrap();
    url_sender.send("https://www.roblox.com".to_string()).unwrap();
    url_sender.send("https://www.globo.com".to_string()).unwrap();
    url_sender.send("https://www.t.me".to_string()).unwrap();
    url_sender.send("https://www.canva.com".to_string()).unwrap();
    url_sender.send("https://www.duckduckgo.com".to_string()).unwrap();
    url_sender.send("https://www.booking.com".to_string()).unwrap();
    url_sender.send("https://www.ebay.com".to_string()).unwrap();
    url_sender.send("https://www.nytimes.com".to_string()).unwrap();
    url_sender.send("https://www.aliexpress.com".to_string()).unwrap();
    url_sender.send("https://www.paypal.com".to_string()).unwrap();
    url_sender.send("https://www.discord.com".to_string()).unwrap();
    url_sender.send("https://www.github.com".to_string()).unwrap();
    url_sender.send("https://www.spotify.com".to_string()).unwrap();
    url_sender.send("https://www.imdb.com".to_string()).unwrap();
    url_sender.send("https://www.quora.com".to_string()).unwrap();
    url_sender.send("https://www.walmart.com".to_string()).unwrap();
    url_sender.send("https://www.bbc.co.uk".to_string()).unwrap();
    url_sender.send("https://www.espn.com".to_string()).unwrap();
    drop(url_sender);

    for status in status_receiver {
        println!("{:?}", status);
    }
}
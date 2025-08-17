use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use wreq::{Client, header};
use wreq_util::Emulation;
use rand::seq::SliceRandom;
use rand::Rng;
use std::time::Duration;
use std::collections::HashMap;
use tokio::time::sleep;
use std::sync::Arc;
use std::fs;
use clap::{Arg, Command};

// API URL-ləri
const NAR_API_URL: &str = "https://esim.nar.az/api/number-discovery/stock";
const BAKCELL_API_URL: &str = "https://esim.bakcell.com/api/number-discovery/stock/msisdn/level/organization";

// Defolt User-Agent siyahısı (genişləndirilmiş)
fn default_user_agents() -> Vec<String> {
    vec![
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36".to_string(),
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36".to_string(),
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36 Edg/138.0.0.0".to_string(),
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36".to_string(),
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:141.0) Gecko/20100101 Firefox/141.0".to_string(),
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:141.0) Gecko/20100101 Firefox/141.0".to_string(),
        "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:141.0) Gecko/20100101 Firefox/141.0".to_string(),
        "Mozilla/5.0 (iPhone; CPU iPhone OS 18_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.0 Mobile/15E148 Safari/604.1".to_string(),
        "Mozilla/5.0 (iPad; CPU OS 18_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.0 Mobile/15E148 Safari/604.1".to_string(),
        "Mozilla/5.0 (Linux; Android 15; Pixel 9) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Mobile Safari/537.36".to_string(),
        "Mozilla/5.0 (Linux; Android 15; SM-S928B) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Mobile Safari/537.36".to_string(),
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36 OPR/123.0.0.0".to_string(),
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.0 Safari/605.1.15".to_string(),
        "Mozilla/5.0 (X11; Linux x86_64; rv:141.0) Gecko/20100101 Firefox/141.0".to_string(),
        "Mozilla/5.0 (Windows NT 11.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36".to_string(),
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 11_0_0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36".to_string(),
        "Mozilla/5.0 (iPhone; CPU iPhone OS 17_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/138.0.0.0 Mobile/15E148 Safari/604.1".to_string(),
        "Mozilla/5.0 (Linux; Android 14; Pixel 8) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Mobile Safari/537.36".to_string(),
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36 Edg/138.0.0.0".to_string(),
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36 Vivaldi/7.0.0.0".to_string(),
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36 Edg/138.0.0.0".to_string(),
        "Mozilla/5.0 (X11; Fedora; Linux x86_64; rv:141.0) Gecko/20100101 Firefox/141.0".to_string(),
        "Mozilla/5.0 (Linux; Android 15; SM-A556B) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Mobile Safari/537.36".to_string(),
        "Mozilla/5.0 (iPad; CPU OS 17_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) GSA/138.0.0.0 Mobile/15E148 Safari/604.1".to_string(),
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) rv:141.0) Gecko/20100101 Firefox/141.0".to_string(),
    ]
}

// Nar üçün parametrlər modeli
#[derive(Deserialize)]
struct NarParams {
    #[serde(default)]
    page: i32,
    number: String,
    #[serde(default = "default_nar_prefix")]
    prefix: String,
    #[serde(default = "default_nar_size")]
    size: i32,
    #[serde(default = "default_nar_provider_id")]
    provider_id: i32,
    #[serde(default)]
    time: u64,
}

fn default_nar_prefix() -> String { "070".to_string() }
fn default_nar_size() -> i32 { 50 }
fn default_nar_provider_id() -> i32 { 1 }

// Bakcell üçün parametrlər modeli
#[derive(Deserialize)]
struct BakcellParams {
    #[serde(default)]
    page: i32,
    number: String,
    #[serde(default = "default_bakcell_prefix")]
    prefix: String,
    #[serde(default = "default_bakcell_size")]
    size: i32,
    #[serde(default = "default_bakcell_provider_id")]
    provider_id: i32,
    #[serde(default = "default_bakcell_organization")]
    organization: i32,
    #[serde(default = "default_bakcell_msisdn_type")]
    msisdn_type: String,
    #[serde(default)]
    time: u64,
}

fn default_bakcell_prefix() -> String { "55".to_string() }
fn default_bakcell_size() -> i32 { 6 }
fn default_bakcell_provider_id() -> i32 { 2 }
fn default_bakcell_organization() -> i32 { 1 }
fn default_bakcell_msisdn_type() -> String { "E_SIM".to_string() }

// Nar başlıqları
fn nar_headers(ua: &str) -> Vec<(&'static str, String)> {
    vec![
        ("Accept", "application/json, text/plain, */*".to_string()),
        ("Accept-Encoding", "gzip, deflate, br, zstd".to_string()),
        ("Accept-Language", "tr-TR,tr;q=0.9,en-US;q=0.8,en;q=0.7".to_string()),
        ("Priority", "u=1, i".to_string()),
        ("Provider-Id", "1".to_string()),
        ("Referer", "https://esim.nar.az".to_string()),
        ("Referrer", "1".to_string()),
        ("Sec-CH-UA", r#""Not)A;Brand";v="8", "Chromium";v="138", "Google Chrome";v="138""#.to_string()),
        ("Sec-CH-UA-Mobile", "?0".to_string()),
        ("Sec-CH-UA-Platform", r#""Linux""#.to_string()),
        ("Sec-Fetch-Dest", "empty".to_string()),
        ("Sec-Fetch-Mode", "cors".to_string()),
        ("Sec-Fetch-Site", "same-origin".to_string()),
        ("User-Agent", ua.to_string()),
        ("X-API-Key", "f14b1e9f-ddaa-4537-ad81-6bc910927caa".to_string()),
        ("X-API-Version", "1.0.0".to_string()),
        ("X-App-Id", "b6920d9083c8e76685bcc8db34b8c9bb".to_string()),
        ("X-Session-Id", "607538e2-a52c-42b3-b926-2d9293298b95".to_string()),
    ]
}

// Bakcell başlıqları
fn bakcell_headers(ua: &str) -> Vec<(&'static str, String)> {
    vec![
        ("Accept", "application/json, text/plain, */*".to_string()),
        ("Accept-Encoding", "gzip, deflate, br, zstd".to_string()),
        ("Accept-Language", "tr-TR,tr;q=0.9,en-US;q=0.8,en;q=0.7".to_string()),
        ("Origin", "https://www.bakcell.com".to_string()),
        ("Provider-Id", "2".to_string()),
        ("Referer", "https://www.bakcell.com/".to_string()),
        ("Referrer", "2".to_string()),
        ("Sec-CH-UA", r#""Not)A;Brand";v="8", "Chromium";v="138", "Google Chrome";v="138""#.to_string()),
        ("Sec-CH-UA-Mobile", "?0".to_string()),
        ("Sec-CH-UA-Platform", r#""Linux""#.to_string()),
        ("Sec-Fetch-Dest", "empty".to_string()),
        ("Sec-Fetch-Mode", "cors".to_string()),
        ("Sec-Fetch-Site", "same-site".to_string()),
        ("User-Agent", ua.to_string()),
        ("X-API-Version", "1.0.0".to_string()),
        ("X-App-Id", "b6920d9083c8e76685bcc8db34b8c9bb".to_string()),
    ]
}

// wreq müştərisi yarat
async fn create_client() -> wreq::Result<Client> {
    Client::builder()
        .emulation(Emulation::Chrome137)
        .timeout(Duration::from_secs(30))
        .cert_verification(false)
        .build()
}

// Cookie alma funksiyası
async fn get_cookies(client: &Client, url: &str, headers: Vec<(&str, String)>) -> wreq::Result<String> {
    let mut req = client.get(url);
    for (key, value) in headers {
        req = req.header(key, value);
    }
    let resp = req.send().await?;
    let cookies = resp
        .headers()
        .get_all(header::SET_COOKIE)
        .iter()
        .map(|v| v.to_str().unwrap_or_default().to_string())
        .collect::<Vec<_>>()
        .join("; ");
    println!("Alınmış cookie-lər: {}", cookies);
    Ok(cookies)
}

// Nar endpoint-i
#[get("/nar/stock")]
async fn nar_proxy(
    params: web::Query<NarParams>,
    app_data: web::Data<Arc<(Vec<String>, u64)>>,
) -> impl Responder {
    let (user_agents, max_delay) = &***app_data;

    // Təsadüfi gecikmə əlavə et (varsa)
    if *max_delay > 0 {
        let delay = rand::thread_rng().gen_range(0..=*max_delay);
        sleep(Duration::from_millis(delay)).await;
    }

    // Query-dən gələn time parametri ilə gecikmə əlavə et
    if params.time > 0 {
        sleep(Duration::from_millis(params.time)).await;
    }

    let client = match create_client().await {
        Ok(client) => client,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Müştəri yaradılmadı: {}", e)),
    };

    // Təsadüfi User-Agent seç
    let ua = user_agents.choose(&mut rand::thread_rng())
        .map(|s| s.as_str())
        .unwrap_or("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36");
    let mut headers = nar_headers(ua);

    // Cookie-ləri al
    let cookies = match get_cookies(&client, "https://esim.nar.az", headers.clone()).await {
        Ok(cookies) => cookies,
        Err(e) => {
            println!("Cookie alınarkən xəta: {}", e);
            "".to_string()
        }
    };
    if !cookies.is_empty() {
        headers.push(("Cookie", cookies));
    }

    // Parametrləri hazırla
    let mut query_map: HashMap<String, String> = HashMap::new();
    query_map.insert("page".to_string(), params.page.to_string());
    query_map.insert("number".to_string(), params.number.clone());
    query_map.insert("prefix".to_string(), params.prefix.clone());
    query_map.insert("size".to_string(), params.size.to_string());
    query_map.insert("providerId".to_string(), params.provider_id.to_string());

    // API sorğusu
    let mut req = client.get(NAR_API_URL);
    for (key, value) in headers {
        req = req.header(key, value);
    }
    req = req.query(&query_map);

    match req.send().await {
        Ok(resp) if resp.status().is_success() => {
            let mut response_json: serde_json::Value = match resp.json().await {
                Ok(json) => json,
                Err(e) => return HttpResponse::InternalServerError().body(format!("JSON parse xətası: {}", e)),
            };

            // Prefix ilə filtrlə və total/has_next hesabla
            let requested_prefix = params.prefix.clone();
            let (total, has_next) = if let Some(data) = response_json.get_mut("data").and_then(|d| d.as_array_mut()) {
                data.retain(|item| item.get("prefix").and_then(|p| p.as_str()) == Some(&requested_prefix));
                let len = data.len();
                (len, len > params.size as usize)
            } else {
                (0, false)
            };

            // Metadata-nı yenilə
            if let Some(metadata) = response_json.get_mut("metadata").and_then(|m| m.get_mut("pagination")) {
                if let Some(obj) = metadata.as_object_mut() {
                    obj.insert("total".to_string(), serde_json::Value::Number(total.into()));
                    obj.insert("has_next".to_string(), serde_json::Value::Bool(has_next));
                }
            }

            HttpResponse::Ok().json(response_json)
        }
        Ok(resp) => {
            let status = actix_web::http::StatusCode::from_u16(resp.status().as_u16())
                .unwrap_or(actix_web::http::StatusCode::BAD_GATEWAY);
            let body = match resp.text().await {
                Ok(text) => format!("Nar API xətası: {}", text),
                Err(e) => format!("Nar API xətası (mətn oxuma uğursuz): {}", e),
            };
            HttpResponse::build(status).body(body)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Nar API giriş xətası: {}", e)),
    }
}

// Bakcell endpoint-i
#[get("/bakcell/stock")]
async fn bakcell_proxy(
    params: web::Query<BakcellParams>,
    app_data: web::Data<Arc<(Vec<String>, u64)>>,
) -> impl Responder {
    let (user_agents, max_delay) = &***app_data;

    // Təsadüfi gecikmə əlavə et (varsa)
    if *max_delay > 0 {
        let delay = rand::thread_rng().gen_range(0..=*max_delay);
        sleep(Duration::from_millis(delay)).await;
    }

    // Query-dən gələn time parametri ilə gecikmə əlavə et
    if params.time > 0 {
        sleep(Duration::from_millis(params.time)).await;
    }

    let client = match create_client().await {
        Ok(client) => client,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Müştəri yaradılmadı: {}", e)),
    };

    // Təsadüfi User-Agent seç
    let ua = user_agents.choose(&mut rand::thread_rng())
        .map(|s| s.as_str())
        .unwrap_or("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36");
    let mut headers = bakcell_headers(ua);

    // Cookie-ləri al
    let cookies = match get_cookies(&client, "https://esim.bakcell.com", headers.clone()).await {
        Ok(cookies) => cookies,
        Err(e) => {
            println!("Cookie alınarkən xəta: {}", e);
            "".to_string()
        }
    };
    if !cookies.is_empty() {
        headers.push(("Cookie", cookies));
    }

    // Parametrləri hazırla
    let mut query_map: HashMap<String, String> = HashMap::new();
    query_map.insert("page".to_string(), params.page.to_string());
    query_map.insert("number".to_string(), params.number.clone());
    query_map.insert("prefix".to_string(), params.prefix.clone());
    query_map.insert("size".to_string(), params.size.to_string());
    query_map.insert("providerId".to_string(), params.provider_id.to_string());
    query_map.insert("organization".to_string(), params.organization.to_string());
    query_map.insert("msisdnType".to_string(), params.msisdn_type.clone());

    // API sorğusu
    let mut req = client.get(BAKCELL_API_URL);
    for (key, value) in headers {
        req = req.header(key, value);
    }
    req = req.query(&query_map);

    match req.send().await {
        Ok(resp) if resp.status().is_success() => {
            let mut response_json: serde_json::Value = match resp.json().await {
                Ok(json) => json,
                Err(e) => return HttpResponse::InternalServerError().body(format!("JSON parse xətası: {}", e)),
            };

            // Prefix ilə filtrlə və total/has_next hesabla
            let requested_prefix = params.prefix.clone();
            let (total, has_next) = if let Some(data) = response_json.get_mut("data").and_then(|d| d.as_array_mut()) {
                data.retain(|item| item.get("prefix").and_then(|p| p.as_str()) == Some(&requested_prefix));
                let len = data.len();
                (len, len > params.size as usize)
            } else {
                (0, false)
            };

            // Metadata-nı yenilə
            if let Some(metadata) = response_json.get_mut("metadata").and_then(|m| m.get_mut("pagination")) {
                if let Some(obj) = metadata.as_object_mut() {
                    obj.insert("total".to_string(), serde_json::Value::Number(total.into()));
                    obj.insert("has_next".to_string(), serde_json::Value::Bool(has_next));
                }
            }

            HttpResponse::Ok().json(response_json)
        }
        Ok(resp) => {
            let status = actix_web::http::StatusCode::from_u16(resp.status().as_u16())
                .unwrap_or(actix_web::http::StatusCode::BAD_GATEWAY);
            let body = match resp.text().await {
                Ok(text) => format!("Bakcell API xətası: {}", text),
                Err(e) => format!("Bakcell API xətası (mətn oxuma uğursuz): {}", e),
            };
            HttpResponse::build(status).body(body)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Bakcell API giriş xətası: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Komanda sətri arqumentlərini parse et
    let matches = Command::new("API Proxy")
        .version("1.0")
        .arg(
            Arg::new("user_agent_file")
                .long("user-agent-file")
                .short('u')
                .value_name("FILE")
                .help("User-Agent faylının yolu (#\\n ilə ayrılmış)")
                .default_value("")
                .required(false)
        )
        .arg(
            Arg::new("port")
                .long("port")
                .short('p')
                .value_name("PORT")
                .help("Serverin işləyəcəyi port")
                .default_value("8000")
                .value_parser(clap::value_parser!(u16))
                .required(false)
        )
        .arg(
            Arg::new("time")
                .long("time")
                .short('t')
                .value_name("MILLISECONDS")
                .help("Hər sorğu üçün maksimum təsadüfi gecikmə (ms)")
                .default_value("0")
                .value_parser(clap::value_parser!(u64))
                .required(false)
        )
        .get_matches();

    // User-Agent faylını oxu
    let user_agent_file = matches.get_one::<String>("user_agent_file").unwrap();
    let user_agents = if !user_agent_file.is_empty() {
        match fs::read_to_string(user_agent_file) {
            Ok(content) => content
                .split("#\n")
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect::<Vec<String>>(),
            Err(e) => {
                println!("User-agent faylı oxuna bilmədi: {}. Defoltlar istifadə olunur.", e);
                default_user_agents()
            }
        }
    } else {
        default_user_agents()
    };

    // Maksimum təsadüfi gecikmə müddətini al
    let max_delay = *matches.get_one::<u64>("time").unwrap();

    // User agents və max_delay-i paylaşılabilən məlumat strukturunda birləşdir
    let app_data = Arc::new((user_agents, max_delay));

    // Portu al
    let port = *matches.get_one::<u16>("port").unwrap();
    println!("Server port {} üzərində işləyir, maksimum təsadüfi gecikmə: {}ms", port, max_delay);


    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_data.clone()))
            .service(nar_proxy)
            .service(bakcell_proxy)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

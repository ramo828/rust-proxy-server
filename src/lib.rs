use wreq::{Client, header};
use wreq_util::Emulation;
use serde_json::Value;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::time::Duration;
use std::collections::HashMap;
use tokio::runtime::Runtime;
use rand::seq::SliceRandom;

// API URL'leri
const NAR_API_URL: &str = "https://esim.nar.az/api/number-discovery/stock";
const BAKCELL_API_URL: &str = "https://esim.bakcell.com/api/number-discovery/stock/msisdn/level/organization";

// Varsayılan User-Agent listesi
fn default_user_agents() -> Vec<String> {
    vec![
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36".to_string(),
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36".to_string(),
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36 Edg/138.0.0.0".to_string(),
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36".to_string(),
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:141.0) Gecko/20100101 Firefox/141.0".to_string(),
        // Diğer user-agent'larınızı buraya ekleyin
    ]
}

// Nar header'ları
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

// Bakcell header'ları
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

// wreq istemcisi oluştur
async fn create_client() -> wreq::Result<Client> {
    Client::builder()
        .emulation(Emulation::Chrome137)
        .timeout(Duration::from_secs(30))
        .cert_verification(false)
        .build()
}

// Cookie alma fonksiyonu
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
    Ok(cookies)
}

// Nar proxy fonksiyonu (FFI uyumlu)
#[unsafe(no_mangle)]
pub extern "C" fn nar_proxy(number: *const c_char, prefix: *const c_char) -> *mut c_char {
    let number = unsafe { CStr::from_ptr(number).to_string_lossy().into_owned() };
    let prefix = unsafe { CStr::from_ptr(prefix).to_string_lossy().into_owned() };

    let rt = Runtime::new().unwrap();
    let result = rt.block_on(async {
        let user_agents = default_user_agents();
        let ua = user_agents.choose(&mut rand::thread_rng())
            .map(|s| s.as_str())
            .unwrap_or("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36");
        let mut headers = nar_headers(ua);

        let client = match create_client().await {
            Ok(client) => client,
            Err(e) => return format!("Client oluşturulamadı: {}", e),
        };

        let cookies = match get_cookies(&client, "https://esim.nar.az", headers.clone()).await {
            Ok(cookies) => cookies,
            Err(e) => {
                println!("Cookie alınırken hata: {}", e);
                "".to_string()
            }
        };
        if !cookies.is_empty() {
            headers.push(("Cookie", cookies));
        }

        let mut query_map: HashMap<String, String> = HashMap::new();
        query_map.insert("page".to_string(), "0".to_string());
        query_map.insert("number".to_string(), number.clone());
        query_map.insert("prefix".to_string(), prefix.clone());
        query_map.insert("size".to_string(), "50".to_string());
        query_map.insert("providerId".to_string(), "1".to_string());

        let mut req = client.get(NAR_API_URL);
        for (key, value) in headers {
            req = req.header(key, value);
        }
        req = req.query(&query_map);

        match req.send().await {
            Ok(resp) if resp.status().is_success() => {
                let mut response_json: Value = match resp.json().await {
                    Ok(json) => json,
                    Err(e) => return format!("JSON parse hatası: {}", e),
                };

                let requested_prefix = prefix.clone();
                let (total, has_next) = if let Some(data) = response_json.get_mut("data").and_then(|d| d.as_array_mut()) {
                    data.retain(|item| item.get("prefix").and_then(|p| p.as_str()) == Some(&requested_prefix));
                    let len = data.len();
                    (len, len > 50)
                } else {
                    (0, false)
                };

                if let Some(metadata) = response_json.get_mut("metadata").and_then(|m| m.get_mut("pagination")) {
                    if let Some(obj) = metadata.as_object_mut() {
                        obj.insert("total".to_string(), serde_json::Value::Number(total.into()));
                        obj.insert("has_next".to_string(), serde_json::Value::Bool(has_next));
                    }
                }

                serde_json::to_string(&response_json).unwrap_or_else(|e| format!("JSON seri hale getirme hatası: {}", e))
            }
            Ok(resp) => format!("Nar API hatası: Status {}", resp.status()),
            Err(e) => format!("Nar API erişim hatası: {}", e),
        }
    });

    let c_result = CString::new(result).unwrap();
    c_result.into_raw()
}

// Bakcell proxy fonksiyonu (FFI uyumlu)
#[unsafe(no_mangle)]
pub extern "C" fn bakcell_proxy(number: *const c_char, prefix: *const c_char) -> *mut c_char {
    let number = unsafe { CStr::from_ptr(number).to_string_lossy().into_owned() };
    let prefix = unsafe { CStr::from_ptr(prefix).to_string_lossy().into_owned() };

    let rt = Runtime::new().unwrap();
    let result = rt.block_on(async {
        let user_agents = default_user_agents();
        let ua = user_agents.choose(&mut rand::thread_rng())
            .map(|s| s.as_str())
            .unwrap_or("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36");
        let mut headers = bakcell_headers(ua);

        let client = match create_client().await {
            Ok(client) => client,
            Err(e) => return format!("Client oluşturulamadı: {}", e),
        };

        let cookies = match get_cookies(&client, "https://esim.bakcell.com", headers.clone()).await {
            Ok(cookies) => cookies,
            Err(e) => {
                println!("Cookie alınırken hata: {}", e);
                "".to_string()
            }
        };
        if !cookies.is_empty() {
            headers.push(("Cookie", cookies));
        }

        let mut query_map: HashMap<String, String> = HashMap::new();
        query_map.insert("page".to_string(), "0".to_string());
        query_map.insert("number".to_string(), number.clone());
        query_map.insert("prefix".to_string(), prefix.clone());
        query_map.insert("size".to_string(), "6".to_string());
        query_map.insert("providerId".to_string(), "2".to_string());
        query_map.insert("organization".to_string(), "1".to_string());
        query_map.insert("msisdnType".to_string(), "E_SIM".to_string());

        let mut req = client.get(BAKCELL_API_URL);
        for (key, value) in headers {
            req = req.header(key, value);
        }
        req = req.query(&query_map);

        match req.send().await {
            Ok(resp) if resp.status().is_success() => {
                let mut response_json: Value = match resp.json().await {
                    Ok(json) => json,
                    Err(e) => return format!("JSON parse hatası: {}", e),
                };

                let requested_prefix = prefix.clone();
                let (total, has_next) = if let Some(data) = response_json.get_mut("data").and_then(|d| d.as_array_mut()) {
                    data.retain(|item| item.get("prefix").and_then(|p| p.as_str()) == Some(&requested_prefix));
                    let len = data.len();
                    (len, len > 6)
                } else {
                    (0, false)
                };

                if let Some(metadata) = response_json.get_mut("metadata").and_then(|m| m.get_mut("pagination")) {
                    if let Some(obj) = metadata.as_object_mut() {
                        obj.insert("total".to_string(), serde_json::Value::Number(total.into()));
                        obj.insert("has_next".to_string(), serde_json::Value::Bool(has_next));
                    }
                }

                serde_json::to_string(&response_json).unwrap_or_else(|e| format!("JSON seri hale getirme hatası: {}", e))
            }
            Ok(resp) => format!("Bakcell API hatası: Status {}", resp.status()),
            Err(e) => format!("Bakcell API erişim hatası: {}", e),
        }
    });

    let c_result = CString::new(result).unwrap();
    c_result.into_raw()
}

// Belleği serbest bırakma fonksiyonu
#[unsafe(no_mangle)]
pub extern "C" fn free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe { let _ = CString::from_raw(s); }
    }
}

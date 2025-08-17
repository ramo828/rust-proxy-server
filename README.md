# API Proxy Server for Dealsbe

**Versiya**: 1.0  
**Müəllif**: Ramiz Mammadli  
**Lisenziya**: MIT  
**Son Yenilənmə**: 17 Avqust 2025  
**Proqramlaşdırma dili**: Rust  
**Çərçivə**: Actix Web  
**Məqsəd**: Nar və Bakcell eSIM API-ləri üçün proksi server təmin etmək və Dealsbe platforması ilə inteqrasiya

---

## Təsvir

Bu layihə, Rust proqramlaşdırma dili və Actix Web çərçivəsi ilə yazılmış bir API proksi serveridir. Əsas məqsədi Nar (https://esim.nar.az) və Bakcell (https://esim.bakcell.com) eSIM API-lərinə sorğular göndərmək, cavabları filtrləmək və bu məlumatları Dealsbe platforması ilə inteqrasiya etməkdir. Dealsbe, tərtibatçılar və startaplar üçün eksklüziv proqram təminatı təklifləri təqdim edən bir platformadır və bu proksi server, eSIM xidmətləri ilə bağlı məlumatların effektiv idarə olunmasını dəstəkləyir.

**Dealsbe haqqında**: Dealsbe, tərtibatçılar və startaplar üçün proqram təminatı ilə bağlı eksklüziv təkliflər təqdim edir. "Fresh Recommendations" bölməsi istifadəçilərə ən son təklifləri kəşf etməyə imkan verir, həmçinin "Post a Deal" funksiyası ilə öz təkliflərini paylaşmaq mümkündür. Bu server, Dealsbe platformasının eSIM ilə bağlı xidmətləri inteqrasiya etmək üçün infrastruktur təmin edir.

### Əsas Xüsusiyyətlər
- **Nar və Bakcell API Dəstəyi**: Hər iki operatorun eSIM stok məlumatlarına giriş imkanı.
- **Təsadüfi User-Agent Seçimi**: 25+ fərqli brauzer və cihaz üçün User-Agent siyahısı ilə real istifadəçi təcrübəsi simulyasiyası.
- **Gecikmə Simulyasiyası**: Sorğular arasında təsadüfi və ya müəyyən edilmiş gecikmələr əlavə etmək imkanı.
- **Cookie İdarəetməsi**: API sorğuları üçün avtomatik cookie əldə etmə və istifadə.
- **Prefix Filtrləmə**: API cavablarında yalnız tələb olunan prefiksə uyğun nömrələri saxlamaq.
- **Komanda Sətri Konfiqurasiyası**: Port, User-Agent faylı və gecikmə müddətini təyin etmək üçün çevik seçimlər.
- **Dealsbe İnteqrasiyası**: eSIM məlumatlarının Dealsbe platformasında təklif kimi paylaşılması üçün infrastruktur.

---

## Description (English)

This project is an API proxy server written in Rust using the Actix Web framework. Its primary purpose is to send requests to the Nar (https://esim.nar.az) and Bakcell (https://esim.bakcell.com) eSIM APIs, filter their responses, and integrate with the Dealsbe platform. Dealsbe is a platform offering exclusive software deals for developers and startups, and this proxy server supports the efficient management of eSIM-related data for integration with Dealsbe.

**About Dealsbe**: Dealsbe provides exclusive software deals for developers and startups. The "Fresh Recommendations" section allows users to discover the latest offers, while the "Post a Deal" feature enables users to share their own deals. This server provides the infrastructure to integrate eSIM services with the Dealsbe platform.

### Key Features
- **Nar and Bakcell API Support**: Access to eSIM stock data for both operators.
- **Random User-Agent Selection**: Over 25 User-Agents to simulate real-world browser and device interactions.
- **Delay Simulation**: Ability to add random or specified delays between requests.
- **Cookie Management**: Automatic retrieval and use of cookies for API requests.
- **Prefix Filtering**: Retains only numbers matching the requested prefix in API responses.
- **Command-Line Configuration**: Flexible options to configure port, User-Agent file, and delay duration.
- **Dealsbe Integration**: Infrastructure for sharing eSIM data as offers on the Dealsbe platform.

---

## Quraşdırma

### Tələblər
- **Rust**: Stabil versiya (1.80 və ya yuxarı)
- **Cargo**: Rust paket meneceri
- **Tokio**: Asinxron runtime (avtomatik quraşdırılır)
- **wreq**: HTTP sorğuları üçün kitabxana
- **clap**: Komanda sətri arqumentləri üçün kitabxana
- **İnternet Bağlantısı**: Nar və Bakcell API-lərinə giriş üçün

### Quraşdırma Addımları
1. **Rust və Cargo-nu quraşdırın**:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh


Repozitoriyanı klonlayın:git clone <repo-url>
cd api-proxy


Asılılıqları quraşdırın və proqramı qurun:cargo build --release




İstifadə
Serverin Başladılması
Proqramı defolt parametrlərlə işə salmaq üçün:
cargo run --release

Bu, serveri 0.0.0.0:8000 ünvanında başlatacaq.
Xüsusi Parametrlər

Port: Serverin işləyəcəyi portu təyin edin:cargo run --release -- --port 8080


User-Agent Faylı: Xüsusi User-Agent siyahısı olan faylı təyin edin:cargo run --release -- --user-agent-file user_agents.txt

Fayl formatı: Hər User-Agent yeni sətirdə, #\n ilə ayrılmış.Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36
#\n
Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36


Gecikmə: Sorğular arasında maksimum təsadüfi gecikmə (millisaniyə):cargo run --release -- --time 500



API Endpointləri

Nar API Proxy (/nar/stock):

Metod: GET
Parametrlər:
page: Səhifə nömrəsi (defolt: 0)
number: Axtarılacaq nömrə (məsələn, "123456")
prefix: Nömrə prefiksi (defolt: "070")
size: Səhifə ölçüsü (defolt: 50)
provider_id: Provayder ID (defolt: 1)
time: Gecikmə müddəti (ms, defolt: 0)


Nümunə Sorğu:curl "http://localhost:8000/nar/stock?page=1&number=123456&prefix=070&size=10"


Cavab Formatı (JSON):{
  "data": [
    {"prefix": "070", "number": "123456789"},
    ...
  ],
  "metadata": {
    "pagination": {
      "total": 10,
      "has_next": true
    }
  }
}




Bakcell API Proxy (/bakcell/stock):

Metod: GET
Parametrlər:
page: Səhifə nömrəsi (defolt: 0)
number: Axtarılacaq nömrə (məsələn, "123456")
prefix: Nömrə prefiksi (defolt: "55")
size: Səhifə ölçüsü (defolt: 6)
provider_id: Provayder ID (defolt: 2)
organization: Təşkilat ID (defolt: 1)
msisdn_type: Nömrə tipi (defolt: "E_SIM")
time: Gecikmə müddəti (ms, defolt: 0)


Nümunə Sorğu:curl "http://localhost:8000/bakcell/stock?page=1&number=123456&prefix=55&size=6&msisdn_type=E_SIM"


Cavab Formatı (JSON):{
  "data": [
    {"prefix": "55", "number": "123456789"},
    ...
  ],
  "metadata": {
    "pagination": {
      "total": 6,
      "has_next": false
    }
  }
}





Dealsbe ilə İnteqrasiya
Dealsbe platformasında eSIM məlumatlarını təklif kimi paylaşmaq üçün:

API-dən alınan nömrə məlumatlarını Dealsbe-nin "Post a Deal" funksiyasına inteqrasiya edin.
Məsələn, Nar və ya Bakcell-dən alınan eSIM nömrələri Dealsbe-də endirimli təklif kimi təqdim oluna bilər.
Təklif formatı:{
  "deal_type": "eSIM",
  "provider": "Nar",
  "prefix": "070",
  "number": "123456789",
  "description": "Exclusive eSIM deal for developers"
}




Kod Strukturu
Əsas Modullar

nar_proxy: Nar API-yə sorğuları idarə edir və prefixə uyğun filtrləmə tətbiq edir.
bakcell_proxy: Bakcell API-yə sorğuları idarə edir və xüsusi parametrlərlə filtrləmə aparır.
create_client: HTTP müştərisi yaradır (wreq kitabxanası istifadə olunur).
get_cookies: API sorğuları üçün cookie-ləri avtomatik əldə edir.
nar_headers / bakcell_headers: Hər bir API üçün xüsusi HTTP header-ləri təyin edir.
default_user_agents: 25+ defolt User-Agent siyahısını təmin edir.

Meta Məlumatların İdarə Edilməsi

API cavablarında metadata.pagination obyekti yenilənir:
total: Prefiksə uyğun nömrələrin ümumi sayı.
has_next: Növbəti səhifənin olub-olmaması (boolean).


Məlumatlar JSON formatında qaytarılır və Dealsbe platformasına inteqrasiya üçün uyğunlaşdırılır.


Təhlükəsizlik və Performans
Təhlükəsizlik

Sertifikat Yoxlaması: Hal-hazırda deaktiv edilib (cert_verification(false)). İstehsal mühitində aktivləşdirilməsi tövsiyə olunur.
API Açarları: X-API-Key və X-Session-Id kimi həssas məlumatlar konfiqurasiya faylına köçürülməlidir.
Cookie İdarəetməsi: API sorğularında təhlükəsizlik üçün avtomatik cookie yenilənməsi tətbiq olunur.

Performans

Asinxron Sorğular: Tokio ilə asinxron işləmə sayəsində yüksək performans təmin edilir.
Gecikmə Simulyasiyası: Təsadüfi gecikmələr API-lərə həddindən artıq yükün qarşısını alır.
Ölçəklənmə: Actix Web çərçivəsi çoxlu sorğuları paralel idarə etməyə imkan verir.


Məhdudiyyətlər

API cavabları yalnız JSON formatında dəstəklənir.
Xüsusi User-Agent faylı təmin edilmədikdə defolt siyahı istifadə olunur.
Gecikmə müddəti 0-dan böyük olarsa, performans təsirlənə bilər.
Dealsbe inteqrasiyası hazırda yalnız eSIM məlumatlarının paylaşılmasını dəstəkləyir; digər təklif növləri üçün əlavə inkişaf tələb oluna bilər.


Gələcək Təkmilləşdirmələr

Dinamik API Açarları: API açarlarının konfiqurasiya faylı və ya mühit dəyişənləri vasitəsilə idarə edilməsi.
Çoxsaylı Provayderlər: Daha çox eSIM provayderi üçün genələşdirilmiş endpoint.
Loglama və Monitorinq: Prometheus və ya Grafana ilə inteqrasiya.
Dealsbe İnteqrasiyası: "Fresh Recommendations" bölməsi üçün avtomatlaşdırılmış təklif yeniləmələri.
Təhlükəsizlik Təkmilləşdirmələri: Sertifikat yoxlamasının aktivləşdirilməsi və OAuth dəstəyi.


Tərtibatçı Resursları

Rust Sənədləri
Actix Web Sənədləri
wreq Kitabxanası
Nar eSIM API
Bakcell eSIM API
Dealsbe Platforması (Qeyd: Link fərzidir, real URL əlavə olunmalıdır)


Lisenziya
Bu layihə MIT Lisenziyası altında paylaşılır. Ətraflı məlumat üçün LICENSE faylına baxın.

Əlaqə: Ramiz Mammadli Dəstək: Problemlər və ya təkliflər üçün ramosoft94@gmail.com səhifəsindən istifadə edin.


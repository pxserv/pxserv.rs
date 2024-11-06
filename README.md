# PxServ

`pxserv`, Rust dilinde verilerinizi basit ve etkili bir şekilde PxServ'e kaydetmenizi ve yönetmenizi sağlayan bir kütüphanedir. Bu kütüphane ile verilerinizi PxServ'e kolayca saklayabilir, alabilir ve silebilirsiniz.

## Kurulum

Projenize `pxserv` kütüphanesini eklemek için öncelikle terminalde şu satırı çalıştırın:

```bash
cargo add pxserv
```

## Kullanım

`pxserv` kütüphanesini kullanmak oldukça basittir. İlk olarak `apikey` değeriyle bir `PxServ` nesnesi oluşturmanız gerekir. Bu nesne üzerinden verilerinizi PxServ'e kaydedebilir, veri çekebilir veya silebilirsiniz.

### Temel Kullanım

```rust
use pxserv::PxServ;

fn main() {
    // PxServ örneği oluşturma
    let client = PxServ::new("API_KEY".to_string());

    // Veri kaydetme
    let status = client.setdata("temperature".to_string(), "22.5°C".to_string());
    println!("Durum: {}, Mesaj: {}", status.status, status.message);

    // Veri çekme
    let response = client.getdata("temperature".to_string());
    if let Some(data) = response.data {
        println!("Veri: {}", data);
    }

    // Veri silme
    let delete_status = client.removedata("temperature".to_string());
    println!(
        "Silme Durumu: {}, Mesaj: {}",
        delete_status.status, delete_status.message
    );
}

```

### Katkıda Bulunma

Katkıda bulunmak isterseniz, projeyi forklayarak kendi branch'inizde geliştirme yapabilirsiniz. Değişikliklerinizi incelemekten memnuniyet duyarız!

## Lisans

[![MIT License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

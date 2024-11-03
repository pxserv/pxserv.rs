# PxServrs

`PxServrs`, Rust dilinde verilerinizi basit ve etkili bir şekilde PxServ'e kaydetmenizi ve yönetmenizi sağlayan bir kütüphanedir. Bu kütüphane ile verilerinizi PxServ'e kolayca saklayabilir, alabilir ve silebilirsiniz.

## Kurulum

Projenize `PxServrs` kütüphanesini eklemek için öncelikle `Cargo.toml` dosyanıza şu satırı ekleyin:

```toml
[dependencies]
pxservrs = "0.0.4"
```

## Kullanım

`PxServrs` kütüphanesini kullanmak oldukça basittir. İlk olarak `apikey` değeriyle bir `PxServ` nesnesi oluşturmanız gerekir. Bu nesne üzerinden verilerinizi PxServ'e kaydedebilir, veri çekebilir veya silebilirsiniz.

### Temel Kullanım

```rust
use pxservrs::PxServ;

fn main() {
    // PxServ örneği oluşturma
    let pxserv = PxServ::new("API_KEY".to_string());

    // Veri kaydetme
    let status = pxserv.setdata("temperature".to_string(), "22.5°C".to_string());
    println!("Durum: {}, Mesaj: {}", status.status, status.message);

    // Veri çekme
    let response = pxserv.getdata("temperature".to_string());
    if let Some(data) = response.data {
        println!("Veri: {}", data);
    }

    // Veri silme
    let delete_status = pxserv.removedata("temperature".to_string());
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

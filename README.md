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
    let client = PxServ::new("API_KEY");

    // Veri kaydetme
    let set_response = client.setdata("temperature", "22.5°C");
    println!(
        "Durum: {}, Mesaj: {}",
        set_response.status, set_response.message
    );

    // Veri çekme
    let get_response = client.getdata("temperature");
    println!(
        "Durum: {}, Mesaj: {}, Veri : {:?}",
        get_response.status, get_response.message, get_response.data
    );

    // Veri silme
    let remove_response = client.removedata("temperature");
    println!(
        "Silme Durumu: {}, Mesaj: {}",
        remove_response.status, remove_response.message
    );
}
```

### Katkıda Bulunma

Katkıda bulunmak isterseniz, projeyi forklayarak kendi branch'inizde geliştirme yapabilirsiniz. Değişikliklerinizi incelemekten memnuniyet duyarız!

## Lisans

[![MIT License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

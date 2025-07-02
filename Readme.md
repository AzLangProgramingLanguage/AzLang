# 🦅 AzLang

> **Python-un sadəliyi**, **Rust-un performansı**, və **TypeScript-in type-safety gücü** ilə hazırlanmış minimalist, güclü və oxunaqlı proqramlaşdırma dili.

<p align="center">
  <img src="https://img.shields.io/badge/build-passing-blue.svg" alt="build">
  <img src="https://img.shields.io/badge/license-MIT-red.svg" alt="license">
  <img src="https://img.shields.io/badge/language-AzLang-green.svg" alt="language">
</p>

---

## 🎯 Məqsədimiz

**AzLang**, yeni başlayanlardan peşəkar tərtibatçılara qədər hər kəs üçün **əlçatan**, **anlaşılan** və **performanslı** kodlama imkanları yaradır.

Biz yeni bir proqramlaşdırma dili təqdim etmirik. Məqsədimiz — **təhlükəsiz**, **bəsit**, **oxunaqlı** və **tərcümə edilə bilən** bir dil arxitekturası təqdim etməkdir.

---

## ✨ Xüsusiyyətlər

- 🧠 **Təbii sintaksis** – Kod yazmaq insan dili qədər aydın olur
- 🛡️ **Type-Safety** – Tip yoxlamaları avtomatik aparılır, lakin zəruri hallarda əl ilə də göstərilir
- ⚡ **Performans** – Rust-vari optimallaşdırıla bilən transpiler çıxışı
- 🧪 **Statik analiz** – Tip analiz sistemi daxildə qurulub
- 🧬 **Transpiler hazırdır** – Lakin bəzi funksiyalar hələ tamamlanmayıb (mətn/siyahı funksiyaları)

---
## 🧠 Tip Sistemi

AzLang-in tip sistemi tamamilə avtomatik tip çıxarımı (type inference) ilə işləyir. Tip yazmaq optional olsa da:

    növ (enum), Obyekt və bəzi spesifik hallar üçün tip yazmaq məcburidir.

    Bu, həm oxunaqlılığı, həm də təhlükəsizliyi qoruyur.
---

## 👥 İcma və Töhfələr

Bu layihə açıq mənbəlidir. Hər bir yardım və fikir dəyərlidir:

    ✨ Yeni sintaksis təklifləri

    🧪 Bug reportlar

    📄 Sənədləşmə dəstəyi

    💻 Kod töhfəsi (Pull Request-lər açıqdır!)

---

## 🛣️ Yol Xəritəsi

    ✅ Sintaksis Dizaynı

    ✅ AST və Parser

    ✅ Tip Analizi

    ✅ Transpiler

    ❌ Funksiyonallıq

    ❌ Optimallasdırma

    ❌ Web IDE və playground 

    ❌ Rəsmi sənədlər və tutorial 📘

---

## 🔤 Sintaksisə Baxış

```azlang
funksiya salam()
    çap("Salam")
    qaytar 1

sabit a = salam()
dəyişən b = 5
çap(a)

gəz aralıq(1, 1000) içində i
    çap(i)

funksiya artir(dəyişən c: ədəd)
    c = c + 1
    əgər c == 6
        qaytar c
    yoxsa c == 7
        qaytar c
    əks
        qaytar c

sabit c = artir(b)
çap(c)

Obyekt Adam
    ad: mətn
    soyad: mətn
    yas: ədəd
    metod qeydi()
        çap(`Mən ${öz.ad} `)

sabit adam = Adam{"Prest", "Guliyev", 17}
adam.qeydi()

növ Rengler
    Qirmizi
    Yasil
    Qara

sabit reng: Rengler = Qirmizi

uyğun reng
    Qirmizi -> çap("Qirmizi")
    Yasil -> çap("Yasil")
    Qara -> çap("Qara")

sabit ad = giriş("Adınızı girin.")
sabit yas = Ədəd(giriş("Yaşınızı girin."))
çap(`Mən ${ad} ${yas} `)


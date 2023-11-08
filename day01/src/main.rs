/*
 * 12 Hello, World!
 */

fn main() {
	// eger bir ifadenin sonu '!' ile bitiyorsa, o ifade bir 'makro' dur.
	println!("Hello, World!");

	// kodumuzu rustc "file_name.rs" komutu ile derliyoruz.
}


/*
 * 13 Hello, Cargo!
 * cargo, rust'in package manager'idir.
 * projenin bagliliklarini, derlenmesini, yonetimini kolaylastirir.
 * 
 * 
 * cargo new 'project_name'
 * ile yeni bir proje olusturuyoruz.
 * eger bu komut herhangi bir git reposunun icerisinde calistirilinirsa .gitignore
 * dosyasini olusturmaz.
 * 
 * 
 * cargo.toml
 * bu dosya paketimizin ayar dosyasidir. TOML dilinde yazilmistir.
 * 
 * rust edition: rust her 6 haftada bir kucuk update yayinlar. bu update'lerin
 * belli bir sureden sonra rust'i cok fazla degistirmesinden dolayi 'edition'
 * lar yayinlanir. (bir nevi versiyon) mevcut rust edition'lari:
 * Rust 2015, Rust 2018 ve Rust 2021
 * eger toml dosyasinda herhangi bir edition versiyonu belirtilmemisse, rust
 * varsayilan olarak edition 2015 kullanir.
 * 
 * 
 * cargo build
 * bu kod ile paketimizi derliyoruz.
 * eger toml dosyasinda herhangi bir 'target' belirtilmemisse, varsayilan
 * target 'debug' tur.
 * 
 * cargo build --release
 * build target'ini 'release' olarak ayarlar.
 * 
 * 
 * cargo run
 * ile kodumuzu derledikten sonra calistiriyoruz.
 * 
 * 
 * cargo check
 * kodumuzu denetler. herhangi bir derleme islemi yapmaz.
 * 
 * 
 */
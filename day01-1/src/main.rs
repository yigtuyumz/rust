use std::io;
use rand::{self, rngs::ThreadRng, Rng};

fn main() {
	// sayi tahmin oyunu. kullanicidan alinan sayi random'a esit olmadigi surece devam eder.
	println!("Sayiyi tahmin et!");

	// bir degiskeni 'let' ile tanimliyoruz.
	// rust'ta degiskenler IMMUTABLE yani degeri degistirilemezdir. (const)
	// mut ile degiskeni 'MUTABLE' yapiyoruz.

	let mut guess : String = String::new();
	// genisleyebilir, UTF-8 string ifadesi

	// degiskenler gibi referanslar da immutable yapidadir.
	// & ile guess'i referans aliyoruz. (pointer)
	io::stdin().
		read_line(&mut guess).
		expect("Hata!!");

	println!("Tahmininiz : {guess}");

	let mut srand: ThreadRng = rand::thread_rng();
	let mut random_number: i32 = srand.gen();
	random_number %= 101;


	println!("random_number : {random_number}");


}
/*
 * cargo add 'package_name'
 * ile projeye ek paketler eklenebilir.
 * https://crates.io/ altyapisini kullanir.
 * 
 * cargo remove 'package_name'
 * ile projedeki paket kaldirilir.
 * 
 * bu proje icin
 * cargo add 'rand'
 * kullanilmistir.
 */

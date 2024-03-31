use secp256k1::{Secp256k1, SecretKey, PublicKey};
use secp256k1::Message;
use sha2::{Digest, Sha256};
use hex::encode;
use hex::FromHex;
use secp256k1::rand::rngs::OsRng;

fn main() {
    // Создаем объект secp256k1
    let secp = Secp256k1::new();
    // Генерируем серкетный и публичный ключ
    let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);
    // Переводим байты в удобный вид 
    let public_key = hex::encode(public_key.serialize_uncompressed());
    let secret_key = hex::encode(secret_key.secret_bytes());
    println!("Публичный ключ: {}", public_key);
    println!("Секретный ключ (в реальных приложениях никогда не раскрывайте секретный ключ!): {}", secret_key);

    // Сообщение для подписи. 
    // Содержание сообщения может быть любое
    let data = "Hello, World!";

    // Переводим текст в байты
    let data_bytes = data.as_bytes();
    // Генерируем хеш собщения с помощью sha256
    let message_hash = Sha256::digest(data_bytes);
    // Создаем объект Message для подписи
    let message = Message::from_digest_slice(&message_hash);
    
    // Переводим секртный ключ обратно в байты и создаем объект SecretKey
    let bytes_secret_key = Vec::from_hex(secret_key).expect("Failes to parse hex secret key");
    let secret_key = SecretKey::from_slice(&bytes_secret_key).expect("Invalid secret key format");

    // Подписываем сообщение с помощью секртного ключа
    let signature = secp.sign_ecdsa(&message.unwrap(), &secret_key);
    // Подписиь в удобном виде. Используется 16-ричная кодировка для удоной передачи данных по сети. (HEX формат)
    let signature_hex = encode(&signature.serialize_der()[..]);
    println!("Сообщение для подписи: {}", data);
    println!("Подпись: {}", signature_hex);

    // Переводим публчиный ключ обратно в байты и создаем объект PublicKey
    let bytes_public_key = hex::decode(public_key).expect("Failes to parse hex public key");
    let public_key = PublicKey::from_slice(&bytes_public_key).expect("Invalid public key format");

    // Проверяем подпись, испольльзуя публичный ключ, подпись и данные, которые були подписаны
    let is_valid = secp.verify_ecdsa(&message.unwrap(), &signature, &public_key).is_ok();
    println!("Подпись верна: {}", is_valid);
}
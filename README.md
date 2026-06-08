# Local File Encryptor

Шифрование и дешифрование файлов локально с использованием симметричного шифрования.

## Возможности
- Генерация ключа шифрования
- Шифрование любого файла
- Дешифрование обратно в исходный вид

## Запуск

### Python
```bash
pip install cryptography
# Генерация ключа (если нет)
python -c "from cryptography.fernet import Fernet; open('key.bin','wb').write(Fernet.generate_key())"
python encryptor.py encrypt secret.txt key.bin
python encryptor.py decrypt secret.txt.enc key.bin

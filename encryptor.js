const crypto = require('crypto');
const fs = require('fs');

const ALGORITHM = 'aes-256-cbc';

function generateKey() {
    return crypto.randomBytes(32);
}

function encryptFile(filePath, key, outPath = filePath + '.enc') {
    const iv = crypto.randomBytes(16);
    const cipher = crypto.createCipheriv(ALGORITHM, key, iv);
    const input = fs.createReadStream(filePath);
    const output = fs.createWriteStream(outPath);
    output.write(iv);
    input.pipe(cipher).pipe(output);
    output.on('finish', () => console.log(`Encrypted: ${outPath}`));
}

function decryptFile(encryptedPath, key, outPath = encryptedPath.replace('.enc', '')) {
    const input = fs.createReadStream(encryptedPath);
    const iv = input.read(16);
    input.pause();
    const decipher = crypto.createDecipheriv(ALGORITHM, key, iv);
    const output = fs.createWriteStream(outPath);
    input.pipe(decipher).pipe(output);
    output.on('finish', () => console.log(`Decrypted: ${outPath}`));
}

// Пример использования: node encryptor.js encrypt file.txt key.bin
const args = process.argv.slice(2);
if (args.length < 3) {
    console.log('Usage: node encryptor.js <encrypt|decrypt> <file> <keyfile>');
    process.exit(1);
}
const action = args[0];
const file = args[1];
const keyFile = args[2];
const key = fs.readFileSync(keyFile);
if (action === 'encrypt') encryptFile(file, key);
else if (action === 'decrypt') decryptFile(file, key);

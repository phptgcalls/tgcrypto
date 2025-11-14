<?php

declare(strict_types = 1);

$text = str_repeat('A',1024 * 1024);

$key = base64_decode('TlNS2AEJLK+QIklSAfFEL4guWXEqbw3f/QmlHhGOu6M=');
$iv = base64_decode('Eow6GVp/66zvfGsrmQu2ZOp6W2I8Bl7FqLGBSzJfXrM=');

$start = microtime(true);

for($i = 0;$i < 100;$i++){
	$padding = strlen($text) % 16 ? 0x10 - strlen($text) % 0x10 : 0;
	$text = str_pad($text,strlen($text) + $padding,chr(0),STR_PAD_RIGHT);

	$encryption = tg_encrypt_ige($text,$key,$iv);
	$decryption = tg_decrypt_ige($encryption,$key,$iv);

	assert($encryption === hex2bin('5c11254ddb01d59f37b71c2c12125f888d9014548582692dc4fc163a76d852d5c52733374a27ce2d3f0edef4f0722b7a32183a61b0ab4560dd91f6019ff7fa6c5c66aefe8f173053081a7b01cb50b729c32f56d7c1b4b3ccf87ebd0d96c8e076613f9fa76105ecfe2589a97fbf0f180a36908e54e680cdb8037c0fd38858b9da'));
	assert($text === $decryption);
}

$finish = microtime(true);

echo 'It took ',$finish - $start,' seconds',PHP_EOL;


?>


<?php

declare(strict_types = 1);

$text = str_repeat('A',1024 * 1024);

$key = base64_decode('TlNS2AEJLK+QIklSAfFEL4guWXEqbw3f/QmlHhGOu6M=');
$iv = base64_decode('Eow6GVp/66zvfGsrmQu2ZOp6W2I8Bl7FqLGBSzJfXrM=');

$start = microtime(true);

for($i = 0;$i < 100;$i++){
	$encryption = tg_encrypt_ige($text,$key,$iv);
	$decryption = tg_decrypt_ige($encryption,$key,$iv);

	assert(strlen($encryption) == 1024 * 1024);
	assert($text === $decryption);
}

$finish = microtime(true);

echo 'It took ' , $finish - $start , ' seconds' , PHP_EOL;

?>


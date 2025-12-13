<?php

declare(strict_types = 1);

$text = str_repeat('A',1024 * 1024);

$key = random_bytes(32);
$iv = random_bytes(32);

$start = microtime(true);

for($i = 0;$i < 1000;$i++){
	$encryption = tg_encrypt_ige($text,$key,$iv);
	$decryption = tg_decrypt_ige($encryption,$key,$iv);

	assert(strlen($encryption) == 1024 * 1024);
	assert($text === $decryption);
}

$finish = microtime(true);

echo 'It took ' , $finish - $start , ' seconds' , PHP_EOL;

?>

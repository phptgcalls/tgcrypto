<?php

declare(strict_types = 1);

$start = microtime(true);

for($i = 0;$i < 100;$i++){
	list($p,$q) = tg_factorize(0x17ED48941A08F981);
	assert($p === 0x494C553B);
	assert($q === 0x53911073);
}

$finish = microtime(true);

echo 'It took ' , $finish - $start , ' seconds' , PHP_EOL;

?>

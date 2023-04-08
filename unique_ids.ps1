cargo build
$env:RUST_BACKTRACE=1
java -jar .\maelstrom\lib\maelstrom.jar test -w unique-ids --rate 1000 --node-count 3 --time-limit 20 --availability total --log-stderr --bin .\target\debug\maelstrom_echo.exe
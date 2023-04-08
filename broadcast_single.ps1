cargo build
$env:RUST_BACKTRACE=1
java -jar .\maelstrom\lib\maelstrom.jar test -w broadcast --node-count 1 --time-limit 20 --rate 100 --log-stderr --bin .\target\debug\maelstrom_echo.exe
cargo build
$env:RUST_BACKTRACE=1
java -jar .\maelstrom\lib\maelstrom.jar test -w echo --node-count 1 --rate 1000 --time-limit 20 --log-stderr --bin .\target\debug\maelstrom_echo.exe
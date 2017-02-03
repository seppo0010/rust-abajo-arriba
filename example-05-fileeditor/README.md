# Example 5: file editor

Un dropdown con un textarea. Está hecho para correr con `example-03-webserver`
de backend y permite editar una lista de archivos.

# Instrucciones

```bash
$ cargo build --target=asmjs-unknown-emscripten
$ pushd ../example-03-webserver && cargo build && popd
$ ../example-03-webserver/target/debug/example-03-webserver &
$ firefox http://localhost:8080/index.htlm
```

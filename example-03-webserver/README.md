# Example 3: webserver

Escucha en el puerto 8080 con un web server y devuelve el contenido de los
archivos que se encuentren en la ruta de la URL. Por ejemplo, si se corre
dentro de este directorio, `http://127.0.0.1:8080/README.md` debería mostrar
este archivo.

## Instrucciones

```bash
$ cargo run &
$ curl -v localhost:8080/README.md
```

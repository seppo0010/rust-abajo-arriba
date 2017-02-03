# Example 2: microservice baas

Escucha en el puerto 12345, y lee tres tipos de mensajes:

* "cost<número>\n" para cambiar la complejidad de bcrypt. Devuelve "OK" si es
válido y crashea sino (oops).

* "hash<alfanumérico>\n" para calcular el bcrypt de <alfanumérico>. Devuelve
el string hasheado.

* "verify<alfanumérico> <hash>\n" para ver si <hash> corresponde a <alfanumérico>.
Devuelve "valid" o "invalid".

## Instrucciones

```bash
$ echo "cost7" | nc 127.0.0.1 12345
OK
$ echo "hashhello" | nc 127.0.0.1 12345
$2y$07$3F9At7aoqqiMCpuxXpXEkelbWjw950P.0G83fxKcqpaZWY671cn1u
$ echo 'verifyhello $2y$07$3F9At7aoqqiMCpuxXpXEkelbWjw950P.0G83fxKcqpaZWY671cn1u' | nc 127.0.0.1 12345
valid
$ echo 'verifyworld $2y$07$3F9At7aoqqiMCpuxXpXEkelbWjw950P.0G83fxKcqpaZWY671cn1u' | nc 127.0.0.1 12345
invalid
```

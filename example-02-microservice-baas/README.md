# Example 1: microservice revstring

Escucha en el puerto 12345, y lee tres tipos de mensajes:

* "cost<número>\n" para cambiar la complejidad de bcrypt. Devuelve "OK" si es
válido y crashea sino (oops).

* "hash<alfanumérico>\n" para calcular el bcrypt de <alfanumérico>. Devuelve
el string hasheado.

* "verify<alfanumérico> <hash>\n" para ver si <hash> corresponde a <alfanumérico>.
Devuelve "valid" o "invalid".

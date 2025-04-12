# Certificate instructions for development

From the template root...

```
openssl req -x509 -newkey ec -pkeyopt ec_paramgen_curve:prime256v1 -keyout ./crates/pha-launcher/web/certs/key.pem -out ./crates/pha-launcher/web/certs/cert.pem -days 14 -nodes -subj "/CN=localhost" -addext "subjectAltName=DNS:localhost,IP:127.0.0.1"
```

# Adaptación del toroide giratorio de C a Rust

## Descripción

Render ASCII de un toroide giratorio en terminal.
Basado en la explicación original: https://www.a1k0n.net/2011/07/20/donut-math.html

## Ejecución

```bash
cargo run --release
```

## Parámetros importantes

- `THETA_SPACING`, `PHI_SPACING`: resolución angular (calidad vs velocidad).  
- `R1`: radio del tubo (grosor).  
- `R2`: radio desde el centro del toro al centro del tubo (tamaño de anillo).  
- `K2`: desplazamiento Z (distancia del observador al toro).  
- `SLEEP_TIME`: pausa entre frames en ms.  

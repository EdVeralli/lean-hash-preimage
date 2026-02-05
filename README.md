# Ejemplo: Prueba de Preimagen de Hash

## El Problema

Querés demostrar que **conocés un secreto** sin revelarlo.

**Situación real:** Una caja fuerte tiene una combinación. Querés probar que la sabés sin decirla en voz alta.

## Cómo funciona

```
Público:  H = hash(???)     ← Todos pueden ver este número
Privado:  S = tu secreto    ← Solo vos lo sabés

Prueba ZK: "Yo sé un S tal que hash(S) = H"
```

## ¿Por qué es útil?

| Aplicación | Cómo usa esto |
|------------|---------------|
| **Contraseñas** | El servidor guarda hash(password), vos probás que conocés el password |
| **Commit-Reveal** | Primero publicás el hash, después revelás el valor |
| **Membresía anónima** | Probás que tu secreto está en una lista sin revelar cuál es |

## Diferencia con el ejemplo trivial

| Trivial | Este ejemplo |
|---------|--------------|
| Verificar `6 × 7 = 42` (cualquiera puede calcularlo) | Verificar que conocés S sin revelarlo |
| No hay secreto | El secreto S nunca se expone públicamente |

## Flujo

```
                    ┌─────────────┐
                    │   Prover    │
                    │  (tiene S)  │
                    └──────┬──────┘
                           │
          Calcula H = hash(S)
          Publica H
                           │
                           ▼
    ┌──────────────────────────────────────┐
    │           Verificador                │
    │   Solo conoce H (el hash público)    │
    └──────────────────────────────────────┘
                           │
                           │  Después...
                           ▼
                    ┌─────────────┐
                    │   Prover    │
                    │  revela S   │
                    └──────┬──────┘
                           │
                           ▼
    ┌──────────────────────────────────────┐
    │           Verificador                │
    │   Calcula hash(S) y compara con H    │
    │   Si coincide → Prover decía verdad  │
    └──────────────────────────────────────┘
```

## Limitación de este ejemplo

Esto es **commit-reveal**, no ZK completo. En ZK real:
- El prover genera una **prueba criptográfica**
- El verificador la valida **sin que el prover revele S nunca**

Para ZK completo necesitaríamos SNARK/STARK (siguiente ejemplo futuro).

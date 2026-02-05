  # Ejemplo: Prueba de Preimagen de Hash                                                                                                            
                                                                                                                                                    
  Ejemplo **introductorio** para entender el concepto de pruebas de conocimiento antes de pasar a implementaciones reales (RISC Zero, SNARKs, etc.) 
                                                                                                                                                    
  ## El Problema                                                                                                                                    
                                                                                                                                                    
  Querés demostrar que **conocés un secreto** sin revelarlo.                                                                                        
                                                                                                                                                    
  **Situación real:** Una caja fuerte tiene una combinación. Querés probar que la sabés sin decirla en voz alta.                                    
                                                                                                                                                    
  ## Cómo funciona                                                                                                                                  
                                                                                                                                                    
  Público:  H = hash(???)     ← Todos pueden ver este número                                                                                        
  Privado:  S = tu secreto    ← Solo vos lo sabés                                                                                                   
                                                                                                                                                    
  Prueba: "Yo sé un S tal que hash(S) = H"                                                                                                          
                                                                                                                                                    
  ## ¿Por qué es útil?                                                                                                                              
                                                                                                                                                    
  | Aplicación | Cómo usa esto |                                                                                                                    
  |------------|---------------|                                                                                                                    
  | **Contraseñas** | El servidor guarda hash(password), vos probás que conocés el password |                                                       
  | **Commit-Reveal** | Primero publicás el hash, después revelás el valor |                                                                        
  | **Membresía anónima** | Probás que tu secreto está en una lista sin revelar cuál es |                                                           
                                                                                                                                                    
  ## Archivos                                                                                                                                       
                                                                                                                                                    
  | Archivo | Descripción |                                                                                                                         
  |---------|-------------|                                                                                                                         
  | `HashPreimage.lean` | Especificación formal del protocolo en Lean 4 |                                                                           
  | `demo.rs` | Demo interactiva del flujo prover-verifier |                                                                                        
  | `README.md` | Este documento |                                                                                                                  
                                                                                                                                                    
  ## Requisitos                                                                                                                                     
                                                                                                                                                    
  - [Lean 4](https://lean-lang.org/) con elan (para la especificación formal)                                                                       
  - [Rust](https://rustup.rs/) (para el demo)                                                                                                       
                                                                                                                                                    
  ### Instalar Lean 4                                                                                                                               
                                                                                                                                                    
  ```bash                                                                                                                                           
  curl -sSf https://raw.githubusercontent.com/leanprover/elan/master/elan-init.sh | sh                                                              
                                                                                                                                                    
  Instalar Rust                                                                                                                                     
                                                                                                                                                    
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh                                                                                    
                                                                                                                                                    
  Ejecutar                                                                                                                                          
                                                                                                                                                    
  Demo Rust                                                                                                                                         
                                                                                                                                                    
  rustc demo.rs -o demo                                                                                                                             
  ./demo                                                                                                                                            
                                                                                                                                                    
  Especificación Lean (opcional)                                                                                                                    
                                                                                                                                                    
  Para verificar la especificación formal:                                                                                                          
                                                                                                                                                    
  # Crear proyecto Lean si no existe                                                                                                                
  lake init lean_verifier                                                                                                                           
  cp HashPreimage.lean lean_verifier/                                                                                                               
  cd lean_verifier                                                                                                                                  
  lake build                                                                                                                                        
                                                                                                                                                    
  Flujo del Protocolo                                                                                                                               
                                                                                                                                                    
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
                                                                                                                                                    
  Diferencia con ZK real                                                                                                                            
  ┌───────────────────────────────┬────────────────────────────┐                                                                                    
  │ Este ejemplo (Commit-Reveal)  │   ZK Real (SNARK/STARK)    │                                                                                    
  ├───────────────────────────────┼────────────────────────────┤                                                                                    
  │ El secreto se revela al final │ El secreto nunca se revela │                                                                                    
  ├───────────────────────────────┼────────────────────────────┤                                                                                    
  │ Verificación por comparación  │ Verificación criptográfica │                                                                                    
  ├───────────────────────────────┼────────────────────────────┤                                                                                    
  │ Simple de entender            │ Matemáticamente complejo   │                                                                                    
  └───────────────────────────────┴────────────────────────────┘                                                                                    
  Siguiente Paso                                                                                                                                    
                                                                                                                                                    
  Una vez que entiendas este concepto, mirá el ejemplo con RISC Zero que implementa ZK real:                                                        
                                                                                                                                                    
  👉 https://github.com/EdVeralli/risc0-zk-example                                                                                                  
                                                                                                                                                    
  Referencias                                                                                                                                       
                                                                                                                                                    
  - https://en.wikipedia.org/wiki/Zero-knowledge_proof                                                                                              
  - https://en.wikipedia.org/wiki/Cryptographic_hash_function                                                                                       
  ENDOFFILE                                                                                                                                         
                               


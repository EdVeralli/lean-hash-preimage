//! Demo: Prueba de Conocimiento de Preimagen de Hash
//!
//! Este ejemplo muestra cómo probar que conocés un secreto
//! sin revelarlo (hasta el momento de verificación).
//!
//! Para ejecutar: rustc demo.rs && ./demo

/// Hash simplificado (en producción sería SHA256, Poseidon, etc.)
fn secure_hash(preimage: u64) -> u64 {
    let h1 = preimage.wrapping_mul(0x517cc1b727220a95);
    let h2 = h1 ^ (h1 >> 32);
    let h3 = h2.wrapping_mul(0x94d049bb133111eb);
    h3 ^ (h3 >> 32)
}

/// Crear compromiso público
fn commit(secret: u64) -> u64 {
    secure_hash(secret)
}

/// Verificar que alguien conoce la preimagen
fn verify_preimage(public_hash: u64, claimed_secret: u64) -> bool {
    public_hash == secure_hash(claimed_secret)
}

fn main() {
    println!("═══════════════════════════════════════════════════════");
    println!("  Ejemplo: Prueba de Conocimiento de Preimagen de Hash");
    println!("═══════════════════════════════════════════════════════");
    println!();

    // ============================================
    // Escenario 1: Contraseña
    // ============================================
    println!("📋 Escenario 1: Sistema de Contraseñas");
    println!("─────────────────────────────────────────");

    let password: u64 = 123456789;

    // Registro: servidor guarda solo el hash, nunca la contraseña
    let stored_hash = commit(password);
    println!("Usuario registra password (servidor guarda hash): {}", stored_hash);
    println!("Nota: El servidor NUNCA ve la contraseña real");
    println!();

    // Login correcto
    let login_attempt = 123456789;
    let login_ok = verify_preimage(stored_hash, login_attempt);
    println!("Intento login con password correcto: {}", if login_ok { "✅ Acceso" } else { "❌ Denegado" });

    // Login incorrecto
    let wrong_attempt = 987654321;
    let login_fail = verify_preimage(stored_hash, wrong_attempt);
    println!("Intento login con password incorrecto: {}", if login_fail { "✅ Acceso" } else { "❌ Denegado" });
    println!();

    // ============================================
    // Escenario 2: Commit-Reveal (Apuesta)
    // ============================================
    println!("🎲 Escenario 2: Apuesta Justa (Commit-Reveal)");
    println!("─────────────────────────────────────────────");

    // Alice piensa un número
    let alice_number: u64 = 7;
    let alice_commitment = commit(alice_number);

    println!("Alice: 'Pensé un número. Mi compromiso es: {}'", alice_commitment);
    println!("       (Bob no puede saber qué número es)");
    println!();

    // Bob adivina
    let bob_guess: u64 = 7;
    println!("Bob: 'Adivino que es el {}'", bob_guess);
    println!();

    // Alice revela
    println!("Alice: 'Era el {}. Verificá vos mismo.'", alice_number);
    let alice_honest = verify_preimage(alice_commitment, alice_number);
    println!("Verificación: {}", if alice_honest { "✅ Alice no hizo trampa" } else { "❌ Alice mintió" });

    if bob_guess == alice_number {
        println!("Resultado: 🎉 ¡Bob adivinó!");
    } else {
        println!("Resultado: Bob no adivinó");
    }
    println!();

    // ============================================
    // Escenario 3: Prueba de Conocimiento
    // ============================================
    println!("🔐 Escenario 3: Probar que Sabés un Secreto");
    println!("────────────────────────────────────────────");

    // El "club secreto" tiene un número mágico
    let club_secret: u64 = 42424242;
    let public_hash = commit(club_secret);

    println!("Hash público del club: {}", public_hash);
    println!("(Cualquiera puede ver este hash, pero no saben el secreto)");
    println!();

    // Alguien intenta entrar con el secreto correcto
    println!("Persona 1 dice conocer el secreto...");
    let knows_secret = verify_preimage(public_hash, 42424242);
    println!("Verificación: {}", if knows_secret { "✅ Conoce el secreto, puede entrar" } else { "❌ No lo conoce" });

    // Alguien intenta entrar sin saber el secreto
    println!("Persona 2 intenta adivinar...");
    let guessing = verify_preimage(public_hash, 11111111);
    println!("Verificación: {}", if guessing { "✅ Conoce el secreto" } else { "❌ No lo conoce, acceso denegado" });
    println!();

    // ============================================
    // Resumen
    // ============================================
    println!("═══════════════════════════════════════════════════════");
    println!("  Resumen");
    println!("═══════════════════════════════════════════════════════");
    println!();
    println!("Este ejemplo demuestra COMMIT-REVEAL:");
    println!("  1. Publicás hash(secreto) → nadie sabe el secreto");
    println!("  2. Después revelás el secreto → todos verifican");
    println!();
    println!("Limitación: Eventualmente revelás el secreto.");
    println!();
    println!("En ZK REAL (SNARK/STARK):");
    println!("  - Generás una PRUEBA CRIPTOGRÁFICA");
    println!("  - El verificador la acepta");
    println!("  - NUNCA revelás el secreto");
    println!();
    println!("→ Siguiente ejemplo: ZK real con RISC Zero");
}

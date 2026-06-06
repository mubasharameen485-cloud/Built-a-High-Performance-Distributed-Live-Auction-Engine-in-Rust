    // shared_protos/build.rs

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Tonic build ko configure kar rahe hain
    tonic_build::configure()
        .compile(
            // 1. Konsi files ko translate karna hai?
            &["proto/auth.proto", "proto/auction.proto"],
            
            // 2. Yeh files kis folder mein pari hain?
            &["proto"],
        )?;
        
    Ok(())
}
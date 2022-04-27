
use rustvent::*;

fn main() -> Result<(), bool> {
    
    let mut handle = eo::event::new("xd", || {
        println!("Evento ejecutado por el suscribe");
    })?;

    handle.on();

    handle.suscribe();

    Ok(())
}

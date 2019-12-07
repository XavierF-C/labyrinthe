extern crate glium;
use glium::{glutin, Surface};// Surface est un trait et doit être importé

fn main() {

    // Initialisation des composantes graphiques principales
    let boucle_evenements = glutin::event_loop::EventLoop::new();
    let parametres_fenetre = glutin::window::WindowBuilder::new()
        .with_resizable(true)
        .with_maximized(true)
        .with_title("Labyrinthe");
    let contexte_opengl = glutin::ContextBuilder::new();
    let affichage = glium::Display::new(
            parametres_fenetre,
            contexte_opengl,
            &boucle_evenements
    ).unwrap();
    
    let debut_programme = std::time::SystemTime::now();

    boucle_evenements.run(move |evenement, _, flot_controle| {

        match evenement {

            // L'application doit se fermer suite à CloseRequested
            glutin::event::Event::WindowEvent{event: glutin::event::WindowEvent::CloseRequested, ..} => {
                
                *flot_controle = glutin::event_loop::ControlFlow::Exit;
                return;
            },

            glutin::event::Event::NewEvents(glutin::event::StartCause::ResumeTimeReached{..}) => (),
            glutin::event::Event::NewEvents(glutin::event::StartCause::Init) => (),
            // On peut procéder à l'affichage dans ces scénarios

            _ => return
        }
        // Si l'évènement est de type NewEvents(StartCause) où StartCause = {ResumeTimeReached, Init},
        // on peut rafraîchir l'affichage

        // Permet de redéclencher la boucle pour la prochaine fois
        let prochaine_date_affichage = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667); // 10^9/60
        *flot_controle = glutin::event_loop::ControlFlow::WaitUntil(prochaine_date_affichage);

        // affichage.draw() retourne un struct Frame, sur lequel on peut dessiner 
        let mut cadre = affichage.draw();

        let difference_temps = debut_programme.elapsed().unwrap().as_millis();
        let periode = 0x7FF;
        let proportion = (difference_temps & periode) as f32 / periode as f32;
        println!("{}", proportion);
        let rouge = proportion * (1.0 - proportion);
        
        cadre.clear_color(rouge, 0.0, 1.0, 1.0);
        //cadre.draw(); À compléter dès la création des shaders
        cadre.finish().unwrap();
    });
}

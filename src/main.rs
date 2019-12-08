#[macro_use]
extern crate glium;
use glium::{glutin, Surface}; // Surface est un trait et doit être importé

mod shader; // Permet de construire les shaders nécéssaires

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
    
    // À garder?
    //let debut_programme = std::time::SystemTime::now();

    let programme_opengl = shader::ProgrammeOpenGL::new(&affichage);

    // Cette closure représente la boucle principale du programme
    boucle_evenements.run(move |evenement, _, flot_controle| {

        match evenement {

            // L'application doit se fermer suite à CloseRequested
            glutin::event::Event::WindowEvent{event: glutin::event::WindowEvent::CloseRequested, ..} => {
                
                *flot_controle = glutin::event_loop::ControlFlow::Exit;
                return;
            },
            
            // On peut procéder à l'affichage dans ces scénarios
            glutin::event::Event::NewEvents(glutin::event::StartCause::ResumeTimeReached{..}) => (),
            glutin::event::Event::NewEvents(glutin::event::StartCause::Init) => (),

            // Sinon, on ignore l'évènement
            _ => return 
        }
        
        // Permet de redéclencher la boucle pour la prochaine fois
        let prochaine_date_affichage = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667); // 10^9/60
        *flot_controle = glutin::event_loop::ControlFlow::WaitUntil(prochaine_date_affichage);


        // affichage.draw() retourne un struct Frame, sur lequel on peut dessiner 
        let mut cadre = affichage.draw();
        
        cadre.clear_color(0.3, 0.3, 0.5, 1.0);
        //cadre.draw(); À compléter dès la création des shaders
        cadre.finish().unwrap();
    });
}

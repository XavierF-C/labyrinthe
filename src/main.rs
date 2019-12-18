#[macro_use]
extern crate glium;
use glium::{glutin}; // Surface est un trait et doit être importé
extern crate nalgebra_glm as glm;

mod shaders; // Permet de construire les shaders nécéssaires
mod donnees; // Permet de gérer les données associées avec OpenGL
mod ecran; // Permet de dessiner et d'interagir avec l'écran

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

    // Variables importantes pour OpenGL
    let programme_opengl = shaders::ProgrammeOpenGL::new(&affichage);
    let mut donnees_opengl = donnees::DonneesOpenGL::new();
    donnees_opengl.ajouter_triangle([
        0.0, 0.7, 0.9,
        -0.5, -0.3, 0.9,
        0.5, -0.3, 0.9,
    ]);
    donnees_opengl.ajouter_triangle([
        0.0, -0.5, 0.9,
        -0.2, -0.8, 0.9,
        0.2, -0.8, 0.9,
    ]);
    donnees_opengl.generer_vertex_buffer(&affichage);

    let mut compteur: u64 = 0;
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

        let mut z = (compteur % 320) as f32 / 720.0;
        if z >= 0.33 {
            z = 0.33
        }//let z = 0.0;
        let angle: f32 = 0.5 * (compteur as f32 / 30.0).cos();

        let vue = ecran::Vue::new(  glm::Vec3::new(0.0, 0.0, z),
                                    glm::Vec3::new(angle.sin(), 0.0, angle.cos()));
        vue.dessiner(&donnees_opengl, &programme_opengl, &affichage);

        compteur += 1;
        /*let mut cadre = affichage.draw();

        cadre.clear_color(0.3, 0.3, 0.5, 1.0);
        cadre.draw(
            donnees_opengl.obtenir_vertex_buffer(),
            &donnees_opengl.obtenir_indices(&affichage),
            &(programme_opengl.programme),
            &glium::uniforms::EmptyUniforms,
            &Default::default()
        ).unwrap();
        cadre.finish().unwrap();*/
    });
}

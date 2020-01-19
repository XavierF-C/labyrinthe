#[macro_use]
extern crate glium;
use glium::{glutin};
extern crate nalgebra_glm as glm;
extern crate image;

mod labyrinthe; // Générer le labyrinthe
mod shaders; // Construire les shaders nécéssaires
mod donnees; // Gérer les données associées avec OpenGL
mod ecran; // Dessiner et d'interagir avec l'écran
mod evenements; // Gérer le clavier, la souris, etc.
mod observateur; // Contrôler la caméra
mod textures; // Charger et utiliser des textures

fn main() {

    // Avant d'ouvrir la fenêtre, on charge les images et on crée le labyrinthe

    const BRIQUES: &str = "briques";
    const PAVEE: &str = "pavee";
    const BOIS: &str = "bois";
    const TORCHE: &str = "torche";

    let mut textures = textures::Textures::new();
    
    const EXTENSION: &str = ".png";
    textures.charger_images(&[BRIQUES, PAVEE, BOIS, TORCHE], EXTENSION);

    let labyrinthe = labyrinthe::Labyrinthe::new(12, 12);


    // Initialisation des composantes graphiques principales

    let boucle_evenements = glutin::event_loop::EventLoop::new();
    let parametres_fenetre = glutin::window::WindowBuilder::new()
        .with_resizable(true)
        .with_maximized(true)
        .with_title("Labyrinthe");
    let contexte_opengl = glutin::ContextBuilder::new()
        .with_vsync(true)    
        .with_depth_buffer(24);
        // 24 bits est un choix commun pour le depth buffer
    let affichage = glium::Display::new(
            parametres_fenetre,
            contexte_opengl,
            &boucle_evenements
        ).unwrap();
    
    textures.generer_textures(&affichage);
    
    
    // Variables importantes pour OpenGL

    let programme_opengl = shaders::ProgrammeOpenGL::new(&affichage);
    let mut donnees_opengl = donnees::DonneesOpenGL::new();
    labyrinthe.ajouter_geometrie(
        [1.0, 1.0, textures.obtenir_id(BOIS)],
        [2.0, 2.0, textures.obtenir_id(PAVEE)],
        [2.0, 2.0, textures.obtenir_id(BRIQUES)],
        textures.obtenir_id(TORCHE),
        &mut donnees_opengl);

    donnees_opengl.generer_vertex_buffer(&affichage);
    
    let mut vue = ecran::Vue::new();
    
    
    // Variables utiles à la logique du programme

    let mut gestionnaire_evenements = evenements::GestionnaireEvenements::new(&affichage);

    let mut observateur = observateur::Observateur::new(
        glm::Vec3::new(0.0, 1.5, 0.0),
        glm::Vec3::new(0.0, 0.0, 1.0),
    );

    const TAUX_RAFRAICHISSEMENT: u64 = 60;
    
    // Cette closure représente la boucle principale du programme
    boucle_evenements.run(move |evenement, _, flot_controle| {

        match evenement {

            // L'application doit se fermer suite à CloseRequested
            glutin::event::Event::WindowEvent{event: glutin::event::WindowEvent::CloseRequested, ..} => {
                
                *flot_controle = glutin::event_loop::ControlFlow::Exit;
                return;
            },

            glutin::event::Event::WindowEvent{event: evenement, ..} => {
                
                gestionnaire_evenements.gerer_evenement_fenetre(&evenement);
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
            std::time::Duration::from_nanos(1_000_000_000 / TAUX_RAFRAICHISSEMENT);
        *flot_controle = glutin::event_loop::ControlFlow::WaitUntil(prochaine_date_affichage);


        // Logique du programme

        observateur.deplacer(&gestionnaire_evenements, TAUX_RAFRAICHISSEMENT);
        labyrinthe.expulser_murs(&mut observateur);

        if gestionnaire_evenements.souris.mode_centre {
            observateur.ajuster_direction(&gestionnaire_evenements, TAUX_RAFRAICHISSEMENT);
        }
        
        gestionnaire_evenements.mise_a_jour_post_logique(&affichage);

        // Affichage du programme
        vue.changer_camera(&observateur.position, observateur.direction());

        let lumieres = labyrinthe.obtenir_lumieres_proches(&observateur);
        vue.dessiner(lumieres, &donnees_opengl, &programme_opengl, &affichage);
    });
}
#[macro_use]
extern crate glium;
use glium::{glutin}; // Surface est un trait et doit être importé
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

    let mut textures = textures::Textures::new();
    const BRIQUES: &str = "briques";
    const PAVEE: &str = "pavee";
    const BOIS: &str = "bois";
    const TORCHE: &str = "torche";
    textures.charger_image("briques.png", BRIQUES);
    textures.charger_image("pavee.png", PAVEE);
    textures.charger_image("bois.png", BOIS);
    textures.charger_image("torche.png", TORCHE);

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

    
    // Variables utiles à la logique du programme
    let mut gestionnaire_evenements = evenements::GestionnaireEvenements::new(&affichage);

    let mut observateur = observateur::Observateur::new(
        glm::Vec3::new(0.0, 0.0, 0.0),
        glm::Vec3::new(0.0, 0.0, 1.0),
    );

    let mut doit_centrer_souris = false;

    const TAUX_RAFRAICHISSEMENT: u64 = 60;
    let mut compteur: u64 = 0;
    
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
        const VITESSE: f32 = 1.25 / TAUX_RAFRAICHISSEMENT as f32;

        if gestionnaire_evenements.clavier.est_appuyee(&glutin::event::VirtualKeyCode::A) {
            observateur.position -= observateur.droite() * VITESSE;
        }
        if gestionnaire_evenements.clavier.est_appuyee(&glutin::event::VirtualKeyCode::D) {
            observateur.position += observateur.droite() * VITESSE;
        }
        if gestionnaire_evenements.clavier.est_appuyee(&glutin::event::VirtualKeyCode::S) {
            observateur.position -= observateur.direction() * VITESSE;
        }
        if gestionnaire_evenements.clavier.est_appuyee(&glutin::event::VirtualKeyCode::W) {
            observateur.position += observateur.direction() * VITESSE;
        }
        if gestionnaire_evenements.clavier.est_appuyee(&glutin::event::VirtualKeyCode::LShift) {
            observateur.position -= observateur.haut() * VITESSE;
        }
        if gestionnaire_evenements.clavier.est_appuyee(&glutin::event::VirtualKeyCode::Space) {
            observateur.position += observateur.haut() * VITESSE;
        }
        labyrinthe.expulser_murs(&mut observateur);
        observateur.position.y = 1.5;

        if doit_centrer_souris {
            observateur.ajuster_direction(
                gestionnaire_evenements.souris.delta_x() / 1000.0,
                gestionnaire_evenements.souris.delta_y() / 1000.0,
            );
            
           gestionnaire_evenements.souris.centrer(&affichage);
        }
        
        if gestionnaire_evenements.clavier.vient_etre_appuyee(&glutin::event::VirtualKeyCode::Escape) {
            
            doit_centrer_souris = !doit_centrer_souris;
            affichage.gl_window().window().set_cursor_visible(!doit_centrer_souris);

            let mut plein_ecran = None;

            if doit_centrer_souris {
                
                plein_ecran = Some(glutin::window::Fullscreen::Borderless(
                    affichage.gl_window().window().current_monitor()));
            }

            affichage.gl_window().window().set_fullscreen(plein_ecran);
        }
        
        gestionnaire_evenements.mise_a_jour_post_logique();

        // Affichage du programme
        let vue = ecran::Vue::new(  &observateur.position,
                                    observateur.direction());

        let lumieres = labyrinthe.obtenir_lumieres_proches(&observateur);
        vue.dessiner(lumieres, &donnees_opengl, &programme_opengl, &affichage);

        compteur += 1;
    });
}
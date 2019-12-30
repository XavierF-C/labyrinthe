use glium::{glutin};

/*
    Interface publique du module evenements

    Sert à gérer tous les événèments importants tels que
    l'état du clavier et l'état de la souris
*/

pub struct GestionnaireEvenements {
    pub clavier: Clavier,
    pub souris: Souris,
}

impl GestionnaireEvenements {

    pub fn new() -> GestionnaireEvenements {

        GestionnaireEvenements {
            clavier: Clavier::new(),
            souris: Souris::new(),
        }
    }

    pub fn gerer_evenement_fenetre(&mut self, evenement: &glutin::event::WindowEvent) {

        match evenement {
    
            glutin::event::WindowEvent::KeyboardInput{input: entree, ..} => {
                
                if let Some(touche) = entree.virtual_keycode {
                    self.clavier.mise_a_jour_touche(&touche, &entree.state);
                }
            },

            glutin::event::WindowEvent::CursorMoved{position, ..} => {
                
                self.souris.mise_a_jour(*position);
            },
    
            _ => return, // Sinon, rien à faire
        }
    }

    // Cette méthode devrait être appelée à la fin de la logique du programme
    pub fn mise_a_jour_post_logique(&mut self) {

        self.clavier.mise_a_jour_changement_etat();
    }
}

// Mémorise l'état des touches
pub struct Clavier {

    etats_touches: std::collections::HashMap<glutin::event::VirtualKeyCode, EtatTouche>,
}

impl Clavier {

    pub fn new() -> Clavier {

        let mut clavier = Clavier {
            etats_touches: std::collections::HashMap::new(),
        };

        clavier.ajouter_touches_base();

        clavier
    }

    pub fn ajouter_touche(&mut self, touche: glutin::event::VirtualKeyCode) {

        self.etats_touches.insert(touche, EtatTouche::Inconnue);
    }

    pub fn est_appuyee(&self, touche: &glutin::event::VirtualKeyCode) -> bool {

        if let Some(etat_touche) = self.etats_touches.get(touche) {

            match etat_touche {
                EtatTouche::Appuyee{..} => return true,
                _ => return false,
            };
        }

        false
    }

    pub fn vient_etre_appuyee(&self, touche: &glutin::event::VirtualKeyCode) -> bool {

        if let Some(etat_touche) = self.etats_touches.get(touche) {

            match etat_touche {
                EtatTouche::Appuyee{vient_etre_appuyee} => return *vient_etre_appuyee,
                _ => return false,
            };
        }

        false
    }

    fn mise_a_jour_touche(&mut self, touche: &glutin::event::VirtualKeyCode, etat_actuel: &glutin::event::ElementState) {

        // Si la touche est répertoriée, on change son état
        if let Some(etat_touche) = self.etats_touches.get_mut(touche) {

            etat_touche.changer_etat(&etat_actuel);
        }
    }

    // Mettre à jour le fait que la touche ne vient plus d'être appuyée/relâchée
    fn mise_a_jour_changement_etat(&mut self) {

        for (_clef, etat) in self.etats_touches.iter_mut() {

            *etat = match etat {

                EtatTouche::Appuyee{..} => EtatTouche::Appuyee{vient_etre_appuyee: false},
                EtatTouche::Relachee{..} => EtatTouche::Relachee{vient_etre_relache: false},
                _ => EtatTouche::Inconnue,
            }
        }
    }

    fn ajouter_touches_base(&mut self) {

        self.ajouter_touche(glutin::event::VirtualKeyCode::A);
        self.ajouter_touche(glutin::event::VirtualKeyCode::D);
        self.ajouter_touche(glutin::event::VirtualKeyCode::S);
        self.ajouter_touche(glutin::event::VirtualKeyCode::W);
        self.ajouter_touche(glutin::event::VirtualKeyCode::LShift);
        self.ajouter_touche(glutin::event::VirtualKeyCode::Space);
        self.ajouter_touche(glutin::event::VirtualKeyCode::Escape);
    }
}

pub struct Souris {

    position_actuelle: glutin::dpi::LogicalPosition,
    position_origine: glutin::dpi::LogicalPosition,
}

impl Souris {

    pub fn new() -> Souris {

        Souris{
            position_actuelle: glutin::dpi::LogicalPosition::new( 0.0, 0.0),
            position_origine: glutin::dpi::LogicalPosition::new( 0.0, 0.0),
        }
    }

    pub fn delta_x(&self) -> f32 {

        Souris::stabiliser_delta(self.position_actuelle.x - self.position_origine.x)
    }

    pub fn delta_y(&self) -> f32 {
        // L'axe y de l'écran pointe vers le bas; on l'inverse
        Souris::stabiliser_delta(self.position_origine.y - self.position_actuelle.y)
    }

    pub fn centrer(&mut self, affichage: &glium::Display) {

        if let Ok(position) = affichage.gl_window().window().inner_position() {
            
            let dimensions = affichage.gl_window().window().inner_size();
            let centre = glutin::dpi::LogicalPosition::new(
                position.x + dimensions.width / 2.0,
                position.y + dimensions.height / 2.0);

            let _ = affichage.gl_window().window().set_cursor_position(centre);

            //self.mise_a_jour(centre);
            self.position_origine = centre;
            //self.position_actuelle = centre;
        }
    }

    fn mise_a_jour(&mut self, position: glutin::dpi::LogicalPosition) {

        //self.position_precedente = self.position_actuelle;
        self.position_actuelle = position;
    }


    // Cette fonction empêche la souris de se déplacer lentement à cause d'un delta minuscule
    fn stabiliser_delta(delta: f64) -> f32 {

        const SEUIL: f64 = 0.5;

        if delta >= -SEUIL && delta <= SEUIL {

            return 0.0;
        }

        delta as f32
    }
}


/*
    Partie privée du module evenements
*/

enum EtatTouche {

    Appuyee {
        vient_etre_appuyee: bool
    },
    Relachee {
        vient_etre_relache: bool
    },
    Inconnue,
}

impl EtatTouche {

    fn changer_etat(&mut self, etat_actuel: &glutin::event::ElementState) {

        *self = match etat_actuel {
                
            glutin::event::ElementState::Pressed => {

                let etat_change = match self {
                    EtatTouche::Appuyee{vient_etre_appuyee} => *vient_etre_appuyee,
                    _ => true,
                };
                
                EtatTouche::Appuyee{vient_etre_appuyee: etat_change}
            },
            
            glutin::event::ElementState::Released => {

                let etat_change = match self {
                    EtatTouche::Relachee{vient_etre_relache} => *vient_etre_relache,
                    _ => true,
                };

                EtatTouche::Relachee{vient_etre_relache: etat_change}
            },
        };
    }
    
}
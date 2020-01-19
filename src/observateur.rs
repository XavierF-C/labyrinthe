extern crate nalgebra_glm as glm;
use glium::{glutin};

use evenements;

/*
    Interface publique du module observateur

    Permet de gérer les paramètres de la caméra
*/

#[derive(Clone)]
pub struct Observateur {

    pub position: glm::Vec3,
    
    deplacement: Deplacement,
    regard: Regard,

    // Vecteurs d'orientation
    direction: glm::Vec3,
    droite: glm::Vec3,
    haut: glm::Vec3,
}

impl Observateur {

    pub fn new(position: glm::Vec3, direction: glm::Vec3) -> Observateur {

        // Valeurs arbitraires, seulement là pour être initialisées
        let mut droite = glm::Vec3::new(0.0, 0.0, 0.0);
        let mut haut = glm::Vec3::new(0.0, 0.0, 0.0);
        Observateur::calculer_haut_droite(&direction, &mut droite, &mut haut);

        Observateur {
            
            position: position,
            
            deplacement: Deplacement::new(),
            regard: Regard::new(),

            direction: direction,
            droite: droite,
            haut: haut,
        }
    }

    pub fn deplacer(&mut self, gestionnaire_evenements: &evenements::GestionnaireEvenements, taux_rafraichissement: u64) {
        
        self.deplacement.ajuster_vitesse(&self.clone(), gestionnaire_evenements, taux_rafraichissement);
        self.position += self.deplacement.delta(taux_rafraichissement);
    }

    pub fn ajuster_direction(&mut self, gestionnaire_evenements: &evenements::GestionnaireEvenements, taux_rafraichissement: u64) {
        
        self.regard.ajuster_direction(gestionnaire_evenements, taux_rafraichissement);

        self.nouvelle_direction(self.regard.angles.obtenir_direction());
    }

    pub fn direction(&self) -> &glm::Vec3 {
        &self.direction
    }

    pub fn haut(&self)  -> &glm::Vec3 {
        &self.haut
    }

    pub fn droite(&self)  -> &glm::Vec3 {
        &self.droite
    }

    fn nouvelle_direction(&mut self, direction: glm::Vec3) {

        self.direction = direction;
        
        Observateur::calculer_haut_droite(&direction, &mut self.droite, &mut self.haut);
    }

    fn calculer_haut_droite(direction: &glm::Vec3, droite: &mut glm::Vec3, haut: &mut glm::Vec3) {

        let haut_temporaire = glm::Vec3::new(0.0, 1.0, 0.0);
        
        *droite = glm::normalize(&haut_temporaire.cross(&direction));
        *haut = glm::normalize(&direction.cross(&droite));
    }
}





/*
    Partie privée du module observateur
*/

// Permet de fournir le déplacement selon les entrées du clavier
#[derive(Clone)]
struct Deplacement {

    pub vitesse: glm::Vec3,
    mode_aerien: bool,
}

impl Deplacement {

    pub fn new() -> Deplacement {

        Deplacement {
            vitesse: glm::Vec3::new(0.0, 0.0, 0.0),
            mode_aerien: false,
        }
    }

    pub fn ajuster_vitesse(
        &mut self,
        observateur: &Observateur,
        gestionnaire_evenements: &evenements::GestionnaireEvenements,
        taux_rafraichissement: u64) {

        let vitesse = 1.25;
        let mut cible = glm::Vec3::new(0.0, 0.0, 0.0);

        if gestionnaire_evenements.clavier.est_appuyee(&glutin::event::VirtualKeyCode::A) {
            cible -= observateur.droite();
        }
        if gestionnaire_evenements.clavier.est_appuyee(&glutin::event::VirtualKeyCode::D) {
            cible += observateur.droite();
        }
        if gestionnaire_evenements.clavier.est_appuyee(&glutin::event::VirtualKeyCode::S) {
            cible -= observateur.direction();
        }
        if gestionnaire_evenements.clavier.est_appuyee(&glutin::event::VirtualKeyCode::W) {
            cible += observateur.direction();
        }

        if self.mode_aerien {

            if gestionnaire_evenements.clavier.est_appuyee(&glutin::event::VirtualKeyCode::LShift) {
                cible -= observateur.haut();
            }
            if gestionnaire_evenements.clavier.est_appuyee(&glutin::event::VirtualKeyCode::Space) {
                cible += observateur.haut();
            }
        }

        if cible == glm::Vec3::new(0.0, 0.0, 0.0) {
            self.ralentir(taux_rafraichissement);
        }
        else {

            cible = cible.normalize() * vitesse;
            self.accelerer_vers(&cible, taux_rafraichissement);
        }
    }

    pub fn delta(&self, taux_rafraichissement: u64) -> glm::Vec3 {
        
        if self.mode_aerien {
            return self.vitesse / taux_rafraichissement as f32;
        }

        glm::Vec3::new(self.vitesse.x, 0.0, self.vitesse.z) / taux_rafraichissement as f32
    }

    fn ralentir(&mut self, taux_rafraichissement: u64) {

        let ralentissement = 1.0 - (10.0 / taux_rafraichissement as f32);
        self.vitesse = self.vitesse * ralentissement;
    }

    fn accelerer_vers(&mut self, cible: &glm::Vec3, taux_rafraichissement: u64) {

        let proportion_cible = 20.0 / taux_rafraichissement as f32;
        self.vitesse = cible * proportion_cible + self.vitesse * (1.0 - proportion_cible);
    }
}

// Représente les angles d'une direction
#[derive(Clone)]
struct Angles {

    pub angle_xz: f32, // gauche-droite
    pub angle_yz: f32, // bas-haut
}

impl Angles {

    pub fn new() -> Angles {

        Angles{
            angle_xz: 0.0,
            angle_yz: 0.0,
        }
    }

    pub fn modifier(&mut self, angle_xz: f32, angle_yz: f32) {

        self.angle_xz = angle_xz;
        self.angle_yz = angle_yz;
    }

    pub fn ajouter(&mut self, delta_angle_xz: f32, delta_angle_yz: f32) {

        self.angle_xz += delta_angle_xz;
        self.angle_yz += delta_angle_yz;
    }

    pub fn obtenir_direction(&self) -> glm::Vec3{

        glm::Vec3::new(
            self.angle_xz.sin() * self.angle_yz.cos(),
            self.angle_yz.sin(),
            self.angle_xz.cos() * self.angle_yz.cos()
        )
    }

    pub fn maintenir_angles(&mut self) {

        // Maintenir l'angle xz entre [-PI, PI]
        if self.angle_xz < -std::f32::consts::PI {
            self.angle_xz += 2.0 * std::f32::consts::PI;
        }
        if self.angle_xz > std::f32::consts::PI {
            self.angle_xz -= 2.0 * std::f32::consts::PI;
        }

        // Restreindre l'angle yz entre [-LIMITE_ANGLE_YZ, LIMITE_ANGLE_YZ]
        const LIMITE_ANGLE_YZ: f32 = 0.35 * std::f32::consts::PI;
        if self.angle_yz < -LIMITE_ANGLE_YZ {
            self.angle_yz = -LIMITE_ANGLE_YZ;
        }
        if self.angle_yz > LIMITE_ANGLE_YZ {
            self.angle_yz = LIMITE_ANGLE_YZ;
        }
    }
}

// Permet l'ajustement des angles selon les mouvements de la souris
#[derive(Clone)]
struct Regard {

    pub angles: Angles,
    vitesse_angles: Angles,
}

impl Regard {

    pub fn new() -> Regard {

        Regard {

            angles: Angles::new(),
            vitesse_angles: Angles::new(),
        }
    }

    pub fn ajuster_direction(&mut self, gestionnaire_evenements: &evenements::GestionnaireEvenements, taux_rafraichissement: u64) {

        let taux_rafraichissement = taux_rafraichissement as f32;
        const SENSABILITE: f32 = 0.04;
        const VITESSE_STABILISATION: f32 = 20.0;

        let mut vitesse_angle_xz = SENSABILITE * gestionnaire_evenements.souris.delta_x();
        let mut vitesse_angle_yz = SENSABILITE * gestionnaire_evenements.souris.delta_y();

        self.vitesse_angles.ajouter(
            -VITESSE_STABILISATION * self.vitesse_angles.angle_xz / taux_rafraichissement,
            -VITESSE_STABILISATION * self.vitesse_angles.angle_yz / taux_rafraichissement);        

        if vitesse_angle_xz.abs() < self.vitesse_angles.angle_xz.abs() {
            vitesse_angle_xz = self.vitesse_angles.angle_xz;
        }
        if vitesse_angle_yz.abs() < self.vitesse_angles.angle_yz.abs() {
            vitesse_angle_yz = self.vitesse_angles.angle_yz;
        }

        self.vitesse_angles.modifier(vitesse_angle_xz, vitesse_angle_yz);

        self.angles.ajouter(
            self.vitesse_angles.angle_xz / taux_rafraichissement,
            self.vitesse_angles.angle_yz / taux_rafraichissement);
        self.angles.maintenir_angles();
    }
}
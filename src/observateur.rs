extern crate nalgebra_glm as glm;

/*
    Interface publique du module observateur

    Permet de gérer les paramètres de la caméra
*/

pub struct Observateur {

    pub position: glm::Vec3,
    
    angles: Angles, // représente la direction avec des angles
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
            
            angles: Angles::new(),
            direction: direction,
            droite: droite,
            haut: haut,
        }
    }

    pub fn changer_direction(&mut self, angle_xz: f32, angle_yz: f32) {

        self.angles.modifier(angle_xz, angle_yz);

        self.nouvelle_direction(self.angles.obtenir_direction());
    }

    pub fn ajuster_direction(&mut self, angle_delta_xz: f32, angle_delta_yz: f32) {

        let angle_xz = self.angles.angle_xz + angle_delta_xz;
        let angle_yz = self.angles.angle_yz + angle_delta_yz;

        self.angles.modifier(
            angle_xz,
            angle_yz,
        );

        self.nouvelle_direction(self.angles.obtenir_direction());
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

    pub fn obtenir_direction(&self) -> glm::Vec3{

        glm::Vec3::new(
            self.angle_xz.sin() * self.angle_yz.cos(),
            self.angle_yz.sin(),
            self.angle_xz.cos() * self.angle_yz.cos()
        )
    }
}
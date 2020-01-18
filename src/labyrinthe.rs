extern crate rand;

use donnees;
use observateur;
use ecran;

/*
    Interface publique du module labyrinthe

    Sert à générer le labyrinthe
*/

pub struct Labyrinthe {

    longueur: u32,
    largeur: u32,
    decalage: [f32; 3],

    hauteur: f32, // hauteur d'une cellule
    cote: f32, // longueur et largeur d'une cellule
    cellules: std::vec::Vec<std::vec::Vec<Cellule>>,
    lumieres: std::vec::Vec<Lumiere>,
}

impl Labyrinthe {

    pub fn new(longueur: u32, largeur: u32) -> Labyrinthe{

        let mut cellules = std::vec::Vec::with_capacity(largeur as usize);

        for z in 0..largeur {

            let mut rangee = std::vec::Vec::with_capacity(longueur as usize);

            for x in 0..longueur {
                
                rangee.push(Cellule::new(x as u32, z as u32));
            }

            cellules.push(rangee);
        }

        const HAUTEUR: f32 = 2.0;
        const COTE: f32 = 1.0;
        let mut labyrinthe = Labyrinthe {

            longueur: longueur,
            largeur: largeur,
            decalage: [
                -(longueur as f32) * COTE / 2.0,
                0.0,
                -(largeur as f32) * COTE / 2.0
            ],
            
            hauteur: HAUTEUR,
            cote: COTE,
            cellules: cellules,
            lumieres: std::vec::Vec::new(),
        };

        labyrinthe.detruire_murs();
        labyrinthe.enlever_murs_inutiles();
        labyrinthe.ajouter_lumieres();

        labyrinthe
    }

    // texture: longueur, hauteur, id 
    pub fn ajouter_geometrie(&self,
        texture_plafond: [f32; 3],
        texture_sol: [f32; 3],
        texture_mur: [f32; 3],
        texture_torche: f32,
        donnees_opengl: &mut donnees::DonneesOpenGL) {

        let hauteur = self.hauteur;
        let cote = self.cote;
        let decalage = self.decalage;

        // Ajouter tous les murs
        for z in 0..self.largeur {

            for x in 0..self.longueur {
                
                let position = Position::new(x, z);
                self.lire_cellule(&position).ajouter_geometrie(hauteur, cote, &decalage, &texture_mur, donnees_opengl);
            }
        }

        const TRIANGLES_PAR_UNITE: u32 = 4;

        // Ajoute le plancher
        donnees_opengl.ajouter_plan(
            [self.longueur * TRIANGLES_PAR_UNITE, self.largeur * TRIANGLES_PAR_UNITE],
            [decalage[0], decalage[1], decalage[2]],
            [decalage[0], decalage[1], decalage[2] + cote * self.largeur as f32],
            [decalage[0] + cote * self.longueur as f32, decalage[1], decalage[2]],
            [texture_sol[0] * self.longueur as f32, texture_sol[1] * self.largeur as f32, texture_sol[2]]
        );
        
        // Ajoute le plafond
        donnees_opengl.ajouter_plan(
            [self.longueur * TRIANGLES_PAR_UNITE, self.largeur * TRIANGLES_PAR_UNITE],
            [decalage[0] + cote * self.longueur as f32, decalage[1] + hauteur, decalage[2] + cote * self.largeur as f32],
            [decalage[0], decalage[1] + hauteur, decalage[2] + cote * self.largeur as f32],
            [decalage[0] + cote * self.longueur as f32, decalage[1] + hauteur, decalage[2]],
            [texture_plafond[0] * self.longueur as f32, texture_plafond[1] * self.largeur as f32, texture_plafond[2]]
        );

        // Ajoute le mur gauche
        donnees_opengl.ajouter_plan(
            [self.longueur * TRIANGLES_PAR_UNITE, self.largeur * TRIANGLES_PAR_UNITE],
            [decalage[0], decalage[1], decalage[2]],
            [decalage[0], decalage[1] + hauteur, decalage[2]],
            [decalage[0], decalage[1], decalage[2] + cote * self.largeur as f32],
            [texture_mur[0] * self.longueur as f32, texture_mur[1], texture_mur[2]]
        );

        // Ajoute le mur bas
        donnees_opengl.ajouter_plan(
            [self.longueur * TRIANGLES_PAR_UNITE, self.largeur * TRIANGLES_PAR_UNITE],
            [decalage[0] + cote * self.longueur as f32, decalage[1], decalage[2]],
            [decalage[0] + cote * self.longueur as f32, decalage[1] + hauteur, decalage[2]],
            [decalage[0], decalage[1], decalage[2]],
            [texture_mur[0] * self.longueur as f32, texture_mur[1], texture_mur[2]]
        );

        // Ajoute le mur droit
        donnees_opengl.ajouter_plan(
            [self.longueur * TRIANGLES_PAR_UNITE, self.largeur * TRIANGLES_PAR_UNITE],
            [decalage[0] + cote * self.longueur as f32, decalage[1], decalage[2] + cote * self.largeur as f32],
            [decalage[0] + cote * self.longueur as f32, decalage[1] + hauteur, decalage[2] + cote * self.largeur as f32],
            [decalage[0] + cote * self.longueur as f32, decalage[1], decalage[2]],
            [texture_mur[0] * self.longueur as f32, texture_mur[1], texture_mur[2]]
        );

        // Ajoute le mur haut
        donnees_opengl.ajouter_plan(
            [self.longueur * TRIANGLES_PAR_UNITE, self.largeur * TRIANGLES_PAR_UNITE],
            [decalage[0], decalage[1], decalage[2] + cote * self.largeur as f32],
            [decalage[0], decalage[1] + hauteur, decalage[2] + cote * self.largeur as f32],
            [decalage[0] + cote * self.longueur as f32, decalage[1], decalage[2] + cote * self.largeur as f32],
            [texture_mur[0] * self.longueur as f32, texture_mur[1], texture_mur[2]]
        );

        // Ajoute les torches
        for i in 0..self.lumieres.len() {

            self.lumieres[i].ajouter_geometrie(texture_torche, donnees_opengl);
        }
    }

    pub fn expulser_murs(&self, observateur: &mut observateur::Observateur) {

        // Position de la cellule la plus centrée sur l'observateur
        let x_observateur = ((observateur.position.x - self.decalage[0]) / self.cote).round() as i32;
        let z_observateur = ((observateur.position.z - self.decalage[2]) / self.cote).round() as i32;

        for x in -1..2 as i32 {

            for z in -1..2 as i32{

                let verifier_collision = match self.essayer_cellule((x + x_observateur, z + z_observateur)) {

                    Some(cellule) => !cellule.est_un_sentier(),
                    None => true, // Si la cellule n'existe pas, c'est à l'extérieur du labyrinthe
                };

                if verifier_collision {

                    let ecart: f32 = self.cote * 0.2;

                    let x_gauche = (((x + x_observateur) as f32) * self.cote) + self.decalage[0] as f32 - ecart;
                    let x_droit = x_gauche + self.cote + ecart + ecart;
                    let z_bas = (((z + z_observateur) as f32) * self.cote) + self.decalage[2] as f32 - ecart;
                    let z_haut = z_bas + self.cote + ecart + ecart;

                    if x_gauche <= observateur.position.x && observateur.position.x <= x_droit &&
                        z_bas <= observateur.position.z && observateur.position.z <= z_haut {
                        
                        let min = |a: f32, b: f32| -> f32 {if a < b {return a;} b};
                        
                        if min(observateur.position.x - x_gauche, x_droit - observateur.position.x) <
                            min(observateur.position.z - z_bas, z_haut - observateur.position.z) {

                            if observateur.position.x - x_gauche < self.cote / 2.0 {
                                observateur.position.x = x_gauche;
                            }
                            else {
                                observateur.position.x = x_droit;
                            }
                        }
                        else {

                            if observateur.position.z - z_bas < self.cote / 2.0 {
                                observateur.position.z = z_bas;
                            }
                            else {
                                observateur.position.z = z_haut;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn obtenir_lumieres_proches(&self, observateur: &observateur::Observateur) -> ecran::Lumieres {

        const INFINI: f32 = 1000000.0;

        let position = [observateur.position.x, observateur.position.y, observateur.position.z];

        struct LumiereProche {
            pub distance: f32,
            pub index: u32,
        }

        impl LumiereProche {
            pub fn new() -> LumiereProche {
                LumiereProche {
                    distance: INFINI,
                    index: 0,
                }
            }
        }

        let mut lumieres_proches = std::vec::Vec::with_capacity(ecran::NOMBRE_LUMIERES);

        const NOMBRE_LUMIERES: usize = ecran::NOMBRE_LUMIERES - 1;// -1 pour laisser une lumière de vision à l'observateur

        for _ in 0..NOMBRE_LUMIERES {
            lumieres_proches.push(LumiereProche::new());
        }

        for index in 0..self.lumieres.len() {

            let mut lumiere_plus_distante = LumiereProche{distance: 0.0, index: 0};
            
            for i in 0..std::cmp::min(self.lumieres.len(), NOMBRE_LUMIERES) {

                if lumieres_proches[i].distance >= lumiere_plus_distante.distance {

                    lumiere_plus_distante.distance = lumieres_proches[i].distance;
                    lumiere_plus_distante.index = i as u32;
                } 
            }

            let dx = position[0] - self.lumieres[index].position[0];
            let dy = position[1] - self.lumieres[index].position[1];
            let dz = position[2] - self.lumieres[index].position[2];
            let distance_actuelle = dx*dx + dy*dy + dz* dz;
            
            if distance_actuelle <= lumiere_plus_distante.distance {

                lumieres_proches[lumiere_plus_distante.index as usize].distance = distance_actuelle;
                lumieres_proches[lumiere_plus_distante.index as usize].index = index as u32;
            }
        }

        let mut lumieres = ecran::Lumieres::new();

        for i in 0..std::cmp::min(self.lumieres.len(), NOMBRE_LUMIERES) {

            lumieres.positions[i] = self.lumieres[lumieres_proches[i].index as usize].position;
            lumieres.couleurs[i] = self.lumieres[lumieres_proches[i].index as usize].couleur;
        }

        const VISION: f32 = 0.5;
        lumieres.positions[NOMBRE_LUMIERES] = [observateur.position.x, observateur.position.y, observateur.position.z, 1.0];
        lumieres.couleurs[NOMBRE_LUMIERES] = [VISION, VISION + 0.02, VISION + 0.04, 1.0];

        lumieres
    }

    fn detruire_murs(&mut self) {

        let position_depart = self.position_aleatoire();

        // sentiers desquels on peut potentiellement ouvrir un sentier adjacent
        let mut sentiers_explorables = std::vec::Vec::new();
        sentiers_explorables.push((position_depart.x, position_depart.z));

        // sentiers adjacents au sentier choisi, qui peuvent être ouverts
        let mut sentiers_a_ouvrir_possibles = std::vec::Vec::with_capacity(4);
        
        while sentiers_explorables.len() > 0 {

            let choix_sentier = entier_aleatoire(sentiers_explorables.len() as u32) as usize;

            let position_courante = sentiers_explorables[choix_sentier];

            let gauche = (position_courante.0 as i32 - 1, position_courante.1 as i32);
            let haut = (position_courante.0 as i32, position_courante.1 as i32 + 1);
            let droit = (position_courante.0 as i32 + 1, position_courante.1 as i32);
            let bas = (position_courante.0 as i32, position_courante.1 as i32 - 1);
            
            if self.peut_ouvrir_sentier(gauche) {
                sentiers_a_ouvrir_possibles.push(gauche);
            }
            if self.peut_ouvrir_sentier(haut) {
                sentiers_a_ouvrir_possibles.push(haut);
            }
            if self.peut_ouvrir_sentier(droit) {
                sentiers_a_ouvrir_possibles.push(droit);
            }
            if self.peut_ouvrir_sentier(bas) {
                sentiers_a_ouvrir_possibles.push(bas);
            }

            if sentiers_a_ouvrir_possibles.len() == 0 { // Le sentier n'a plus de débouché
                sentiers_explorables.swap_remove(choix_sentier);
            }
            else { // On ouvre au hasard un sentier parmis les choix possibles

                let position_choisie = sentiers_a_ouvrir_possibles[entier_aleatoire(sentiers_a_ouvrir_possibles.len() as u32) as usize];

                self.obtenir_cellule(
                    &Position::new(position_choisie.0 as u32, position_choisie.1 as u32)
                ).ouvrir_sentier();

                sentiers_explorables.push((position_choisie.0 as u32, position_choisie.1 as u32));
                sentiers_a_ouvrir_possibles.clear();
            }
        }
    }

    fn enlever_murs_inutiles(&mut self) {

        for x in 0..self.longueur - 1 {

            for z in 0..self.largeur - 1 {

                if  !(self.lire_cellule(&Position::new(x, z)).est_un_sentier()) &&
                    !(self.lire_cellule(&Position::new(x + 1, z)).est_un_sentier()) {

                    self.obtenir_cellule(&Position::new(x, z)).mur_droit = false;
                    self.obtenir_cellule(&Position::new(x + 1, z)).mur_gauche = false;
                }

                if  !(self.lire_cellule(&Position::new(x, z)).est_un_sentier()) &&
                    !(self.lire_cellule(&Position::new(x, z + 1)).est_un_sentier()) {

                    self.obtenir_cellule(&Position::new(x, z)).mur_haut = false;
                    self.obtenir_cellule(&Position::new(x, z + 1)).mur_bas = false;
                }
            }
        }
    }

    fn peut_ouvrir_sentier(&self, position: (i32, i32)) -> bool {

        // Vérifier si la position existe
        let position = match self.essayer_cellule(position) {

            Some(cellule) => Position::new(cellule.x, cellule.z),
            None => return false,
        };

        // Si la cellule à la position est déjà un sentier, on ne peut pas l'ouvrir
        if self.lire_cellule(&position).est_un_sentier() {
            return false;
        }

        // Vecteur de booléens indiquant si la cellule est un sentier
        let mut cellules = std::vec::Vec::with_capacity(4);
        /* Configuration du vecteur après insertion
              2  
            0 p 3
              1  
        */

        for x in -1..2 {

            for z in -1..2 {

                if (x == 0 || z == 0) && x != z {
                    cellules.push(
                        match self.essayer_cellule((position.x as i32 + x, position.z as i32 + z)) {

                            Some(cellule) => cellule.est_un_sentier(),
                            None => false,
                        }
                    );
                }
            }
        }

        // On peut ouvrir le sentier s'il demeure un arbre
        return cellules[0] as u32 + cellules[1] as u32 + cellules[2] as u32 + cellules[3] as u32 <= 1
    }

    fn ajouter_lumieres(&mut self) {

        let hauteur = self.hauteur;
        let cote = self.cote;
        let decalage = self.decalage;
        
        for x in 1..self.longueur - 1 {

            for z in 1..self.largeur - 1 {

                let doit_ajouter = entier_aleatoire(2) == 0;

                if doit_ajouter {
                    let lumiere = self.obtenir_cellule(&Position::new(x, z)).essayer_eclairer(hauteur, cote, &decalage);

                    match lumiere {
                        Some(lumiere) => {self.lumieres.push(lumiere)},
                        None => {}, // Rien à faire
                    }
                }
            }
        }
    }

    fn position_aleatoire(&self) -> Position {

        Position::new(entier_aleatoire(self.longueur), entier_aleatoire(self.largeur))
    }

    fn lire_cellule(&self, position: &Position) -> &Cellule {
        
        &self.cellules[position.z as usize][position.x as usize]
    }

    // position: x, z
    fn essayer_cellule(&self, position: (i32, i32)) -> Option<&Cellule> {
        
        /*if  position.0 < 0 ||
            position.0 >= self.longueur as i32 ||
            position.1 < 0 ||
            position.1 >= self.largeur as i32
            {
            return None;
        }*/
        if !self.position_valide(position.0, position.1) {
            return None;
        }

        Some(self.lire_cellule(&Position::new(position.0 as u32, position.1 as u32)))
    }

    fn obtenir_cellule(&mut self, position: &Position) -> &mut Cellule {
        
        &mut self.cellules[position.z as usize][position.x as usize]
    }

    fn position_valide(&self, x: i32, z: i32) -> bool {

        x >= 0 &&
        x < self.longueur as i32 &&
        z >= 0 &&
        z < self.largeur as i32
    }
}





/*
    Partie privée du module labyrinthe
*/

fn entier_aleatoire(limite: u32) -> u32 {

    use self::rand::{Rng};
    let mut rng = rand::thread_rng();

    rng.gen_range(0, limite)
}

fn nombre_aleatoire(limite: f32) -> f32 {

    use self::rand::{Rng};
    let mut rng = rand::thread_rng();

    limite * rng.gen_range(0, 10000) as f32 / 10000.0
}

#[derive(Clone)]
struct Position {

    pub x: u32,
    pub z: u32,
}

impl Position {

    pub fn new(x: u32, z: u32) -> Position {

        Position {

            x: x,
            z: z,
        }
    }
}

struct Lumiere {

    pub position: [f32; 4],
    pub position_bas: [f32; 4],
    pub couleur: [f32; 4],
}

impl Lumiere {

    pub fn new(x: f32, y: f32, z: f32, x_bas: f32, y_bas: f32, z_bas: f32) -> Lumiere {

        const LUMIERE_ALEATOIRE: f32 = 0.6;
        const LUMIERE_BASE: f32 = 1.0 - LUMIERE_ALEATOIRE;

        Lumiere {
            position: [x, y, z, 1.0],
            position_bas: [x_bas, y_bas, z_bas, 1.0],
            couleur: [
                LUMIERE_BASE + nombre_aleatoire(LUMIERE_ALEATOIRE),
                LUMIERE_BASE + nombre_aleatoire(LUMIERE_ALEATOIRE),
                LUMIERE_BASE + nombre_aleatoire(LUMIERE_ALEATOIRE),
                1.0]
        }
    }

    pub fn ajouter_geometrie(&self, texture_id: f32, donnees_opengl: &mut donnees::DonneesOpenGL) {

        donnees_opengl.ajouter_torche(
            [self.position[0], self.position[1], self.position[2]],
            [self.position_bas[0], self.position_bas[1], self.position_bas[2]],
            texture_id);
    }
}

#[derive(Clone)]
struct Cellule {

    pub x: u32,
    pub z: u32,

    sentier: bool,
    eclaire: bool,

    // Sens selon une vue de dessus
    pub mur_gauche: bool,
    pub mur_haut: bool,
    pub mur_droit: bool,
    pub mur_bas: bool,
}

impl Cellule {

    pub fn new(x: u32, z: u32) -> Cellule {

        Cellule {

            x: x,
            z: z,

            sentier: false,
            eclaire: false,

            mur_gauche: true,
            mur_haut: true,
            mur_droit: true,
            mur_bas: true
        }
    }

    pub fn est_un_sentier(&self) -> bool {
        self.sentier
    }

    pub fn ouvrir_sentier(&mut self) {
        
        self.sentier = true;

        self.mur_gauche = false;
        self.mur_haut = false;
        self.mur_droit = false;
        self.mur_bas = false;
    }

    pub fn essayer_eclairer(
        &mut self,
        hauteur: f32, // dimension y
        cote: f32, // dimension x et z
        decalage: &[f32; 3]) -> Option<Lumiere> {

        if !self.sentier && !self.eclaire {

            self.eclaire = true;
            
            let ecart_centre = cote * 0.7;
            let hauteur_lumiere = hauteur * 0.85;

            let x = decalage[0] + cote * (self.x as f32 + 0.5);
            let y = decalage[1] + hauteur_lumiere;
            let z = decalage[2] + cote * (self.z as f32 + 0.5);

            let ecart_bas = cote * 0.45;
            let hauteur_bas = y - hauteur * 0.2;

            if self.mur_gauche && entier_aleatoire(4) == 0 {
                return Some(Lumiere::new(x - ecart_centre, y, z, x - ecart_bas, hauteur_bas, z));
            }
            if self.mur_haut && entier_aleatoire(3) == 0 {
                return Some(Lumiere::new(x, y, z + ecart_centre, x, hauteur_bas, z + ecart_bas));
            }
            if self.mur_droit && entier_aleatoire(2) == 0 {
                return Some(Lumiere::new(x + ecart_centre, y, z, x + ecart_bas, hauteur_bas, z));
            }
            if self.mur_bas {
                return Some(Lumiere::new(x, y, z - ecart_centre, x, hauteur_bas, z - ecart_bas));
            }
        }

        None
    }

    pub fn ajouter_geometrie(
        &self,
        hauteur: f32, // dimension y
        cote: f32, // dimension x et z
        decalage: &[f32; 3],
        texture: &[f32; 3], // longueur, hauteur, id 
        donnees_opengl: &mut donnees::DonneesOpenGL)
        {

        let x = self.x as f32;
        let z = self.z as f32;

        // Affecte le nombre de triangles dessinés
        const COLONNNES: u32 = 8;//4 8 16 32;
        const RANGEES: u32 = 16;//8 16 32 64;
        
        if self.mur_gauche {
            
            donnees_opengl.ajouter_plan(
                [COLONNNES, RANGEES],
                [decalage[0] + x * cote, decalage[1], decalage[2] + (z + 1.0) * cote],
                [decalage[0] + x * cote, decalage[1] + hauteur, decalage[2] + (z + 1.0) * cote],
                [decalage[0] + x * cote, decalage[1], decalage[2] + z * cote],
                texture.clone(),
            );
        }

        if self.mur_haut {
            
            donnees_opengl.ajouter_plan(
                [COLONNNES, RANGEES],
                [decalage[0] + (x + 1.0) * cote, decalage[1], decalage[2] + (z + 1.0) * cote],
                [decalage[0] + (x + 1.0) * cote, decalage[1] + hauteur, decalage[2] + (z + 1.0) * cote],
                [decalage[0] + x * cote, decalage[1], decalage[2] + (z + 1.0) * cote],
                texture.clone(),
            );
        }

        if self.mur_droit {
            
            donnees_opengl.ajouter_plan(
                [COLONNNES, RANGEES],
                [decalage[0] + (x + 1.0) * cote, decalage[1], decalage[2] + z * cote],
                [decalage[0] + (x + 1.0) * cote, decalage[1] + hauteur, decalage[2] + z * cote],
                [decalage[0] + (x + 1.0) * cote, decalage[1], decalage[2] + (z + 1.0) * cote],
                texture.clone(),
            );
        }

        if self.mur_bas {
            
            donnees_opengl.ajouter_plan(
                [COLONNNES, RANGEES],
                [decalage[0] + x * cote, decalage[1], decalage[2] + z * cote],
                [decalage[0] + x * cote, decalage[1] + hauteur, decalage[2] + z * cote],
                [decalage[0] + (x + 1.0) * cote, decalage[1], decalage[2] + z * cote],
                texture.clone(),
            );
        }
    }
}

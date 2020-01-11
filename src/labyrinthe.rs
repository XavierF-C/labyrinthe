extern crate rand;

use donnees;
use observateur;

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
        };

        labyrinthe.detruire_murs();

        labyrinthe
    }

    // texture: longueur, hauteur, id 
    pub fn ajouter_geometrie(&self, texture_plafond: [f32; 3], texture_sol: [f32; 3], texture_mur: [f32; 3], donnees_opengl: &mut donnees::DonneesOpenGL) {

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
            [decalage[0], decalage[1] + hauteur, decalage[2]],
            [decalage[0], decalage[1] + hauteur, decalage[2] + cote * self.largeur as f32],
            [decalage[0] + cote * self.longueur as f32, decalage[1] + hauteur, decalage[2]],
            [texture_plafond[0] * self.longueur as f32, texture_plafond[1] * self.largeur as f32, texture_plafond[2]]
        );
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

    fn detruire_murs(&mut self) {

        let position_depart = self.position_aleatoire();

        // sentiers desquels on peut potentiellement ouvrir un sentier adjacent
        let mut sentiers_explorables = std::vec::Vec::new();
        sentiers_explorables.push((position_depart.x, position_depart.z));

        // sentiers adjacents au sentier choisi, qui peuvent être ouverts
        let mut sentiers_a_ouvrir_possibles = std::vec::Vec::with_capacity(4);
        
        while sentiers_explorables.len() > 0 {

            let choix_sentier = nombre_aleatoire(sentiers_explorables.len() as u32) as usize;

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

                let position_choisie = sentiers_a_ouvrir_possibles[nombre_aleatoire(sentiers_a_ouvrir_possibles.len() as u32) as usize];

                let longueur = self.longueur;
                let largeur = self.largeur;

                self.obtenir_cellule(
                    &Position::new(position_choisie.0 as u32, position_choisie.1 as u32)
                ).ouvrir_sentier(longueur, largeur);

                sentiers_explorables.push((position_choisie.0 as u32, position_choisie.1 as u32));
                sentiers_a_ouvrir_possibles.clear();
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

    fn position_aleatoire(&self) -> Position {

        Position::new(nombre_aleatoire(self.longueur), nombre_aleatoire(self.largeur))
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

fn nombre_aleatoire(limite: u32) -> u32 {

    use self::rand::{Rng};
    let mut rng = rand::thread_rng();

    rng.gen_range(0, limite)
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

#[derive(Clone)]
struct Cellule {

    pub x: u32,
    pub z: u32,

    sentier: bool,

    // Sens selon une vue de dessus
    mur_gauche: bool,
    mur_haut: bool,
    mur_droit: bool,
    mur_bas: bool,
}

impl Cellule {

    pub fn new(x: u32, z: u32) -> Cellule {

        Cellule {

            x: x,
            z: z,

            sentier: false,

            mur_gauche: true,
            mur_haut: true,
            mur_droit: true,
            mur_bas: true
        }
    }

    pub fn est_un_sentier(&self) -> bool {
        self.sentier
    }

    pub fn ouvrir_sentier(&mut self, longueur: u32, largeur: u32) {
        
        self.sentier = true;

        // Les murs sont enlevés, sauf pour le périmètre
        self.mur_gauche = self.x == 0;
        self.mur_haut = self.z >= largeur - 1;
        self.mur_droit = self.x >= longueur - 1;
        self.mur_bas = self.z == 0;
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
        const COLONNNES: u32 = 32;
        const RANGEES: u32 = 64;
        
        if self.mur_gauche {
            
            /*
            donnees_opengl.ajouter_plan(
                [COLONNNES, RANGEES],
                [decalage[0] + x * cote, decalage[1], decalage[2] + z * cote],
                [decalage[0] + x * cote, decalage[1] + hauteur, decalage[2] + z * cote],
                [decalage[0] + x * cote, decalage[1], decalage[2] + (z + 1.0) * cote],
                texture.clone(),
            );*/
            donnees_opengl.ajouter_plan(
                [COLONNNES, RANGEES],
                [decalage[0] + x * cote, decalage[1], decalage[2] + (z + 1.0) * cote],
                [decalage[0] + x * cote, decalage[1] + hauteur, decalage[2] + (z + 1.0) * cote],
                [decalage[0] + x * cote, decalage[1], decalage[2] + z * cote],
                texture.clone(),
            );
        }

        if self.mur_haut {
            
            /*
            donnees_opengl.ajouter_plan(
                [COLONNNES, RANGEES],
                [decalage[0] + x * cote, decalage[1], decalage[2] + (z + 1.0) * cote],
                [decalage[0] + x * cote, decalage[1] + hauteur, decalage[2] + (z + 1.0) * cote],
                [decalage[0] + (x + 1.0) * cote, decalage[1], decalage[2] + (z + 1.0) * cote],
                texture.clone(),
            );*/
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

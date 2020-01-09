extern crate rand;

use donnees;

/*
    Interface publique du module labyrinthe

    Sert à générer le labyrinthe
*/

pub struct Labyrinthe {

    longueur: u32,
    largeur: u32, 
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

        let mut labyrinthe = Labyrinthe {

            longueur: longueur,
            largeur: largeur, 
            cellules: cellules,
        };

        labyrinthe.generer();

        labyrinthe
    }

    // texture: longueur, hauteur, id 
    pub fn ajouter_geometrie(&self, texture_plafond: [f32; 3], texture_sol: [f32; 3], texture_mur: [f32; 3], donnees_opengl: &mut donnees::DonneesOpenGL) {

        const HAUTEUR: f32 = 2.0;
        const COTE: f32 = 1.0;
        let decalage: [f32; 3] = [
            -(self.longueur as f32) * COTE / 2.0,
            0.0,//-HAUTEUR / 2.0,
            -(self.largeur as f32) * COTE / 2.0
        ];

        // Ajouter tous les murs
        for z in 0..self.largeur {

            for x in 0..self.longueur {
                
                let position = Position::new(x, z);
                self.lire_cellule(&position).ajouter_geometrie(HAUTEUR, COTE, &decalage, &texture_mur, donnees_opengl);
            }
        }

        const QUALITE: u32 = 4;

        // Ajoute le plancher
        donnees_opengl.ajouter_gros_plan(
            [self.longueur * QUALITE, self.largeur * QUALITE],
            [decalage[0], decalage[1], decalage[2]],
            [decalage[0], decalage[1], decalage[2] + COTE * self.largeur as f32],
            [decalage[0] + COTE * self.longueur as f32, decalage[1], decalage[2]],
            [texture_sol[0] * self.longueur as f32, texture_sol[1] * self.largeur as f32, texture_sol[2]]
        );
        /*donnees_opengl.ajouter_plan(
            [decalage[0], decalage[1], decalage[2]],
            [decalage[0], decalage[1], decalage[2] + COTE * self.largeur as f32],
            [decalage[0] + COTE * self.longueur as f32, decalage[1], decalage[2] + COTE * self.largeur as f32],
            [decalage[0] + COTE * self.longueur as f32, decalage[1], decalage[2]],
            [texture_sol[0] * self.longueur as f32, texture_sol[1] * self.largeur as f32, texture_sol[2]]
        );*/
        
        // Ajoute le plafond
        donnees_opengl.ajouter_gros_plan(
            [self.longueur * QUALITE, self.largeur * QUALITE],
            [decalage[0], decalage[1] + HAUTEUR, decalage[2]],
            [decalage[0], decalage[1] + HAUTEUR, decalage[2] + COTE * self.largeur as f32],
            [decalage[0] + COTE * self.longueur as f32, decalage[1] + HAUTEUR, decalage[2]],
            [texture_plafond[0] * self.longueur as f32, texture_plafond[1] * self.largeur as f32, texture_plafond[2]]
        );
        /*donnees_opengl.ajouter_plan(
            [decalage[0], decalage[1] + HAUTEUR, decalage[2]],
            [decalage[0], decalage[1] + HAUTEUR, decalage[2] + COTE * self.largeur as f32],
            [decalage[0] + COTE * self.longueur as f32, decalage[1] + HAUTEUR, decalage[2] + COTE * self.largeur as f32],
            [decalage[0] + COTE * self.longueur as f32, decalage[1] + HAUTEUR, decalage[2]],
            [texture_plafond[0] * self.longueur as f32, texture_plafond[1] * self.largeur as f32, texture_plafond[2]]
        );*/
    }

    fn generer(&mut self) {

        //self.enlever_murs_aleatoire(10 * self.longueur * self.largeur);

        let position_depart = self.position_aleatoire();

        let mut sentiers_commences = std::vec::Vec::new();
        sentiers_commences.push((position_depart.x, position_depart.z));

        let longueur = self.longueur;
        let largeur = self.largeur;

        let mut sentiers_a_ouvrir_possibles = std::vec::Vec::with_capacity(4);

        while sentiers_commences.len() > 0 {

            let choix_sentier = nombre_aleatoire(sentiers_commences.len() as u32) as usize;

            let position_courante = sentiers_commences[choix_sentier];

            let gauche = (position_courante.0 as i32 - 1, position_courante.1 as i32);
            if self.peut_ouvrir_sentier(gauche) {
                sentiers_a_ouvrir_possibles.push(gauche);
            }
            
            let haut = (position_courante.0 as i32, position_courante.1 as i32 + 1);
            if self.peut_ouvrir_sentier(haut) {
                sentiers_a_ouvrir_possibles.push(haut);
            }
            
            let droit = (position_courante.0 as i32 + 1, position_courante.1 as i32);
            if self.peut_ouvrir_sentier(droit) {
                sentiers_a_ouvrir_possibles.push(droit);
            }
            
            let bas = (position_courante.0 as i32, position_courante.1 as i32 - 1);
            if self.peut_ouvrir_sentier(bas) {
                sentiers_a_ouvrir_possibles.push(bas);
            }
            
            if self.peut_ouvrir_sentier(bas) {
                sentiers_a_ouvrir_possibles.push(bas);
            }

            if sentiers_a_ouvrir_possibles.len() == 0 {
                sentiers_commences.swap_remove(choix_sentier);
            }
            else {

                let position_choisie = sentiers_a_ouvrir_possibles[nombre_aleatoire(sentiers_a_ouvrir_possibles.len() as u32) as usize];

                self.obtenir_cellule(
                    &Position::new(position_choisie.0 as u32, position_choisie.1 as u32)
                ).ouvrir_sentier(longueur, largeur);

                sentiers_commences.push((position_choisie.0 as u32, position_choisie.1 as u32));
                sentiers_a_ouvrir_possibles.clear();
            }
        }
    }

    /*
    fn enlever_murs_aleatoire(&mut self, tentatives: u32) {

        for _ in 0..tentatives {

            let position = self.position_aleatoire();
            let mur = self.mur_aleatoire(self.lire_cellule(&position));
            let position_voisine = self.lire_cellule(&position).position_voisine(&mur);

            if self.peut_enlever_mur(&position, &position_voisine) {
                
                self.obtenir_cellule(&position).enlever_mur(&mur);
                self.obtenir_cellule(&position_voisine).enlever_mur(&mur.inverse());
            }
        }
    }*/

    // Retourne vrai si l'enlèvement du mur ne cause pas l'apparition d'un mur «mince»
    /*
    fn peut_enlever_mur(&self, position: &Position, position_voisine: &Position) -> bool {

        let mut position = position.clone();
        let mut position_voisine = position_voisine.clone();

        if position_voisine.x < position.x {
            std::mem::swap(&mut position, &mut position_voisine);
        }

        if position_voisine.z < position.z {
            std::mem::swap(&mut position, &mut position_voisine);
        }

        const NOMBRE_MURS_MINIMAL: u32 = 3;

        if position.x != position_voisine.x { // Mur vertical

            let cellule_haut_gauche = self.essayer_cellule((position.x as i32, position.z as i32 + 1));
            let cellule_haut_droite = self.essayer_cellule((position.x as i32 + 1, position.z as i32 + 1));

            if Cellule::nombre_murs_connexes(
                Some(self.lire_cellule(&position)),
                cellule_haut_gauche,
                cellule_haut_droite,
                Some(self.lire_cellule(&position_voisine))) < NOMBRE_MURS_MINIMAL {

                return false;
            }
            
            let cellule_bas_gauche = self.essayer_cellule((position.x as i32, position.z as i32 - 1));
            let cellule_bas_droite = self.essayer_cellule((position.x as i32 + 1, position.z as i32 - 1));

            if Cellule::nombre_murs_connexes(
                cellule_bas_gauche,
                Some(self.lire_cellule(&position)),
                Some(self.lire_cellule(&position_voisine)),
                cellule_bas_droite) < NOMBRE_MURS_MINIMAL {

                return false;
            }
        }
        else { // Mur horizontal

            let cellule_haut_gauche = self.essayer_cellule((position.x as i32 - 1, position.z as i32 + 1));
            let cellule_bas_gauche = self.essayer_cellule((position.x as i32 - 1, position.z as i32));

            if Cellule::nombre_murs_connexes(
                cellule_bas_gauche,
                cellule_haut_gauche,
                Some(self.lire_cellule(&position_voisine)),
                Some(self.lire_cellule(&position))) < NOMBRE_MURS_MINIMAL {

                return false;
            }
            
            let cellule_haut_droite = self.essayer_cellule((position.x as i32 + 1, position.z as i32 + 1));
            let cellule_bas_droite = self.essayer_cellule((position.x as i32 + 1, position.z as i32));

            if Cellule::nombre_murs_connexes(
                Some(self.lire_cellule(&position)),
                Some(self.lire_cellule(&position_voisine)),
                cellule_haut_droite,
                cellule_bas_droite) < NOMBRE_MURS_MINIMAL {

                return false;
            }
        }

        true
    }*/

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
        /*let mut cellules = std::vec::Vec::with_capacity(8);
        /* Configuration du vecteur après insertion
            2 4 7
            1 p 6
            0 3 5
        */

        for x in -1..2 {

            for z in -1..2 {

                if !(x == 0 && z == 0) {
                    cellules.push(
                        match self.essayer_cellule((position.x as i32 + x, position.z as i32 + z)) {

                            Some(cellule) => cellule.est_un_sentier(),
                            None => false,
                        }
                    );
                }
            }
        }*/

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

        // On peut ouvrir le sentier si il demeure un arbre
        return cellules[0] as u32 + cellules[1] as u32 + cellules[2] as u32 + cellules[3] as u32 <= 1

        // Si un des 4 coins a 3 sentiers, on ne peut pas ouvrir de sentier
        /* !(cellules[0] && cellules[1] && cellules[3]) &&
        !(cellules[1] && cellules[2] && cellules[4]) &&
        !(cellules[4] && cellules[6] && cellules[7]) &&
        !(cellules[3] && cellules[5] && cellules[6])*/
        //true
    }

    fn position_aleatoire(&self) -> Position {

        /*use self::rand::{Rng};
        let mut rng = rand::thread_rng();

        Position::new(rng.gen_range(0, self.longueur), rng.gen_range(0, self.largeur))*/
        Position::new(nombre_aleatoire(self.longueur), nombre_aleatoire(self.largeur))
    }

    fn lire_cellule(&self, position: &Position) -> &Cellule {
        
        &self.cellules[position.z as usize][position.x as usize]
    }

    // position: x, z
    fn essayer_cellule(&self, position: (i32, i32)) -> Option<&Cellule> {
        
        if  position.0 < 0 ||
            position.0 >= self.longueur as i32 ||
            position.1 < 0 ||
            position.1 >= self.largeur as i32
            {
            return None;
        }

        Some(self.lire_cellule(&Position::new(position.0 as u32, position.1 as u32)))
    }

    fn obtenir_cellule(&mut self, position: &Position) -> &mut Cellule {
        
        &mut self.cellules[position.z as usize][position.x as usize]
    }

    /*
    fn mur_aleatoire(&self, cellule: &Cellule) -> Mur {

        let mut mur_choisi: Option<Mur> = None;

        let x = cellule.x;
        let z = cellule.z;

        while mur_choisi == None {

            mur_choisi = Some(Mur::aleatoire());

            // Éviter de choisir un mur extérieur
            if (mur_choisi == Some(Mur::Gauche) && x == 0) ||
                (mur_choisi == Some(Mur::Droit) && x >= self.longueur - 1) ||
                (mur_choisi == Some(Mur::Haut) && z >= self.largeur - 1) ||
                (mur_choisi == Some(Mur::Bas) && z == 0)
                {

                mur_choisi = None;
            }
        }

        mur_choisi.unwrap()
    }*/
}





/*
    Partie privée du module labyrinthe
*/

fn nombre_aleatoire(limite: u32) -> u32 {

    use self::rand::{Rng};
    let mut rng = rand::thread_rng();

    rng.gen_range(0, limite)
}

/*
#[derive(PartialEq)]
enum Mur {
    Gauche,
    Droit,
    Haut,
    Bas,
}

impl Mur {

    pub fn aleatoire() -> Mur {

        /*use self::rand::{Rng};
        let mut rng = rand::thread_rng();

        let choix = rng.gen_range(0, 4);*/
        let choix = nombre_aleatoire(4);

        match choix {
            0 => Mur::Gauche,
            1 => Mur::Droit,
            2 => Mur::Haut,
            _ => Mur::Bas, 
        }
    }

    pub fn inverse(&self) -> Mur {

        match self {

            Mur::Gauche => Mur::Droit,
            Mur::Droit => Mur::Gauche,
            Mur::Haut => Mur::Bas,
            Mur::Bas => Mur::Haut,
        }
    }
}*/

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

    /*
    pub fn position_voisine(&self, mur: &Mur) -> Position {

        let x = self.x;
        let z = self.z;

        match mur {

            Mur::Gauche => Position::new(x - 1, z),
            Mur::Droit => Position::new(x + 1, z),
            Mur::Haut => Position::new(x, z + 1),
            Mur::Bas => Position::new(x, z - 1),
        }
    }*/

    /*
    pub fn enlever_mur(&mut self, mur: &Mur) {

        match mur {

            Mur::Gauche => self.mur_gauche = false,
            Mur::Droit => self.mur_droit = false,
            Mur::Haut => self.mur_haut = false,
            Mur::Bas => self.mur_bas = false,
        }
    }*/

    // Cette fonction retourne un nombre entre 0 et 4 selon le nombre de murs joints au centre des 4 cellules
    /*
    pub fn nombre_murs_connexes(
        cel_bas_gauche: Option<&Cellule>,
        cel_haut_gauche: Option<&Cellule>,
        cel_haut_droite: Option<&Cellule>,
        cel_bas_droite: Option<&Cellule>) -> u32
        {

        /*  Configuration de l'intersection

             h
            g+d
             b
        */
        let mut mur_gauche = false;
        let mut mur_haut = false;
        let mut mur_droit = false;
        let mut mur_bas = false;

        if let Some(cellule_bas_gauche) = cel_bas_gauche {
            
            if cellule_bas_gauche.mur_haut {
                mur_gauche = true;
            }
            if cellule_bas_gauche.mur_droit {
                mur_bas = true;
            }
        }

        if let Some(cellule_haut_gauche) = cel_haut_gauche {
            
            if cellule_haut_gauche.mur_droit {
                mur_haut = true;
            }
            if cellule_haut_gauche.mur_bas {
                mur_gauche = true;
            }
        }

        if let Some(cellule_haut_droite) = cel_haut_droite {
            
            if cellule_haut_droite.mur_gauche {
                mur_haut = true;
            }
            if cellule_haut_droite.mur_bas {
                mur_droit = true;
            }
        }

        if let Some(cellule_bas_droite) = cel_bas_droite {
            
            if cellule_bas_droite.mur_gauche {
                mur_bas = true;
            }
            if cellule_bas_droite.mur_haut {
                mur_droit = true;
            }
        }
        
        mur_gauche as u32 + mur_haut as u32 + mur_droit as u32 + mur_bas as u32
    }*/

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
        
        if self.mur_gauche {
            
            donnees_opengl.ajouter_plan(
                
                [decalage[0] + x * cote, decalage[1], decalage[2] + z * cote],
                [decalage[0] + x * cote, decalage[1] + hauteur, decalage[2] + z * cote],
                [decalage[0] + x * cote, decalage[1] + hauteur, decalage[2] + (z + 1.0) * cote],
                [decalage[0] + x * cote, decalage[1], decalage[2] + (z + 1.0) * cote],
                texture.clone(),
            );
        }

        if self.mur_haut {
            
            donnees_opengl.ajouter_plan(
                
                [decalage[0] + x * cote, decalage[1], decalage[2] + (z + 1.0) * cote],
                [decalage[0] + x * cote, decalage[1] + hauteur, decalage[2] + (z + 1.0) * cote],
                [decalage[0] + (x + 1.0) * cote, decalage[1] + hauteur, decalage[2] + (z + 1.0) * cote],
                [decalage[0] + (x + 1.0) * cote, decalage[1], decalage[2] + (z + 1.0) * cote],
                texture.clone(),
            );
        }

        if self.mur_droit {
            
            donnees_opengl.ajouter_plan(
                
                [decalage[0] + (x + 1.0) * cote, decalage[1], decalage[2] + z * cote],
                [decalage[0] + (x + 1.0) * cote, decalage[1] + hauteur, decalage[2] + z * cote],
                [decalage[0] + (x + 1.0) * cote, decalage[1] + hauteur, decalage[2] + (z + 1.0) * cote],
                [decalage[0] + (x + 1.0) * cote, decalage[1], decalage[2] + (z + 1.0) * cote],
                texture.clone(),
            );
        }

        if self.mur_bas {
            
            donnees_opengl.ajouter_plan(
                
                [decalage[0] + x * cote, decalage[1], decalage[2] + z * cote],
                [decalage[0] + x * cote, decalage[1] + hauteur, decalage[2] + z * cote],
                [decalage[0] + (x + 1.0) * cote, decalage[1] + hauteur, decalage[2] + z * cote],
                [decalage[0] + (x + 1.0) * cote, decalage[1], decalage[2] + z * cote],
                texture.clone(),
            );
        }
    }
}

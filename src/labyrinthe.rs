extern crate rand;

use donnees;

/*
    Interface publique du module labyrinthe

    Sert à générer le labyrinthe
*/

pub struct Labyrinthe {

    longueur: usize,
    largeur: usize, 
    cellules: std::vec::Vec<std::vec::Vec<Cellule>>,
}

impl Labyrinthe {

    pub fn new(longueur: usize, largeur: usize) -> Labyrinthe{

        let mut cellules = std::vec::Vec::with_capacity(largeur);

        for z in 0..largeur {

            let mut rangee = std::vec::Vec::with_capacity(longueur);

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
    pub fn ajouter_geometrie(&self, texture: [f32; 3], donnees_opengl: &mut donnees::DonneesOpenGL) {

        const HAUTEUR: f32 = 2.0;
        const COTE: f32 = 1.0;
        let decalage: [f32; 3] = [
            -(self.longueur as f32) * COTE / 2.0,
            -HAUTEUR / 2.0,
            -(self.largeur as f32) * COTE / 2.0
        ];

        for z in 0..self.largeur {

            for x in 0..self.longueur {
                
                self.cellules[z][x].ajouter_geometrie(HAUTEUR, COTE, &decalage, &texture, donnees_opengl);
            }
        }
    }

    fn generer(&mut self) {

        self.enlever_murs_aleatoire(self.longueur * self.largeur);
    }

    fn enlever_murs_aleatoire(&mut self, nombre: usize) {

        for _ in 0..nombre {

            let position = self.position_aleatoire();
            let mur = self.mur_aleatoire(&self.cellules[position.1][position.0]);
            let position_voisine = self.cellules[position.1][position.0].position_voisine(&mur);

            &mut self.cellules[position.1][position.0].enlever_mur(&mur);
            &mut self.cellules[position_voisine.1][position_voisine.0].enlever_mur(&mur.inverse());
        }
    }

    fn position_aleatoire(&self) -> (usize, usize) {

        use self::rand::{Rng};
        let mut rng = rand::thread_rng();

        // retourne (x, z)
        (rng.gen_range(0, self.longueur), rng.gen_range(0, self.largeur))
    }

    /*
    fn cellule_voisine_aleatoire(&mut self, cellule: &Cellule) -> &mut Cellule {

        let mut mur_choisi: Option<Mur> = None;

        let x = cellule.x as usize;
        let z = cellule.z as usize;

        while mur_choisi == None {

            mur_choisi = Some(Mur::aleatoire());

            // Éviter de choisir une cellule n'existant pas
            if (mur_choisi == Some(Mur::Gauche) && x == 0) ||
                (mur_choisi == Some(Mur::Droit) && x >= self.longueur - 1) ||
                (mur_choisi == Some(Mur::Haut) && z >= self.largeur - 1) ||
                (mur_choisi == Some(Mur::Bas) && z == 0)
                {

                mur_choisi = None;
            }
        }

        match mur_choisi.unwrap() {
            Mur::Gauche => return &mut self.cellules[z][x - 1],
            Mur::Droit => return &mut self.cellules[z][x + 1],
            Mur::Haut => return &mut self.cellules[z + 1][x],
            Mur::Bas => return &mut self.cellules[z - 1][x],
        }
    }*/

    fn mur_aleatoire(&self, cellule: &Cellule) -> Mur {

        let mut mur_choisi: Option<Mur> = None;

        let x = cellule.x as usize;
        let z = cellule.z as usize;

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
    }
}





/*
    Partie privée du module labyrinthe
*/

#[derive(PartialEq)]
enum Mur {
    Gauche,
    Droit,
    Haut,
    Bas,
}

impl Mur {

    pub fn aleatoire() -> Mur {

        use self::rand::{Rng};
        let mut rng = rand::thread_rng();

        let choix = rng.gen_range(0, 4);

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
}

#[derive(Clone)]
struct Cellule {

    pub x: u32,
    pub z: u32,

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

            mur_gauche: true,
            mur_haut: true,
            mur_droit: true,
            mur_bas: true
        }
    }

    /*pub fn enlever_mur(cellule_1: &mut Cellule, cellule_2: &mut Cellule) {

        if cellule_1.x < cellule_2.x {
            
            cellule_1.mur_droit = false;
            cellule_2.mur_gauche = false;
        }
        else if cellule_1.x > cellule_2.x {

            cellule_1.mur_gauche = false;
            cellule_2.mur_droit = false;
        }

        if cellule_1.z < cellule_2.z {

            cellule_1.mur_haut = false;
            cellule_2.mur_bas = false;
        }
        else if cellule_1.z > cellule_2.z {

            cellule_1.mur_bas = false;
            cellule_2.mur_haut = false;
        }
    }*/

    pub fn position_voisine(&self, mur: &Mur) -> (usize, usize) {

        let x = self.x as usize;
        let z = self.z as usize;

        match mur {

            Mur::Gauche => (x - 1, z),
            Mur::Droit => (x + 1, z),
            Mur::Haut => (x, z + 1),
            Mur::Bas => (x, z - 1),
        }
    }

    pub fn enlever_mur(&mut self, mur: &Mur) {

        match mur {

            Mur::Gauche => self.mur_gauche = false,
            Mur::Droit => self.mur_droit = false,
            Mur::Haut => self.mur_haut = false,
            Mur::Bas => self.mur_bas = false,
        }
    }

    pub fn ajouter_geometrie(
        &self,
        hauteur: f32,
        cote: f32,
        decalage: &[f32; 3],
        texture: &[f32; 3],
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

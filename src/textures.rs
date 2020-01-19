/*
    Interface publique du module textures

    Sert à charger et à utiliser les textures
*/

pub struct Textures<'a> {

    images: std::vec::Vec<glium::texture::RawImage2d<'a, u8>>,
    identifiants: std::collections::HashMap<String, f32>,
    textures: Option<glium::texture::texture2d_array::Texture2dArray>,
}

impl<'a> Textures<'a> {

    pub fn new() -> Textures<'a> {

        Textures {

            images: std::vec::Vec::new(),
            identifiants: std::collections::HashMap::new(),
            textures: None
        }
    }

    // Donne le numéro associé à l'identifiant
    pub fn obtenir_id(&self, identifiant: &str) -> f32 {

        match self.identifiants.get(identifiant) {

            Some(id) => return *id,

            None => panic!("L'identifiant pour la texture "),
        }
    }

    // identifiant servira à obtenir l'index de la texture
    pub fn charger_images(&mut self, noms_images: &[&str], extension: &str) {

        for i in 0..noms_images.len() {

            self.charger_image(&(noms_images[i].to_owned() + extension), noms_images[i]);
        }
    }

    // identifiant servira à obtenir l'index de la texture
    fn charger_image(&mut self, nom_image_et_extension: &str, identifiant: &str) {

        const CHEMIN: &str = "images/";

        let image = image::io::Reader::open(CHEMIN.to_owned() + nom_image_et_extension).unwrap().decode().unwrap().to_rgba();
        let dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), dimensions);

        self.identifiants.insert(identifiant.to_string(), self.images.len() as f32);
        self.images.push(image);
    }

    // Cette fonction devrait être appelée une fois après avoir chargé toutes les images
    pub fn generer_textures(&mut self, affichage: &glium::Display) {

        let mut vecteur: std::vec::Vec<glium::texture::RawImage2d<u8>> = std::vec::Vec::new();
        std::mem::swap(&mut vecteur, &mut self.images);

        let textures = glium::texture::texture2d_array::Texture2dArray::new(affichage, vecteur);
        self.textures = Some(textures.unwrap());
    }
}

/*
    Partie privée du module textures
*/


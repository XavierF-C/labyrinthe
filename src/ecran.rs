extern crate nalgebra_glm as glm;
use glium::{Surface}; // Surface est un trait et doit être importé

/*
    Interface publique du module ecran

    Permet de dessiner et d'interagir avec l'écran
*/

pub struct Vue {

    position: glm::Vec3,
    direction: glm::Vec3,
}

impl Vue {

    pub fn new(position: &glm::Vec3, direction: &glm::Vec3) -> Vue {

        Vue {

            position: position.clone(),
            direction: direction.clone(),
        }
    }

    // Permet de tout dessiner sur la fenêtre
    pub fn dessiner(&self,
                    donnees_opengl: &::donnees::DonneesOpenGL,
                    programme_opengl: &::shaders::ProgrammeOpenGL,
                    affichage: &glium::Display)
    {
        // affichage.draw() retourne un struct Frame, sur lequel on peut dessiner 
        let mut cadre = affichage.draw();

        let matrice_camera_perspective = ::donnees::matrice_camera_perspective(
                                            &self.position,
                                            &self.direction,
                                            Vue::obtenir_ratio_ecran(&affichage));
        // Données globales à envoyer, vers le bloc uniform
        let donnees_globales = uniform! {
            cameraPerspective: matrice_camera_perspective,
            positionObservateur: [self.position.x, self.position.y, self.position.z]
        };

        cadre.clear_color_and_depth((0.3, 0.3, 0.5, 1.0), 1.0);

        // Permet de tenir compte de la profondeur
        let parametres = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        cadre.draw(
            donnees_opengl.obtenir_vertex_buffer(),
            &donnees_opengl.obtenir_indices(&affichage),
            &programme_opengl.programme,
            &donnees_globales,
            &parametres,
        ).unwrap();
        cadre.finish().unwrap();
    }

    // Donne le ratio largeur / hauteur de l'écran
    fn obtenir_ratio_ecran(affichage: &glium::Display) -> f32 {

        let fenetre = affichage.gl_window().window().inner_size();

        return (fenetre.width / fenetre.height) as f32;
    }
}


/*
    Partie privée du module ecran
*/
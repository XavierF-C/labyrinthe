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

    pub fn new(position: glm::Vec3, direction: glm::Vec3) -> Vue {

        Vue {

            position: position,
            direction: direction,
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

        let matrice_camera_perspective = ::donnees::matrice_camera_perspective(&self.position, &self.direction);
        // Données globales à envoyer, vers le bloc uniform
        let donnees_globales = uniform! {
            cameraPerspective: matrice_camera_perspective
        };

        cadre.clear_color(0.3, 0.3, 0.5, 1.0);
        cadre.draw(
            donnees_opengl.obtenir_vertex_buffer(),
            &donnees_opengl.obtenir_indices(&affichage),
            &programme_opengl.programme,
            &donnees_globales,//&glium::uniforms::EmptyUniforms,
            &Default::default()
        ).unwrap();
        cadre.finish().unwrap();
    }
}


/*
    Partie privée du module ecran
*/
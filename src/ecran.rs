extern crate nalgebra_glm as glm;
use glium::{Surface}; // Surface est un trait et doit être importé

use donnees;

/*
    Interface publique du module ecran

    Permet de dessiner et d'interagir avec l'écran
*/

pub struct Vue {

    position: glm::Vec3,
    direction: glm::Vec3,
}

impl Vue {

    pub fn new() -> Vue {

        Vue {

            position: glm::Vec3::new(0.0, 0.0, 0.0),
            direction: glm::Vec3::new(0.0, 0.0, 1.0),
        }
    }

    pub fn changer_camera(&mut self, position: &glm::Vec3, direction: &glm::Vec3) {

        self.position = position.clone();
        self.direction = direction.clone();
    }

    // Permet de tout dessiner sur la fenêtre
    pub fn dessiner(&self,
                    lumieres: Lumieres,
                    donnees_opengl: &donnees::DonneesOpenGL,
                    programme_opengl: &::shaders::ProgrammeOpenGL,
                    affichage: &glium::Display)
    {
        let mut tampon_lumieres: glium::uniforms::UniformBuffer<Lumieres> =
            glium::uniforms::UniformBuffer::empty(affichage).unwrap();

        // On remplit le tampon des lumières
        {
            let mut mapping = tampon_lumieres.map();
            let mut compteur = 0;

            for valeur in mapping.positions.iter_mut() {
                
                *valeur = lumieres.positions[compteur];
                compteur += 1;
            }

            compteur = 0;
            for valeur in mapping.couleurs.iter_mut() {
                
                *valeur = lumieres.couleurs[compteur];
                compteur += 1;
            }
        }

        let matrice_camera_perspective = ::donnees::matrice_camera_perspective(
            &self.position,
            &self.direction,
            Vue::obtenir_ratio_ecran(&affichage));
        
         // affichage.draw() retourne un struct Frame, sur lequel on peut mettre à jour les tampons de couleur et de profondeur
         // Une fois les tampons remplis, on peut dessiner le tout
         let mut cadre = affichage.draw();

        /* ------------------------------------------------
            Les commandes ci-dessous permettent de calculer la profondeur avant de dessiner dans le tampon de couleur
            Cette technique améliore les performances du fragment shader qui s'éxécutera par après
        ------------------------------------------------ */

        // Données globales à envoyer, vers le bloc uniform
        let donnees_globales_prepasse = uniform! {
            camera_perspective: matrice_camera_perspective
        };

        cadre.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        
        let parametres_prepasse = glium::DrawParameters {
            depth: glium::Depth { // Permet de tenir compte de la profondeur
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };

        cadre.draw(
            donnees_opengl.obtenir_vertex_buffer(),
            &donnees_opengl.obtenir_indices(&affichage),
            &programme_opengl.programme_prepasse,
            &donnees_globales_prepasse,
            &parametres_prepasse,
        ).unwrap(); // Mets à jour le tampon de profondeur

        
        /* ------------------------------------------------
            Les commandes ci-dessous permettent de tout dessiner
        ------------------------------------------------ */

        // Données globales à envoyer, vers le bloc uniform
        let donnees_globales = uniform! {
            camera_perspective: matrice_camera_perspective,
            direction_regard: [self.direction.x, self.direction.y, self.direction.z],
            lumieres: &*tampon_lumieres,
        };

        let parametres = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfEqual, // Permet d'utiliser la profondeur déjà calculée
                write: false, // On n'a donc pas besoin d'écrire dans le depth buffer
                .. Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };

        cadre.draw(
            donnees_opengl.obtenir_vertex_buffer(),
            &donnees_opengl.obtenir_indices(&affichage),
            &programme_opengl.programme,
            &donnees_globales,
            &parametres,
        ).unwrap(); // Mets à jour le tampon de couleur
        
        cadre.finish().unwrap(); // Dessine sur la fenêtre
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

const INFINI: f32 = 1000000.0;
pub const NOMBRE_LUMIERES: usize = 8;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Lumieres {

    pub positions: [[f32; 4]; NOMBRE_LUMIERES],
    pub couleurs: [[f32; 4]; NOMBRE_LUMIERES],
}

implement_uniform_block!(Lumieres, positions, couleurs);

impl Lumieres {

    pub fn new() -> Lumieres {
        
        let positions = [[INFINI, INFINI, INFINI, 1.0]; NOMBRE_LUMIERES];
        let couleurs = [[1.0, 1.0, 1.0, 1.0]; NOMBRE_LUMIERES];

        Lumieres {

            positions: positions,
            couleurs: couleurs
        }
    }
}
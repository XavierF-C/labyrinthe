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
                    lumieres: Lumieres,
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
        
        //let mut lumieres = Lumieres::new();
        //lumieres.positions[0] = [self.position.x, self.position.y, self.position.z, 1.0];
        //lumieres.couleurs[0] = [0.0, 0.0, 0.0, 1.0];

        /*
        for x in 0..2 {
            for z in 0..2 {

                lumieres.positions[1 + x*2 + z] = [-4.0 + 6.0*x as f32, 1.7, -4.0 + 6.0*z as f32, 1.0];
            }
        }*/

        /*
        for x in 0..2 {
            for z in 0..2 {

                lumieres.positions[1 + x*4 + z] = [-2.9 + 2.0*x as f32, 1.7, -2.9 + 2.0*z as f32, 1.0];
            }
        }*/

        let mut tampon_lumieres: glium::uniforms::UniformBuffer<Lumieres> =
            glium::uniforms::UniformBuffer::empty(affichage).unwrap();

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

        // Données globales à envoyer, vers le bloc uniform
        let donnees_globales = uniform! {
            camera_perspective: matrice_camera_perspective,
            direction_regard: [self.direction.x, self.direction.y, self.direction.z],
            lumieres: &*tampon_lumieres,
        };

        cadre.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        // Permet de tenir compte de la profondeur
        let parametres = glium::DrawParameters {
            depth: glium::Depth {
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
extern crate nalgebra_glm as glm;

/*
    Interface publique du module donnees

    Sert à créer les données à envoyer pour OpenGL
*/

// Contient les données nécessaires pour afficher avec OpenGL
pub struct DonneesOpenGL {
    
    sommets: std::vec::Vec<Sommet>,
    indices: std::vec::Vec<u32>, // Permet à Opengl d'interpréter les sommets

    pub vertex_buffer: std::option::Option<glium::VertexBuffer<Sommet>>,
}

impl DonneesOpenGL {

    // Initialisation sans donnée. Il faut les ajouter, puis appeler generer_vertex_buffer
    pub fn new() -> DonneesOpenGL {

        const NOMBRE_SOMMETS_BASE: usize = 16;

        DonneesOpenGL {

            sommets: std::vec::Vec::with_capacity(NOMBRE_SOMMETS_BASE),
            indices: std::vec::Vec::with_capacity(NOMBRE_SOMMETS_BASE),

            vertex_buffer: None, // Doit être créé après avoir fourni les données
        }
    }

    pub fn ajouter_plan(
        &mut self,
        coin_bas_gauche: [f32; 3],
        coin_haut_gauche: [f32; 3],
        coin_haut_droit: [f32; 3],
        coin_bas_droit: [f32; 3],
        texture: [f32; 3]) // longueur, hauteur, id 
        {
        
        self.sommets.reserve(4);
        let premier_sommet = self.sommets.len() as u32;

        self.sommets.push(Sommet{
            position: [coin_bas_gauche[0], coin_bas_gauche[1], coin_bas_gauche[2]],
            coordonnees_texture: [0.0, 0.0, texture[2]],
        });

        self.sommets.push(Sommet{
            position: [coin_haut_gauche[0], coin_haut_gauche[1], coin_haut_gauche[2]],
            coordonnees_texture: [0.0, texture[1], texture[2]],
        });

        self.sommets.push(Sommet{
            position: [coin_haut_droit[0], coin_haut_droit[1], coin_haut_droit[2]],
            coordonnees_texture: [texture[0], texture[1], texture[2]],
        });

        self.sommets.push(Sommet{
            position: [coin_bas_droit[0], coin_bas_droit[1], coin_bas_droit[2]],
            coordonnees_texture: [texture[0], 0.0, texture[2]],
        });


        self.indices.reserve(6);

        self.indices.push(premier_sommet); // Créer un triangle «dégénéré» avec le mode «trianglestrip»
        self.indices.push(premier_sommet);
        self.indices.push(premier_sommet + 1);
        self.indices.push(premier_sommet + 3);
        self.indices.push(premier_sommet + 2);
        self.indices.push(premier_sommet + 2); // Créer un triangle «dégénéré» avec le mode «trianglestrip»
    }

    // Cette fonction est nécessaire pour appeler correctement obtenir_vertex_buffer
    pub fn generer_vertex_buffer(&mut self, affichage: &glium::Display) {

        self.vertex_buffer = Some(glium::VertexBuffer::new(affichage, &(self.sommets)).unwrap());
    }

    // Cette fonction requiert d'avoir appelé generer_vertex_buffer
    pub fn obtenir_vertex_buffer(&self) -> &glium::VertexBuffer<Sommet> {

        self.vertex_buffer.as_ref().unwrap()
    }

    pub fn obtenir_indices(&self, affichage: &glium::Display) -> glium::index::IndexBuffer<u32> {

        glium::index::IndexBuffer::new(
            affichage,
            glium::index::PrimitiveType::TriangleStrip,//glium::index::PrimitiveType::TrianglesList,
            &self.indices[..]
        ).unwrap()
        // TriangleStrip <=> Chaque triplet consécutif représente un triangle
        // TrianglesList <=> Un triangle par triplet d'indices
    }
}

// Primitive pour OpenGL
#[derive(Copy, Clone)]
pub struct Sommet {
    position: [f32; 3],
    coordonnees_texture: [f32; 3], // x,y et index
}
// Permet à Glium de l'utiliser avec OpenGL
implement_vertex!(Sommet, position, coordonnees_texture);

// Matrice importante qui sera appliquée sur tous les sommets
pub fn matrice_camera_perspective(position: &glm::Vec3, direction: &glm::Vec3, ratio: f32) -> [[f32; 4]; 4] {

    // Champ de vision de 90° et visibilité entre 0.01 et 100
    let resultat =  matrice_perspective(std::f32::consts::PI * 0.5, ratio, 0.01, 100.0) *
                    matrice_camera(*position, *direction);

    matrice_opengl(resultat)
}





/*
    Partie privée du module donnees
*/

fn matrice_camera(position: glm::Vec3, direction: glm::Vec3) -> glm::Mat4 {

    let direction = glm::normalize(&direction);
    let haut = glm::Vec3::new(0.0, 1.0, 0.0);// Le haut est l'axe des y positif avec OpenGL

    let droite = haut.cross(&direction);
    let haut = direction.cross(&droite);

    
    glm::Mat4::new(
        droite.x, droite.y, droite.z, 0.0,
        haut.x, haut.y, haut.z, 0.0,
        direction.x, direction.y, direction.z, 0.0,
        0.0, 0.0, 0.0, 1.0
    ) // matrice de la caméra
    *
    glm::Mat4::new(
        1.0, 0.0, 0.0, -position.x,
        0.0, 1.0, 0.0, -position.y,
        0.0, 0.0, 1.0, -position.z,
        0.0, 0.0, 0.0, 1.0
    ) // matrice de translation
}

// Le champ_de_vision est un angle en radian, le ratio est la largeur/hauteur de l'écran, et [proche, loin] est l'intervalle visible
// Contraintes: 0 < champ_de_vision < pi, 0 < ratio et 0 < proche < loin
fn matrice_perspective(champ_de_vision: f32, ratio: f32, proche: f32, loin: f32) -> glm::Mat4 {

    let dilatation = proche * (champ_de_vision / 2.0).tan();

    glm::Mat4::new(
        2.0 * proche / (ratio * dilatation), 0.0, 0.0, 0.0,
        0.0, 2.0 * proche / dilatation, 0.0, 0.0,
        0.0, 0.0, (proche + loin) / (loin - proche), -2.0 * proche * loin / (loin - proche),
        0.0, 0.0, 1.0, 0.0
    )
}

fn matrice_opengl(matrice: glm::Mat4) -> [[f32; 4]; 4] {

    [
        [matrice.m11, matrice.m21, matrice.m31, matrice.m41],
        [matrice.m12, matrice.m22, matrice.m32, matrice.m42],
        [matrice.m13, matrice.m23, matrice.m33, matrice.m43],
        [matrice.m14, matrice.m24, matrice.m34, matrice.m44]
    ]
}
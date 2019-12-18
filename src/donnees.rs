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

    pub fn ajouter_triangle(&mut self, coordonnees: [f32; 9]) {

        self.sommets.reserve(3);

        self.sommets.push(Sommet{
            position: [coordonnees[0], coordonnees[1], coordonnees[2]],
        });

        self.sommets.push(Sommet{
            position: [coordonnees[3], coordonnees[4], coordonnees[5]],
        });

        self.sommets.push(Sommet{
            position: [coordonnees[6], coordonnees[7], coordonnees[8]],
        });

        for _ in 0..3 {
            self.indices.push(self.indices.len() as u32);
        }
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
            glium::index::PrimitiveType::TrianglesList,
            &self.indices[..]
        ).unwrap()
        // TrianglesList <=> Un triangle par triplet d'indices
    }
}

// Primitive pour OpenGL
#[derive(Copy, Clone)]
pub struct Sommet {
    position: [f32; 3],
}
// Permet à Glium de l'utiliser avec OpenGL
implement_vertex!(Sommet, position);

// Matrice importante qui sera appliquée sur tous les sommets
pub fn matrice_camera_perspective(position: &glm::Vec3, direction: &glm::Vec3) -> [[f32; 4]; 4] {

    /*let resultat = matrice_camera(  glm::Vec3::new(0.0, 0.0, 0.0),
                                    glm::Vec3::new(0.0, 0.0, 1.0))
                    * matrice_perspective(16.0 / 9.0, 0.01, 20.0);*/
    let resultat =  matrice_perspective(std::f32::consts::PI * 0.5, 16.0 / 9.0, 0.01, 100.0) *
                    matrice_camera(*position, *direction);

    /*println!("{},{},{},{}", resultat.m11, resultat.m12, resultat.m13, resultat.m14);
    println!("{},{},{},{}", resultat.m21, resultat.m22, resultat.m23, resultat.m24);
    println!("{},{},{},{}", resultat.m31, resultat.m32, resultat.m33, resultat.m34);
    println!("{},{},{},{}", resultat.m41, resultat.m42, resultat.m43, resultat.m44);*/

    /*let test = resultat * glm::Vec4::new(0.5, -0.3, 0.9, 1.0);
    println!("{},{},{},{}", test.x, test.y, test.z, test.w);*/

    /*let a1 = resultat * glm::Vec4::new(0.5, 0.5, 0.4, 1.0);
    let a2 = resultat * glm::Vec4::new(0.5, 0.5, -0.4, 1.0);
    println!("{},{},{},{}", a1.x, a1.y, a1.z, a1.w);
    println!("{},{},{},{}", a2.x, a2.y, a2.z, a2.w);*/

    matrice_opengl(resultat)
}





/*
    Partie privée du module donnees
*/

/*impl<'a> DonneesOpenGL<'a> {

    fn transferer_donnees(&mut self, vecteur: std::vec::Vec<Sommet>)
    {
        
        for sommet in vecteur.iter() {
            self.sommets.push(*sommet);
        }
    }
}*/

fn matrice_camera(position: glm::Vec3, direction: glm::Vec3) -> glm::Mat4 {

    /*let direction = glm::normalize(&direction);
    let haut = glm::Vec3::new(0.0, 1.0, 0.0);// Le haut est l'axe des y positif avec OpenGL

    let cote = direction.cross(&haut);
    let haut = cote.cross(&direction);

    
    glm::Mat4::new(
        cote.x, haut.x, direction.x, -position.x,
        cote.y, haut.y, direction.y, -position.y,
        cote.z, haut.z, direction.z, -position.z,
        0.0, 0.0, 0.0, 1.0
    )*/

    
    /*let direction = glm::normalize(&(-1.0*direction));
    let haut = glm::Vec3::new(0.0, 1.0, 0.0);// Le haut est l'axe des y positif avec OpenGL

    let droite = haut.cross(&direction);
    let haut = direction.cross(&droite);

    
    glm::Mat4::new(
        droite.x, droite.y, droite.z, -position.x,
        haut.x, haut.y, haut.z, -position.y,
        direction.x, direction.y, direction.z, -position.z,
        0.0, 0.0, 0.0, 1.0
    )*/

    /*glm::look_at(&position, 
                &(position + direction),
                &glm::Vec3::new(0.0, 1.0, 0.0))*/

    let direction = glm::normalize(&direction);
    let haut = glm::Vec3::new(0.0, 1.0, 0.0);// Le haut est l'axe des y positif avec OpenGL

    let droite = haut.cross(&direction);
    let haut = direction.cross(&droite);

    
    glm::Mat4::new(
        droite.x, droite.y, droite.z, -position.x,
        haut.x, haut.y, haut.z, -position.y,
        direction.x, direction.y, direction.z, -position.z,
        0.0, 0.0, 0.0, 1.0
    )

    /*glm::look_at(&glm::Vec3::new(0.0, 0.0, 0.0), 
                &direction,
                &glm::Vec3::new(0.0, 1.0, 0.0))*/
}

// Le champ_de_vision est un angle en radian, le ratio est la largeur/hauteur de l'écran, et [proche, loin] est l'intervalle visible
// Contraintes: 0 < champ_de_vision < pi, 0 < ratio et 0 < proche < loin
fn matrice_perspective(champ_de_vision: f32, ratio: f32, proche: f32, loin: f32) -> glm::Mat4 {

    /*glm::Mat4::new(
        1.0/ratio, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, -(proche + loin) / (loin - proche), -2.0 * proche * loin / (loin - proche),
        0.0, 0.0, -1.0, 0.0
    )*/

    /*glm::Mat4::new(
        ratio, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, (proche + loin) / (loin - proche), 2.0 * proche * loin / (loin - proche),
        0.0, 0.0, 1.0, 0.0
    )*/

    /*glm::Mat4::new(
        proche * ratio, 0.0, 0.0, 0.0,
        0.0, proche, 0.0, 0.0,
        0.0, 0.0, (proche + loin) / (loin - proche), 2.0 * proche * loin / (loin - proche),
        0.0, 0.0, 1.0, 0.0
    )*/

    //glm::perspective(1.57, ratio, proche, loin)

    let dilatation = proche * (champ_de_vision / 2.0).tan();

    glm::Mat4::new(
        2.0 * proche / (ratio * dilatation), 0.0, 0.0, 0.0,
        0.0, 2.0 * proche / dilatation, 0.0, 0.0,
        0.0, 0.0, (proche + loin) / (loin - proche), -2.0 * proche * loin / (loin - proche),
        0.0, 0.0, 1.0, 0.0
    )
    
    /*glm::Mat4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )*/
}

fn matrice_opengl(matrice: glm::Mat4) -> [[f32; 4]; 4] {

    [
        [matrice.m11, matrice.m21, matrice.m31, matrice.m41],
        [matrice.m12, matrice.m22, matrice.m32, matrice.m42],
        [matrice.m13, matrice.m23, matrice.m33, matrice.m43],
        [matrice.m14, matrice.m24, matrice.m34, matrice.m44]
    ]
}

/*
#[derive(uniforms)]
struct Test {
    a:[[f32; 4]; 4],
}*/

/*
// Permet de générer les données globales des shaders pour OpenGL.
fn generer_uniforms() -> dyn glium::uniforms::Uniforms {


    uniform! { model: Test{a:[
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]}}
}*/
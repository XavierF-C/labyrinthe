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
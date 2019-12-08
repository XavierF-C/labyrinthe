/**
    Interface publique du module shader
*/


// Représente un programme de shaders OpenGL
pub struct ProgrammeOpenGL {

    vertex_shader: std::string::String,
    fragment_shader: std::string::String,

    pub programme: glium::Program,
}

impl ProgrammeOpenGL {

    pub fn new(affichage: &glium::Display) -> ProgrammeOpenGL {

        let vertex_shader = obtenir_vertex_shader();
        let fragment_shader = obtenir_fragment_shader();
        let programme = glium::Program::from_source(
            affichage,
            &vertex_shader,
            &fragment_shader,
            None);

        // On vérifie si le programme est correct, sinon on arrête le programme avec l'erreur
        let programme = match programme {

            Ok(o) => o,
            Err(e) => match e {
                glium::program::ProgramCreationError::CompilationError(e) => {

                    println!("\n\nIl y a au moins une erreur de compilation des shaders:\n{}", e);
                    panic!("La compilation des shaders a échouée");
                },

                glium::program::ProgramCreationError::LinkingError(e) => {

                    println!("\n\nIl y a au moins une erreur de linking des shaders:\n{}", e);
                    panic!("La compilation des shaders a échouée");
                },

                _ => { 
                    println!("\n\nUne erreur inconnue est survenue à la compilation des shaders.");
                    println!("Voir glium::program::ProgramCreationError\n");
                    panic!("La compilation des shaders a échouée")
                },
            }
        };

        ProgrammeOpenGL {
            vertex_shader: vertex_shader,
            fragment_shader: fragment_shader,
            programme: programme 
        }
    }
}





/**
    Partie privée du module shader 
*/


// Primitive pour OpenGL
#[derive(Copy, Clone)]
struct Sommet {
    position: [f32; 3],
}
// Permet à Glium de l'utiliser avec OpenGL
implement_vertex!(Sommet, position);


// Déclaration de tous les shaders utilisés
// La notation r#""# permet de préserver la chaîne brute

fn obtenir_vertex_shader() -> std::string::String
{
    std::string::String::from(r#"
        #version 330

        in vec3 position;

        void main() {
            gl_Position = vec4(position, 1.0);
        }
    "#)
}

fn obtenir_fragment_shader() -> std::string::String
{
    std::string::String::from(r#"
        #version 330
        
        out vec4 couleur;
        
        void main() {
            couleur = vec4(0.5, 0.5, 0.5, 1.0);
        }
    "#)
}
/*
    Interface publique du module shaders

    Sert à compiler les shaders.
*/

// Représente un programme de shaders OpenGL
pub struct ProgrammeOpenGL {

    pub programme: glium::Program,
}

impl ProgrammeOpenGL {

    pub fn new(affichage: &glium::Display) -> ProgrammeOpenGL {

        let vertex_shader = code_source::vertex_shader();
        let fragment_shader = code_source::fragment_shader();
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
            
            programme: programme 
        }
    }
}





/*
    Partie privée du module shaders
*/

mod code_source
{
    // Déclaration de tous les shaders utilisés
    // La notation r#""# permet de préserver la chaîne brute

    pub fn vertex_shader() -> std::string::String
    {
        std::string::String::from(r#"
            #version 330
            uniform layout(std140);

            uniform mat4 cameraPerspective;
            
            in vec3 position;

            void main() {
                gl_Position = cameraPerspective * vec4(position, 1.0);
            }
        "#)
    }
    // Déclaration du bloque uniform, avec les données globales partagées entre les shaders

    pub fn fragment_shader() -> std::string::String
    {
        std::string::String::from(r#"
            #version 330

            out vec4 couleur;
            
            void main() {
                couleur = vec4(0.5, 0.5, 0.5, 1.0);
            }
        "#)
    }
}
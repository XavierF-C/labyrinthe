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
            Err(e) => {
                
                match e {
                    
                    glium::program::ProgramCreationError::CompilationError(e) => {
                        println!("\n\nIl y a au moins une erreur de compilation des shaders:\n{}", e);
                    },

                    glium::program::ProgramCreationError::LinkingError(e) => {
                        println!("\n\nIl y a au moins une erreur de linking des shaders:\n{}", e);
                    },

                    _ => { 
                        println!("\n\nUne erreur inconnue est survenue à la compilation des shaders.");
                        println!("Voir glium::program::ProgramCreationError\n");
                    },
                }

                panic!("La compilation des shaders a échouée");
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

            uniform mat4 camera_perspective;
            uniform vec3 position_lumiere;
            //uniform vec3 position_lumieres[nbrLumieres];
            //uniform vec3 couleur_lumieres[nbrLumieres];
            uniform vec3 direction_regard;
            
            in vec3 position;
            in vec3 normale;
            in vec3 coordonnees_texture;
            
            out vec3 normal;
            out vec3 coord_tex;
            
            out vec3 directionLumiere;
            out vec3 directionRegard;
            
            out float distance;

            void main() {
                gl_Position = camera_perspective * vec4(position, 1.0);
                
                normal = normale;

                coord_tex = coordonnees_texture;

                directionLumiere = normalize(position - position_lumiere);
                directionRegard = direction_regard;

                distance = distance(position_lumiere, position);
            }
        "#)
    }
    
    pub fn fragment_shader() -> std::string::String
    {
        std::string::String::from(r#"
            #version 330
            uniform layout(std140);

            uniform sampler2DArray textures;

            in vec3 normal;
            in vec3 coord_tex;

            in vec3 directionLumiere;
            in vec3 directionRegard;

            in float distance;

            out vec4 couleur;
            
            void main() {

                vec3 direction_reflexion = reflect(-directionLumiere, normal);
                const float intensite_speculaire = 0.45;
                float lumiere_speculaire = intensite_speculaire * pow(max(dot(directionRegard, direction_reflexion), 0.0), 12);

                const float intensite_diffuse = 0.45;
                float lumiere_diffuse = intensite_diffuse * max(dot(normal, directionLumiere), 0.0);

                const float intensite_ambiante = 1.0 - (intensite_diffuse + intensite_speculaire);
                
                const float FACTEUR_DIMINUTION = 5.0;
                float diminution = (FACTEUR_DIMINUTION * (distance + distance * distance) + 1.0);
                vec4 luminosite = (lumiere_speculaire + lumiere_diffuse + intensite_ambiante) * vec4(1.0, 0.8, 0.5, 1.0) / diminution;
               
                couleur = texture(textures, coord_tex) * luminosite;// / (2.0 * distance * distance + 1.0);
            }
        "#)
    }
}
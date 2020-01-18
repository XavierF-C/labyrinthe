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
            #version 430
            uniform layout(std140);

            uniform mat4 camera_perspective;
            uniform vec3 direction_regard;

            const uint NBR_LUMIERES = 8;

            layout(std140) buffer lumieres {
                vec4 positions[NBR_LUMIERES];
                vec4 couleurs[NBR_LUMIERES];
            };
            
            in vec3 position;
            in vec3 normale;
            in vec3 coordonnees_texture;

            out vec3 normal;
            out vec3 coord_tex;
            
            out vec3 directionRegard;
            
            out Lumieres {
                vec4 couleurs[NBR_LUMIERES];
                vec3 directions[NBR_LUMIERES];
                float distances[NBR_LUMIERES];
            } outLumieres;

            void main() {
                gl_Position = camera_perspective * vec4(position, 1.0);
                
                normal = normale;

                coord_tex = coordonnees_texture;

                directionRegard = direction_regard;

                for(int i=0; i<NBR_LUMIERES; ++i) {
                    outLumieres.distances[i] = distance(vec3(positions[i]), position);
                    outLumieres.directions[i] = normalize(position - vec3(positions[i]));
                    outLumieres.couleurs[i] = couleurs[i];
                }
            }
        "#)
    }
    
    pub fn fragment_shader() -> std::string::String
    {
        std::string::String::from(r#"
            #version 430
            uniform layout(std140);

            uniform sampler2DArray textures;

            in vec3 normal;
            in vec3 coord_tex;

            in vec3 directionRegard;

            const uint NBR_LUMIERES = 8;

            in Lumieres {
                vec4 couleurs[NBR_LUMIERES];
                vec3 directions[NBR_LUMIERES];
                float distances[NBR_LUMIERES];
            } lumieres;

            out vec4 couleur;
            
            void main() {

                vec4 luminosite = vec4(0.0, 0.0, 0.0, 1.0);

                const float INTENSITE_SPECULAIRE = 0.45;
                const float INTENSITE_DIFFUSE = 0.45;
                const float INTENSITE_AMBIANTE = 1.0 - (INTENSITE_DIFFUSE + INTENSITE_SPECULAIRE);

                for(int i=0; i<NBR_LUMIERES; ++i) {

                    vec3 direction_reflexion = reflect(-lumieres.directions[i], normal);

                    float lumiere_speculaire = INTENSITE_SPECULAIRE * pow(max(dot(directionRegard, direction_reflexion), 0.0), 12);

                    float lumiere_diffuse = INTENSITE_DIFFUSE * max(dot(normal, lumieres.directions[i]), 0.0);
                    
                    const float FACTEUR_DIMINUTION = 10.0;
                    float diminution = FACTEUR_DIMINUTION * (lumieres.distances[i] + lumieres.distances[i] * lumieres.distances[i]) + 1.0;
                    vec4 luminosite2 = (lumiere_speculaire + lumiere_diffuse + INTENSITE_AMBIANTE) * lumieres.couleurs[i] / diminution;
                    
                    luminosite.x = max(luminosite.x, luminosite2.x);
                    luminosite.y = max(luminosite.y, luminosite2.y);
                    luminosite.z = max(luminosite.z, luminosite2.z);
                }

                couleur = texture(textures, coord_tex) * luminosite;
            }
        "#)
    }
}